//! ui - Stateless rendering for the TreeClip TUI.
//!
//! `render(app, frame)` is the only public entry point. It computes layout
//! rects, then delegates to per-panel pure functions. Nothing here mutates
//! `App` - the renderer is a pure `&App -> Frame` projection.
//!
//! Layout:
//!
//! ```text
//! ┌──────────────────────────────────────────────────────┐
//! │ Title bar (TreeClip TUI - root path)                 │
//! ├─────────────────────────────────┬────────────────────┤
//! │                                 │                    │
//! │  File tree                      │  Options panel     │
//! │  (multi-select + exclude)       │  (toggles)         │
//! │                                 │                    │
//! │                                 │                    │
//! ├─────────────────────────────────┴────────────────────┤
//! │ Status bar (last message)                            │
//! │ Help bar (keybind hints)                             │
//! └──────────────────────────────────────────────────────┘
//! ```

use crate::tui::{
    app::{App, Mode, OPTION_ROWS, Panel, Toggles},
    file_tree::FileNode,
    help, theme,
};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
};
use std::time::Duration;

const STATUS_TTL: Duration = Duration::from_secs(5);

/// Top-level render. Called by the TUI loop after every input event.
pub fn render(app: &App, frame: &mut Frame) {
    let size = frame.area();

    // Top-level vertical layout: title / main / status / footer.
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // title
            Constraint::Min(10),   // main area (tree + options)
            Constraint::Length(1), // status bar
            Constraint::Length(2), // footer / keybind hints
        ])
        .split(size);

    render_title_bar(app, frame, chunks[0]);

    // Main area is split horizontally into tree + options.
    let main = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
        .split(chunks[1]);

    render_tree_panel(app, frame, main[0]);
    render_options_panel(app, frame, main[1]);

    render_status_bar(app, frame, chunks[2]);
    render_footer_hints(frame, chunks[3]);

    // Modal overlays on top of everything else.
    match app.mode {
        Mode::Help => render_help_overlay(frame, size),
        Mode::InputIncludeGlob | Mode::InputExcludeGlob | Mode::InputOutputPath => {
            render_input_overlay(app, frame, size)
        }
        Mode::ConfirmRun => render_confirm_overlay(app, frame, size),
        Mode::Message => render_message_overlay(app, frame, size),
        Mode::Normal => {}
    }
}

