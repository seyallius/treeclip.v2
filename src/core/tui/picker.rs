//! Module picker - Stateful file picker widget and application loop.
//! Implements a dual-pane layout with directory navigation and multi-selection support.

use crate::core::ui::messages::Messages;
use anyhow::{Context, Result};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::widgets::Wrap;
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use std::{
    collections::HashSet,
    io,
    path::{Path, PathBuf},
    time::Duration,
};

/// Result returned by the TUI upon successful confirmation.
pub struct TuiResult {
    pub input_paths: Vec<PathBuf>,
    pub exclude_patterns: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
enum InputMode {
    Normal,
    GlobInclude,
    GlobExclude,
}

/// Represents a node in the file tree (file or directory)
#[derive(Debug, Clone)]
struct FileNode {
    path: PathBuf,
    name: String,
    is_dir: bool,
    depth: usize,
    children: Vec<FileNode>,
}

/// Main application state holding UI and data models
struct App {
    root: PathBuf,
    nodes: Vec<FileNode>,
    flat_nodes: Vec<PathBuf>,
    expanded_dirs: HashSet<PathBuf>,
    selected_items: HashSet<PathBuf>,
    excluded_items: HashSet<PathBuf>,
    list_state: ListState,
    status_message: String,
    input_mode: InputMode,
    input_buffer: String,
    glob_include_patterns: Vec<String>,
    glob_exclude_patterns: Vec<String>,
}
impl App {
    fn new(root: &Path) -> Result<Self> {
        let nodes = build_tree(root, 0)?;
        let mut app = Self {
            root: root.to_path_buf(),
            nodes,
            flat_nodes: Vec::new(),
            expanded_dirs: HashSet::new(),
            selected_items: HashSet::new(),
            excluded_items: HashSet::new(),
            list_state: ListState::default(),
            status_message: Messages::starting_adventure(),
            input_mode: InputMode::Normal,
            input_buffer: String::new(),
            glob_include_patterns: Vec::new(),
            glob_exclude_patterns: Vec::new(),
        };

        app.expanded_dirs.insert(root.to_path_buf());
        app.rebuild_flat_view();
        if !app.flat_nodes.is_empty() {
            app.list_state.select(Some(0));
        }

        Ok(app)
    }

