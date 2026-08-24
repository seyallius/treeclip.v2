//! theme - Stateless color palette, borders, and reusable ratatui styles for the TreeClip TUI.
//!
//! Everything in this module is a pure constant or pure function. No state is held here.
//! Styles are derived once and shared across all render calls so the look stays consistent.
//!
//! Design language: a calm, "forest terminal" palette - dark green/sage background,
//! warm amber accents for selection, muted red for exclusion, cyan for the cursor.

use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

/// Background of the whole TUI.
pub const BG: Color = Color::Reset;
/// Background used for panels (slightly distinct from app bg).
pub const PANEL_BG: Color = Color::Black;
/// Default text color.
pub const FG: Color = Color::Gray;
/// Bright primary text (titles, file names that matter).
pub const FG_BRIGHT: Color = Color::White;
/// Muted secondary text (hints, secondary labels).
pub const FG_DIM: Color = Color::DarkGray;

/// Forest green - directories, success, brand.
pub const GREEN: Color = Color::LightGreen;
/// Sage - subtle directory hint.
pub const SAGE: Color = Color::Green;
/// Amber - selected items (warm highlight).
pub const AMBER: Color = Color::LightYellow;
/// Muted red - excluded items.
pub const RED: Color = Color::LightRed;
/// Cyan - cursor, focused borders, active elements.
pub const CYAN: Color = Color::LightCyan;
/// Blue - info badges.
pub const BLUE: Color = Color::LightBlue;
/// Magenta - glob matches.
pub const MAGENTA: Color = Color::LightMagenta;
/// Yellow - warnings, the output path field.
pub const YELLOW: Color = Color::Yellow;

/// Style for the application title bar.
pub fn title() -> Style {
    Style::default()
        .fg(FG_BRIGHT)
        .bg(GREEN)
        .add_modifier(Modifier::BOLD)
}

/// Style for normal body text.
pub fn body() -> Style {
    Style::default().fg(FG).bg(BG)
}

/// Style for the focused panel border.
pub fn focused_border() -> Style {
    Style::default().fg(CYAN).add_modifier(Modifier::BOLD)
}

/// Style for an unfocused panel border.
pub fn unfocused_border() -> Style {
    Style::default().fg(FG_DIM)
}

/// Style for a regular file row.
pub fn file_row() -> Style {
    Style::default().fg(FG).bg(BG)
}

/// Style for a directory row.
pub fn dir_row() -> Style {
    Style::default()
        .fg(SAGE)
        .bg(BG)
        .add_modifier(Modifier::BOLD)
}

/// Style for the cursor row (the row the user is currently on).
pub fn cursor_row() -> Style {
    Style::default().fg(FG_BRIGHT).bg(Color::DarkGray)
}

/// Style for a selected (to be bundled) file.
pub fn selected() -> Style {
    Style::default().fg(AMBER).add_modifier(Modifier::BOLD)
}

/// Style for an excluded file/folder.
pub fn excluded() -> Style {
    Style::default().fg(RED).add_modifier(Modifier::DIM)
}

/// Style for a glob-matched (but not-yet-confirmed) file.
pub fn glob_match() -> Style {
    Style::default().fg(MAGENTA).add_modifier(Modifier::BOLD)
}

/// Style for a status bar message.
pub fn status() -> Style {
    Style::default().fg(FG_BRIGHT).bg(Color::DarkGray)
}

/// Style for a keybind hint inside the help bar.
pub fn hint_key() -> Style {
    Style::default().fg(CYAN).add_modifier(Modifier::BOLD)
}

/// Style for a keybind description inside the help bar.
pub fn hint_text() -> Style {
    Style::default().fg(FG)
}

/// Style for popup background.
pub fn popup_bg() -> Style {
    Style::default().fg(FG_BRIGHT).bg(Color::Black)
}

/// Style for popup border.
pub fn popup_border() -> Style {
    Style::default().fg(YELLOW).add_modifier(Modifier::BOLD)
}

/// Style for an input text field.
pub fn input_field() -> Style {
    Style::default()
        .fg(FG_BRIGHT)
        .bg(Color::Black)
        .add_modifier(Modifier::BOLD)
}

/// Style for the input caret.
pub fn caret() -> Style {
    Style::default()
        .fg(CYAN)
        .add_modifier(Modifier::RAPID_BLINK | Modifier::BOLD)
}

/// Builds a one-line "key  description" hint suitable for a help/status bar.
pub fn hint(key: &str, desc: &str) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!(" {key} "), hint_key()),
        Span::styled(format!("{desc}  "), hint_text()),
    ])
}

/// Builds a colored single-character badge (e.g. "[x]" or "[ ]").
pub fn badge(on: bool) -> Span<'static> {
    if on {
        Span::styled(
            "[x]",
            Style::default().fg(GREEN).add_modifier(Modifier::BOLD),
        )
    } else {
        Span::styled("[ ]", Style::default().fg(FG_DIM))
    }
}