fn render_title_bar(app: &App, frame: &mut Frame, area: Rect) {
    let root_display = app.tree.root.display().to_string();
    let (selected, excluded) = app.file_counts();
    let title = Line::from(vec![
        Span::styled(" 🌳 TreeClip TUI ", theme::title()),
        Span::raw("  "),
        Span::styled(
            root_display,
            Style::default()
                .fg(theme::CYAN)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("   "),
        Span::styled(
            format!("{selected} selected  ·  {excluded} excluded"),
            Style::default().fg(theme::AMBER),
        ),
    ]);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .border_style(theme::focused_border())
        .style(Style::default().bg(theme::PANEL_BG));

    let p = Paragraph::new(title)
        .block(block)
        .alignment(Alignment::Left);
    frame.render_widget(p, area);
}

fn render_tree_panel(app: &App, frame: &mut Frame, area: Rect) {
    let visible = app.visible();
    let focused = app.panel == Panel::Tree;

    // Compute visible-line offset that keeps the cursor in view, while
    // preserving the previous scroll position if the cursor is already in
    // view. The scroll is stored as a `Cell<usize>` inside `App` so the
    // otherwise-pure renderer can update it without taking `&mut App`.
    let height = area.height.saturating_sub(2) as usize; // -2 for borders
    let cursor_visible_pos = visible.iter().position(|&i| i == app.cursor).unwrap_or(0);
    let prev_scroll = app.scroll.get();
    let scroll = if cursor_visible_pos < prev_scroll {
        cursor_visible_pos
    } else if cursor_visible_pos >= prev_scroll.saturating_add(height).min(visible.len()) {
        cursor_visible_pos.saturating_sub(height.saturating_sub(1))
    } else {
        prev_scroll
    };
    app.scroll.set(scroll);

    // Build list items from visible rows.
    let items: Vec<ListItem> = visible
        .iter()
        .enumerate()
        .skip(scroll)
        .take(height)
        .map(|(visible_pos, &node_idx)| {
            let node = &app.tree.nodes[node_idx];
            ListItem::new(render_tree_row(node, visible_pos == cursor_visible_pos))
        })
        .collect();

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .border_style(if focused {
            theme::focused_border()
        } else {
            theme::unfocused_border()
        })
        .style(Style::default().bg(theme::PANEL_BG))
        .title(Line::from(vec![
            Span::styled(
                " 📂 File Tree ",
                Style::default()
                    .fg(theme::CYAN)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" "),
            Span::styled(
                "(Space=select  x=exclude  I/X=glob)",
                Style::default().fg(theme::FG_DIM),
            ),
        ]));

    let list = List::new(items)
        .block(block)
        .style(Style::default().bg(theme::PANEL_BG));

    let mut state = ListState::default();
    state.select(Some(cursor_visible_pos.saturating_sub(scroll)));
    frame.render_stateful_widget(list, area, &mut state);
}

/// Builds the rich row for one file-tree node.
fn render_tree_row(node: &FileNode, is_cursor: bool) -> Line<'static> {
    // Compute the base row style based on state.
    // Excluded wins, then selected (amber), then glob-match, then default.
    let state_style = if node.excluded {
        theme::excluded()
    } else if node.matched_glob {
        theme::glob_match()
    } else if node.selected {
        theme::selected()
    } else {
        theme::file_row()
    };

    // Icon
    let icon = if node.is_dir {
        if node.expanded { "▾" } else { "▸" }
    } else {
        " "
    };
    let kind = if node.is_dir { "📂" } else { "📄" };

    // Marker: ✓ for selected, ✗ for excluded, • for default, ◦ for glob-match
    let marker = if node.excluded {
        "✗"
    } else if node.matched_glob {
        "◆"
    } else if node.selected {
        "✓"
    } else {
        "•"
    };

    // Indent
    let indent: String = "  ".repeat(node.depth);
    let name = node
        .path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| node.path.display().to_string());

    // Apply cursor highlight (background) on top of state style.
    let row_style = if is_cursor {
        // Preserve the state color so the user still sees ✓/✗ semantics,
        // but draw the row reversed so it stands out as the cursor.
        let color = state_style.fg.unwrap_or(theme::FG_BRIGHT);
        Style::default()
            .fg(color)
            .add_modifier(Modifier::BOLD | Modifier::REVERSED)
    } else {
        state_style
    };

    Line::from(vec![
        Span::styled(format!("{indent}"), theme::body()),
        Span::styled(format!("{icon} {kind} "), row_style),
        Span::styled(format!("[{marker}] "), row_style),
        Span::styled(name, row_style),
    ])
}

