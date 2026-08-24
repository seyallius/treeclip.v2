//! Module picker - Stateful file picker widget and application loop.
//! Implements a dual-pane layout with directory navigation and multi-selection support.

use crate::core::ui::messages::Messages;
use anyhow::{Context, Result};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use std::{
    collections::HashSet,
    io,
    path::{Path, PathBuf},
    time::Duration,
};

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
    flat_nodes: Vec<PathBuf>,         // Flattened view for navigation
    expanded_dirs: HashSet<PathBuf>,  // Tracks which directories are open
    selected_items: HashSet<PathBuf>, // Multi-selection state
    list_state: ListState,            // Ratatui list state for scrolling/cursor
    status_message: String,
}
impl App {
    /// Creates a new App instance by scanning the root directory
    fn new(root: &Path) -> Result<Self> {
        let nodes = build_tree(root, 0)?;
        let mut app = Self {
            root: root.to_path_buf(),
            nodes,
            flat_nodes: Vec::new(),
            expanded_dirs: HashSet::new(),
            selected_items: HashSet::new(),
            list_state: ListState::default(),
            status_message: Messages::starting_adventure(),
        };

        // Initialize with root expanded
        app.expanded_dirs.insert(root.to_path_buf());
        app.rebuild_flat_view();
        app.list_state.select(Some(0));

        Ok(app)
    }

    /// Main event loop.
    ///
    /// Note: We constrain B::Error to be Send + Sync so that it can be
    /// converted into anyhow::Error via the ? operator. CrosstermBackend
    /// satisfies these bounds, making this safe for our use case.
    fn run<B>(&mut self, terminal: &mut Terminal<B>) -> Result<()>
    where
        B: ratatui::backend::Backend,
        <B as ratatui::backend::Backend>::Error: Send + Sync + 'static,
    {
        loop {
            terminal.draw(|f| self.ui(f))?;

            if event::poll(Duration::from_millis(50))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                            KeyCode::Char(' ') => self.toggle_selection(),
                            KeyCode::Enter => self.toggle_expand(),
                            KeyCode::Up | KeyCode::Char('k') => self.move_up(),
                            KeyCode::Down | KeyCode::Char('j') => self.move_down(),
                            KeyCode::Char('a') => self.select_all_visible(),
                            KeyCode::Char('c') => {
                                self.status_message =
                                    "✨ Selection confirmed! Ready to bundle~".to_string();
                                return Ok(());
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    /// Rebuilds the flattened navigation list based on expanded directories
    fn rebuild_flat_view(&mut self) {
        self.flat_nodes.clear();
        flatten_nodes(&self.nodes, &self.expanded_dirs, &mut self.flat_nodes);

        // Ensure cursor stays in bounds
        if let Some(selected) = self.list_state.selected() {
            if selected >= self.flat_nodes.len() && !self.flat_nodes.is_empty() {
                self.list_state.select(Some(self.flat_nodes.len() - 1));
            }
        }
    }

    /// Toggles expansion state of currently focused directory
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

    /// Toggles selection of currently focused item
    fn toggle_selection(&mut self) {
        if let Some(idx) = self.list_state.selected() {
            if idx < self.flat_nodes.len() {
                let path = self.flat_nodes[idx].clone();
                if self.selected_items.contains(&path) {
                    self.selected_items.remove(&path);
                } else {
                    self.selected_items.insert(path);
                }
            }
        }
    }

    /// Selects all currently visible items
    fn select_all_visible(&mut self) {
        for path in &self.flat_nodes {
            self.selected_items.insert(path.clone());
        }
        self.status_message = "🎯 Selected all visible items!".to_string();
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

    /// Returns all selected paths sorted alphabetically
    fn get_selected_paths(&self) -> Vec<PathBuf> {
        let mut paths: Vec<PathBuf> = self.selected_items.iter().cloned().collect();
        paths.sort();
        paths
    }

    /// Renders the complete UI frame
    fn ui(&mut self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),    // Main content
                Constraint::Length(3), // Status bar
            ])
            .split(f.area());

        // Main content area split into file list and info pane
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(chunks[0]);

        // Render file list
        let items: Vec<ListItem> = self
            .flat_nodes
            .iter()
            .map(|path| {
                let is_selected = self.selected_items.contains(path);
                let is_expanded = self.expanded_dirs.contains(path);

                // Calculate indentation based on depth relative to root
                let depth = path
                    .strip_prefix(&self.root)
                    .map(|p| p.components().count())
                    .unwrap_or(0);
                let indent = "  ".repeat(depth);

                let icon = if path.is_dir() {
                    if is_expanded { "📂 " } else { "📁 " }
                } else {
                    "📄 "
                };

                let check = if is_selected { "✅ " } else { "   " };

                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("???");

                let style = if is_selected {
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
                    .title(" 🌳 TreeClip File Picker ")
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("➤ ");

        f.render_stateful_widget(list, main_chunks[0], &mut self.list_state);

        // Info pane showing selection summary
        let selected_count = self.selected_items.len();
        let total_size = self
            .selected_items
            .iter()
            .filter_map(|p| std::fs::metadata(p).ok())
            .map(|m| m.len())
            .sum::<u64>();

        let info_text = vec![
            Line::from(Span::styled(
                "Selection Summary",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(format!("📊 Items: {}", selected_count)),
            Line::from(format!("💾 Size:  {}", format_bytes(total_size as usize))),
            Line::from(""),
            Line::from(Span::styled(
                "Controls:",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Line::from("  ↑/↓ Navigate"),
            Line::from("  Space Toggle select"),
            Line::from("  Enter Expand/Collapse"),
            Line::from("  a     Select all visible"),
            Line::from("  c     Confirm selection"),
            Line::from("  q/Esc Quit"),
        ];

        let info = Paragraph::new(info_text).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" ℹ️ Info ")
                .border_style(Style::default().fg(Color::Yellow)),
        );
        f.render_widget(info, main_chunks[1]);

        // Status bar
        let status = Paragraph::new(Line::from(vec![
            Span::styled(" 💡 ", Style::default().fg(Color::Magenta)),
            Span::raw(&self.status_message),
        ]))
        .block(Block::default().borders(Borders::ALL));
        f.render_widget(status, chunks[1]);
    }
}

/// Entry point for the interactive TUI file picker.
///
/// Returns a vector of selected absolute paths upon successful confirmation.
/// Returns an empty vector if the user cancels.
///
/// # Errors
/// Returns error if terminal initialization fails or file system access is denied.
pub fn run_tui(root: &Path) -> Result<Vec<PathBuf>> {
    // Setup terminal
    enable_raw_mode().context("Failed to enable raw mode")?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
        .context("Failed to enter alternate screen")?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).context("Failed to initialize terminal")?;

    // Create app state
    let mut app = App::new(root)?;
    let res = app.run(&mut terminal);

    // Restore terminal
    disable_raw_mode().context("Failed to disable raw mode")?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .context("Failed to leave alternate screen")?;
    terminal.show_cursor().context("Failed to show cursor")?;

    // Handle result
    match res {
        Ok(_) => Ok(app.get_selected_paths()),
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

            // Skip hidden files by default (can be made configurable)
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

        // Sort: directories first, then alphabetical
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