    fn run<B>(&mut self, terminal: &mut Terminal<B>) -> Result<bool>
    where
        B: ratatui::backend::Backend,
        <B as ratatui::backend::Backend>::Error: Send + Sync + 'static,
    {
        loop {
            terminal.draw(|f| self.ui(f))?;

            if event::poll(Duration::from_millis(50))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        if self.input_mode != InputMode::Normal {
                            match key.code {
                                KeyCode::Enter => self.apply_glob_pattern(),
                                KeyCode::Esc => {
                                    self.input_buffer.clear();
                                    self.input_mode = InputMode::Normal;
                                }
                                KeyCode::Char(c) => self.input_buffer.push(c),
                                KeyCode::Backspace => {
                                    self.input_buffer.pop();
                                }
                                _ => {}
                            }
                        } else {
                            match key.code {
                                KeyCode::Char('q') => return Ok(false),
                                KeyCode::Char('c') => return Ok(true),
                                KeyCode::Char(' ') => self.toggle_selection(),
                                KeyCode::Char('e') => self.toggle_exclusion(),
                                KeyCode::Char('g') => self.input_mode = InputMode::GlobInclude,
                                KeyCode::Char('x') => self.input_mode = InputMode::GlobExclude,
                                KeyCode::Enter => self.toggle_expand(),
                                KeyCode::Up | KeyCode::Char('k') => self.move_up(),
                                KeyCode::Down | KeyCode::Char('j') => self.move_down(),
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }

    fn rebuild_flat_view(&mut self) {
        self.flat_nodes.clear();
        flatten_nodes(&self.nodes, &self.expanded_dirs, &mut self.flat_nodes);

        if let Some(selected) = self.list_state.selected() {
            if selected >= self.flat_nodes.len() && !self.flat_nodes.is_empty() {
                self.list_state.select(Some(self.flat_nodes.len() - 1));
            }
        }
    }

    fn toggle_expand(&mut self) {
        if let Some(idx) = self.list_state.selected() {
            if idx < self.flat_nodes.len() {
                let path = &self.flat_nodes[idx];
                if path.is_dir() {
                    if self.expanded_dirs.contains(path) {
                        self.expanded_dirs.remove(path);
                    } else {
                        self.expanded_dirs.insert(path.clone());
                    }
                    self.rebuild_flat_view();
                }
            }
        }
    }

    fn toggle_selection(&mut self) {
        if let Some(idx) = self.list_state.selected() {
            if idx < self.flat_nodes.len() {
                let path = self.flat_nodes[idx].clone();
                if self.excluded_items.contains(&path) {
                    self.status_message = "⚠️ Cannot select an excluded item!".to_string();
                    return;
                }
                if self.selected_items.contains(&path) {
                    self.selected_items.remove(&path);
                } else {
                    self.selected_items.insert(path);
                }
            }
        }
    }

    fn toggle_exclusion(&mut self) {
        if let Some(idx) = self.list_state.selected() {
            if idx < self.flat_nodes.len() {
                let path = self.flat_nodes[idx].clone();
                if self.excluded_items.contains(&path) {
                    self.excluded_items.remove(&path);
                    self.status_message = format!("✅ Included: {}", path.display());
                } else {
                    self.excluded_items.insert(path.clone());
                    self.selected_items.remove(&path);
                    self.status_message = format!("❌ Excluded: {}", path.display());
                }
            }
        }
    }

    fn apply_glob_pattern(&mut self) {
        let pattern = self.input_buffer.clone();
        if pattern.is_empty() {
            self.input_mode = InputMode::Normal;
            return;
        }

        let is_exclude = matches!(self.input_mode, InputMode::GlobExclude);

        match crate::core::glob::expand_glob(&pattern) {
            Ok(paths) => {
                let mut count = 0;
                for path in paths {
                    let abs_path = std::fs::canonicalize(&path).unwrap_or(path);
                    if is_exclude {
                        if self.excluded_items.insert(abs_path.clone()) {
                            self.selected_items.remove(&abs_path);
                            count += 1;
                        }
                    } else {
                        if !self.excluded_items.contains(&abs_path)
                            && self.selected_items.insert(abs_path.clone())
                        {
                            count += 1;
                        }
                    }
                }
                self.status_message = format!(
                    "✨ {} {} items via glob: {}",
                    if is_exclude { "Excluded" } else { "Selected" },
                    count,
                    pattern
                );
                if is_exclude {
                    self.glob_exclude_patterns.push(pattern);
                } else {
                    self.glob_include_patterns.push(pattern);
                }
            }
            Err(e) => {
                self.status_message = format!("❌ Glob error: {}", e);
            }
        }
        self.input_buffer.clear();
        self.input_mode = InputMode::Normal;
    }

    fn move_up(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.flat_nodes.len().saturating_sub(1)
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    fn move_down(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.flat_nodes.len().saturating_sub(1) {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    fn get_preview(&self) -> String {
        if let Some(idx) = self.list_state.selected() {
            if let Some(path) = self.flat_nodes.get(idx) {
                if path.is_file() {
                    if let Ok(content) = std::fs::read_to_string(path) {
                        return content.lines().take(20).collect::<Vec<_>>().join("\n");
                    }
                } else if path.is_dir() {
                    return format!("📂 Directory: {}", path.display());
                }
            }
        }
        "No preview available".to_string()
    }

    /// Renders the complete UI frame
    fn ui(&mut self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(0),    // Main
                Constraint::Length(3), // Footer
            ])
            .split(f.area());

        // Header
        let header = Paragraph::new(Line::from(vec![
            Span::styled(
                " 🌳 TreeClip ",
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("| "),
            Span::styled(
                self.root.display().to_string(),
                Style::default().fg(Color::Cyan),
            ),
        ]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        );
        f.render_widget(header, chunks[0]);

        // Main split
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
            .split(chunks[1]);

        // File List
        let items: Vec<ListItem> = self
            .flat_nodes
            .iter()
            .map(|path| {
                let is_selected = self.selected_items.contains(path);
                let is_excluded = self.excluded_items.contains(path);
                let is_expanded = self.expanded_dirs.contains(path);

                let depth = path
                    .strip_prefix(&self.root)
                    .map(|p| p.components().count())
                    .unwrap_or(0);
                let indent = "  ".repeat(depth);

                let (icon, check) = if is_excluded {
                    ("🚫 ", "   ")
                } else if is_selected {
                    if path.is_dir() {
                        ("📂 ", "✅ ")
                    } else {
                        ("📄 ", "✅ ")
                    }
                } else {
                    if path.is_dir() {
                        if is_expanded {
                            ("📂 ", "   ")
                        } else {
                            ("📁 ", "   ")
                        }
                    } else {
                        ("📄 ", "   ")
                    }
                };

                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("???");

                let style = if is_excluded {
                    Style::default()
                        .fg(Color::DarkGray)
                        .add_modifier(Modifier::CROSSED_OUT)
                } else if is_selected {
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };

                ListItem::new(Line::from(vec![
                    Span::styled(format!("{}{}{}", indent, check, icon), style),
                    Span::styled(name.to_string(), style),
                ]))
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" 🗂️ Files ")
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("➤ ");

        f.render_stateful_widget(list, main_chunks[0], &mut self.list_state);

        // Right pane split
        let right_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(60), // Preview
                Constraint::Percentage(40), // Summary
            ])
            .split(main_chunks[1]);

        // Preview
        let preview_text = self.get_preview();
        let preview = Paragraph::new(preview_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" 👁️ Preview ")
                    .border_style(Style::default().fg(Color::Yellow)),
            )
            .wrap(Wrap { trim: true });
        f.render_widget(preview, right_chunks[0]);

        // Summary
        let selected_count = self.selected_items.len();
        let excluded_count = self.excluded_items.len();
        let total_size = self
            .selected_items
            .iter()
            .filter_map(|p| std::fs::metadata(p).ok())
            .map(|m| m.len())
            .sum::<u64>();

        let mut summary_lines = vec![
            Line::from(format!("✅ Selected: {} items", selected_count)),
            Line::from(format!("❌ Excluded: {} items", excluded_count)),
            Line::from(format!("💾 Size: {}", format_bytes(total_size as usize))),
        ];

        if !self.glob_include_patterns.is_empty() || !self.glob_exclude_patterns.is_empty() {
            summary_lines.push(Line::from(""));
            summary_lines.push(Line::from(Span::styled(
                "Active Globs:",
                Style::default().add_modifier(Modifier::BOLD),
            )));
            for g in &self.glob_include_patterns {
                summary_lines.push(Line::from(Span::styled(
                    format!("  + {}", g),
                    Style::default().fg(Color::Green),
                )));
            }
            for g in &self.glob_exclude_patterns {
                summary_lines.push(Line::from(Span::styled(
                    format!("  - {}", g),
                    Style::default().fg(Color::Red),
                )));
            }
        }

        let summary = Paragraph::new(summary_lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" 📊 Summary ")
                .border_style(Style::default().fg(Color::Green)),
        );
        f.render_widget(summary, right_chunks[1]);

        // Footer
        let footer = match self.input_mode {
            InputMode::Normal => Paragraph::new(Line::from(vec![
                Span::styled(
                    " [NORMAL] ",
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(
                    " Space: Select | e: Exclude | g: Glob + | x: Glob - | c: Confirm | q: Quit ",
                ),
            ])),
            InputMode::GlobInclude => Paragraph::new(Line::from(vec![
                Span::styled(
                    " [GLOB INCLUDE] ",
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(&self.input_buffer),
                Span::styled("█", Style::default().fg(Color::White)),
            ])),
            InputMode::GlobExclude => Paragraph::new(Line::from(vec![
                Span::styled(
                    " [GLOB EXCLUDE] ",
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Red)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(&self.input_buffer),
                Span::styled("█", Style::default().fg(Color::White)),
            ])),
        }
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        );
        f.render_widget(footer, chunks[2]);
    }
}

/// Entry point for the interactive TUI file picker.
///
/// Returns a vector of selected absolute paths upon successful confirmation.
/// Returns an empty vector if the user cancels.
///
/// # Errors
/// Returns error if terminal initialization fails or file system access is denied.
/// Entry point for the interactive TUI file picker.
pub fn run_tui(root: &Path) -> Result<Option<TuiResult>> {
    enable_raw_mode().context("Failed to enable raw mode")?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
        .context("Failed to enter alternate screen")?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).context("Failed to initialize terminal")?;

    let mut app = App::new(root)?;
    let res = app.run(&mut terminal);

    disable_raw_mode().context("Failed to disable raw mode")?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .context("Failed to leave alternate screen")?;
    terminal.show_cursor().context("Failed to show cursor")?;

    match res {
        Ok(true) => {
            let mut input_paths: Vec<PathBuf> = app.selected_items.iter().cloned().collect();
            input_paths.sort();

            let mut exclude_patterns: Vec<String> = app
                .excluded_items
                .iter()
                .map(|p| p.to_string_lossy().to_string())
                .collect();
            exclude_patterns.extend(app.glob_exclude_patterns);

            Ok(Some(TuiResult {
                input_paths,
                exclude_patterns,
            }))
        }
        Ok(false) => Ok(None),
        Err(e) => Err(e),
    }
}

/// Recursively builds file tree structure
fn build_tree(path: &Path, depth: usize) -> Result<Vec<FileNode>> {
    let mut nodes = Vec::new();

    if path.is_dir() {
        let entries = std::fs::read_dir(path)
            .with_context(|| format!("Failed to read directory: {}", path.display()))?;

        for entry in entries {
            let entry = entry?;
            let entry_path = entry.path();

            if entry.file_name().to_string_lossy().starts_with('.') {
                continue;
            }

            let is_dir = entry_path.is_dir();
            let children = if is_dir && depth < 3 {
                build_tree(&entry_path, depth + 1)?
            } else {
                Vec::new()
            };

            nodes.push(FileNode {
                name: entry.file_name().to_string_lossy().to_string(),
                path: entry_path,
                is_dir,
                depth,
                children,
            });
        }

        nodes.sort_by(|a, b| b.is_dir.cmp(&a.is_dir).then(a.name.cmp(&b.name)));
    }

    Ok(nodes)
}

/// Flattens tree respecting expansion state for linear navigation
fn flatten_nodes(nodes: &[FileNode], expanded: &HashSet<PathBuf>, out: &mut Vec<PathBuf>) {
    for node in nodes {
        out.push(node.path.clone());
        if node.is_dir && expanded.contains(&node.path) {
            flatten_nodes(&node.children, expanded, out);
        }
    }
}

/// Formats bytes into human-readable string (reused from utils)
fn format_bytes(bytes: usize) -> String {
    const UNITS: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
    if bytes == 0 {
        return "0 B".to_string();
    }
    let base: f64 = 1024.0;
    let bytes_f64 = bytes as f64;
    let exponent = (bytes_f64.ln() / base.ln()).floor() as usize;
    let exponent = exponent.min(UNITS.len() - 1);
    let value = bytes_f64 / base.powi(exponent as i32);
    if exponent == 0 {
        format!("{} {}", bytes, UNITS[exponent])
    } else {
        format!("{:.1} {}", value, UNITS[exponent])
    }
}