fn render_options_panel(app: &App, frame: &mut Frame, area: Rect) {
    let focused = app.panel == Panel::Options;

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .border_style(if focused {
            theme::focused_border()
        } else {
            theme::unfocused_border()
        })
        .style(Style::default().bg(theme::PANEL_BG))
        .title(Line::from(vec![Span::styled(
            " ⚙ Options ",
            Style::default()
                .fg(theme::CYAN)
                .add_modifier(Modifier::BOLD),
        )]));

    let t = app.toggles;

    // Build rows: each row is either [x]/[ ] + label, plus the output path row.
    let mut rows: Vec<ListItem> = Vec::new();
    for (i, (label, _key)) in OPTION_ROWS.iter().enumerate() {
        let on = read_toggle(t, i);
        let badge = theme::badge(on);
        let label_style = if i == app.option_cursor && focused {
            Style::default()
                .fg(theme::FG_BRIGHT)
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD | Modifier::REVERSED)
        } else if on {
            Style::default().fg(theme::FG_BRIGHT)
        } else {
            Style::default().fg(theme::FG)
        };
        rows.push(ListItem::new(Line::from(vec![
            badge,
            Span::raw("  "),
            Span::styled(*label, label_style),
        ])));
    }

    // Output path row (sits at index `OPTION_ROWS.len()` so the cursor on it
    // matches the index checked in `event::handle_options_panel`).
    let output_label_style = if app.option_cursor == OPTION_ROWS.len() && focused {
        Style::default()
            .fg(theme::FG_BRIGHT)
            .bg(Color::DarkGray)
            .add_modifier(Modifier::BOLD | Modifier::REVERSED)
    } else {
        Style::default().fg(theme::YELLOW)
    };
    rows.push(ListItem::new(Line::from(vec![
        Span::styled("💾 ", Style::default().fg(theme::YELLOW)),
        Span::styled(
            "Output: ",
            Style::default()
                .fg(theme::FG_BRIGHT)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(app.output_path.clone(), output_label_style),
    ])));

    let list = List::new(rows)
        .block(block)
        .style(Style::default().bg(theme::PANEL_BG));
    let mut state = ListState::default();
    state.select(Some(app.option_cursor.min(OPTION_ROWS.len())));
    frame.render_stateful_widget(list, area, &mut state);

    // Hint footer for the options panel.
    let hint_area = Rect::new(
        area.x + 1,
        area.y + area.height.saturating_sub(2),
        area.width.saturating_sub(2),
        1,
    );
    let hint = Line::from(vec![
        Span::styled(
            " ↑↓ ",
            Style::default()
                .fg(theme::CYAN)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("navigate   ", Style::default().fg(theme::FG)),
        Span::styled(
            "Space ",
            Style::default()
                .fg(theme::CYAN)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("toggle   ", Style::default().fg(theme::FG)),
        Span::styled(
            "o ",
            Style::default()
                .fg(theme::CYAN)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("edit output", Style::default().fg(theme::FG)),
    ]);
    frame.render_widget(Paragraph::new(hint), hint_area);
}

fn read_toggle(t: Toggles, idx: usize) -> bool {
    match OPTION_ROWS.get(idx).map(|(_, k)| *k) {
        Some("clipboard") => t.clipboard,
        Some("stats") => t.stats,
        Some("editor") => t.editor,
        Some("delete") => t.delete,
        Some("tree") => t.tree,
        Some("skip_hidden") => t.skip_hidden,
        Some("fast_mode") => t.fast_mode,
        Some("verbose") => t.verbose,
        _ => false,
    }
}

fn render_status_bar(app: &App, frame: &mut Frame, area: Rect) {
    let text = match &app.status {
        Some((msg, ts)) if ts.elapsed() <= STATUS_TTL => msg.clone(),
        Some(_) => "  (idle)".to_string(),
        None => "  (idle)".to_string(),
    };
    let para = Paragraph::new(Line::from(vec![
        Span::styled(
            " ⚡ ",
            Style::default()
                .fg(theme::CYAN)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(text, Style::default().fg(theme::FG_BRIGHT)),
    ]))
    .style(Style::default().bg(Color::DarkGray));
    frame.render_widget(para, area);
}

fn render_footer_hints(frame: &mut Frame, area: Rect) {
    let hints = help::footer_hints();
    let mut spans: Vec<Span> = Vec::with_capacity(hints.len() * 2);
    for (key, desc) in hints {
        spans.push(Span::styled(format!(" {key} "), theme::hint_key()));
        spans.push(Span::styled(format!("{desc} "), theme::hint_text()));
    }
    let p = Paragraph::new(Line::from(spans))
        .style(Style::default().bg(theme::PANEL_BG))
        .wrap(Wrap { trim: false });
    frame.render_widget(p, area);
}

fn render_help_overlay(frame: &mut Frame, area: Rect) {
    // Centered popup ~60% width, ~80% height.
    let popup = centered_rect(area, 70, 80);

    let mut sections_spans: Vec<Line> = Vec::new();
    sections_spans.push(Line::from(vec![Span::styled(
        "  🌳  TreeClip TUI - Help  🌳  ",
        Style::default()
            .fg(theme::FG_BRIGHT)
            .bg(theme::GREEN)
            .add_modifier(Modifier::BOLD),
    )]));
    sections_spans.push(Line::from(""));

    for (section_name, lines) in help::sections() {
        sections_spans.push(Line::from(vec![Span::styled(
            format!("── {section_name} ──"),
            Style::default()
                .fg(theme::CYAN)
                .add_modifier(Modifier::BOLD),
        )]));
        for line in *lines {
            sections_spans.push(Line::from(vec![
                Span::styled(
                    format!("  {:<12}", line.key),
                    Style::default().fg(theme::AMBER),
                ),
                Span::raw("  "),
                Span::styled(line.desc.to_string(), Style::default().fg(theme::FG)),
            ]));
        }
        sections_spans.push(Line::from(""));
    }

    sections_spans.push(Line::from(vec![Span::styled(
        "  Press ? / Esc / q to close",
        Style::default()
            .fg(theme::FG_DIM)
            .add_modifier(Modifier::ITALIC),
    )]));

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Double)
        .border_style(theme::popup_border())
        .style(theme::popup_bg());

    // Clear the background behind the popup.
    frame.render_widget(Clear, popup);

    let p = Paragraph::new(sections_spans)
        .block(block)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false });
    frame.render_widget(p, popup);
}

fn render_input_overlay(app: &App, frame: &mut Frame, area: Rect) {
    let popup = centered_rect(area, 60, 12);

    let (title, hint) = match app.mode {
        Mode::InputIncludeGlob => (
            " Include via Glob Pattern ",
            "Enter a git-style glob (e.g. \"src/**/*.rs\" or \"object/*.go\") - matches will be marked selected.",
        ),
        Mode::InputExcludeGlob => (
            " Exclude via Glob Pattern ",
            "Enter a git-style glob (e.g. \"target/**\" or \"*.log\") - matches will be excluded.",
        ),
        Mode::InputOutputPath => (
            " Output File Path ",
            "Where to write the bundled output file.",
        ),
        _ => return,
    };

    // Build the input line: clone of buffer, split at the caret so we can
    // draw a blinking caret character in the middle.
    let value = app.input.clone();
    let (before, after) = value.split_at(app.caret);
    let input_line = Line::from(vec![
        Span::styled(" ", Style::default()),
        Span::styled(before.to_string(), theme::input_field()),
        Span::styled("▌", theme::caret()),
        Span::styled(after.to_string(), theme::input_field()),
        Span::raw(" "),
    ]);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .border_style(theme::popup_border())
        .style(theme::popup_bg())
        .title(Line::from(vec![Span::styled(
            title,
            Style::default()
                .fg(theme::YELLOW)
                .add_modifier(Modifier::BOLD),
        )]));

    frame.render_widget(Clear, popup);

    // Build the inner content: title already in block; we add the input line + hint.
    let inner_area = block.inner(popup);
    frame.render_widget(block.clone(), popup);

    let inner_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)])
        .split(inner_area);

    // Input line + visible framing box.
    let input_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::CYAN))
        .style(Style::default().bg(Color::Black));
    let input_para = Paragraph::new(input_line)
        .block(input_block)
        .style(theme::input_field());
    frame.render_widget(input_para, inner_chunks[0]);

    let hint_para = Paragraph::new(Line::from(vec![Span::styled(
        format!(" {hint} "),
        Style::default().fg(theme::FG_DIM),
    )]))
    .wrap(Wrap { trim: false });
    frame.render_widget(hint_para, inner_chunks[1]);
}

fn render_confirm_overlay(app: &App, frame: &mut Frame, area: Rect) {
    let popup = centered_rect(area, 50, 7);
    let paths = app.selected_input_paths();
    let (selected, excluded) = app.file_counts();
    // `selected` is informational only - it's the number of effectively
    // included files. `paths.len()` is the count we'll actually bundle
    // (which the walker will dedup & re-walk anyway).
    let _ = selected;
    let summary = Line::from(vec![
        Span::styled(
            "  🚀  Run TreeClip?  \n",
            Style::default()
                .fg(theme::GREEN)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!(
                "\n  Bundling {} file(s) ({} excluded)\n",
                paths.len(),
                excluded
            ),
            Style::default().fg(theme::FG_BRIGHT),
        ),
        Span::styled(
            format!("  Output → {}\n", app.output_path),
            Style::default().fg(theme::YELLOW),
        ),
        Span::styled(
            "\n  y = run   ·   n = back  ",
            Style::default().fg(theme::FG_DIM),
        ),
    ]);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Double)
        .border_style(
            Style::default()
                .fg(theme::GREEN)
                .add_modifier(Modifier::BOLD),
        );
    frame.render_widget(Clear, popup);
    frame.render_widget(Paragraph::new(Text::from(summary)).block(block), popup);
}

fn render_message_overlay(app: &App, frame: &mut Frame, area: Rect) {
    let popup = centered_rect(area, 50, 5);
    let msg = app
        .status
        .as_ref()
        .map(|(m, _)| m.clone())
        .unwrap_or_default();
    let spans = Line::from(vec![
        Span::styled(
            "  ⓘ  ",
            Style::default()
                .fg(theme::BLUE)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(msg, Style::default().fg(theme::FG_BRIGHT)),
    ]);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .border_style(Style::default().fg(theme::BLUE));
    frame.render_widget(Clear, popup);
    frame.render_widget(
        Paragraph::new(spans)
            .block(block)
            .wrap(Wrap { trim: false }),
        popup,
    );
}

/// Returns a `Rect` of size `width_pct% × height_pct%` of `area`, centered.
fn centered_rect(area: Rect, width_pct: u16, height_pct: u16) -> Rect {
    let pop_w = area.width.saturating_mul(width_pct) / 100;
    let pop_h = area.height.saturating_mul(height_pct) / 100;
    let x = area.x + (area.width.saturating_sub(pop_w)) / 2;
    let y = area.y + (area.height.saturating_sub(pop_h)) / 2;
    Rect::new(x, y, pop_w, pop_h)
}
