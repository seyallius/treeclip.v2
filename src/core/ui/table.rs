//! table - A Unicode-aware box formatter for creating aligned, emoji-safe tables.
//!
//! This utility renders clean, aligned tables using box-drawing characters.
//! It automatically measures visible width using `unicode-width` to ensure
//! labels and values align correctly, regardless of emoji or multi-byte characters.
//!
//! # Example
//!
//! ```
//! use treeclip::core::ui::table::FormattedBox;
//!
//! let box_output = FormattedBox::new("Content Statistics")
//!     .row("📝 Characters:", "1,234")
//!     .row("📄 Lines:", "456")
//!     .row("💬 Words:", "7,890")
//!     .row("💾 Size:", "12.3 MB")
//!     .render();
//!
//! println!("{}", box_output);
//! ```
//!
//! This prints:
//!
//! ```text
//! ┌──────────────────────────────────────────────────┐
//! │                Content Statistics                │
//! ├──────────────────────────────────────────────────┤
//! │  📝 Characters:                            1,234 │
//! │  📄 Lines:                                   456 │
//! │  💬 Words:                                 7,890 │
//! │  💾 Size:                                12.3 MB │
//! └──────────────────────────────────────────────────┘
//! ```

use unicode_width::UnicodeWidthStr;

// -------------------------------------------- Public Structs and Enums --------------------------------------------

/// FormattedBox creates beautifully aligned boxes with statistics or messages.
pub struct FormattedBox {
    title: String,
    rows: Vec<RowKind>,
    theme: BoxTheme,
}

/// Represents different types of rows in the box.
enum RowKind {
    Stat { label: String, value: String },
    Message(String),
}

/// Border style options for the box.
#[derive(Clone, Copy)]
pub enum BorderStyle {
    /// Sharp corners: ┌ ┐ └ ┘ ─ │
    Sharp,
    /// Rounded corners: ╭ ╮ ╰ ╯ ─ │
    Rounded,
    /// Double lines: ╔ ╗ ╚ ╝ ═ ║
    Double,
}

/// Text alignment options.
#[derive(Clone, Copy)]
pub enum Align {
    #[allow(dead_code)]
    Left,
    Center,
}

/// Theme configuration for box appearance.
#[derive(Clone)]
pub struct BoxTheme {
    pub padding: usize,
    pub border: BorderStyle,
    pub align: Align,
}

impl Default for BoxTheme {
    fn default() -> Self {
        Self {
            padding: 2,
            border: BorderStyle::Sharp,
            align: Align::Center,
        }
    }
}

/// Border characters for different styles.
struct BorderChars {
    top_left: &'static str,
    top_right: &'static str,
    bottom_left: &'static str,
    bottom_right: &'static str,
    h: &'static str,
    v: &'static str,
}

impl FormattedBox {
    /// Creates a new FormattedBox with the specified title.
    pub fn new<T: Into<String>>(title: T) -> Self {
        Self {
            title: title.into(),
            rows: Vec::new(),
            theme: BoxTheme::default(),
        }
    }

    /// Adds a label/value row to the box (builder pattern).
    pub fn row<L: Into<String>, V: Into<String>>(mut self, label: L, value: V) -> Self {
        self.rows.push(RowKind::Stat {
            label: label.into(),
            value: value.into(),
        });
        self
    }

    /// Adds a message line to the box (builder pattern).
    pub fn message_line<S: Into<String>>(mut self, line: S) -> Self {
        self.rows.push(RowKind::Message(line.into()));
        self
    }

    /// Sets the theme (builder pattern).
    #[allow(dead_code)]
    pub fn theme(mut self, theme: BoxTheme) -> Self {
        self.theme = theme;
        self
    }

    /// Sets the border style (builder pattern).
    pub fn border_style(mut self, style: BorderStyle) -> Self {
        self.theme.border = style;
        self
    }

    /// Sets the padding (builder pattern).
    pub fn padding(mut self, pad: usize) -> Self {
        self.theme.padding = pad;
        self
    }

    /// Sets the text alignment (builder pattern).
    pub fn align(mut self, align: Align) -> Self {
        self.theme.align = align;
        self
    }

    /// Renders the box as a formatted string.
    pub fn render(&self) -> String {
        let is_stats = self.rows.iter().any(|r| matches!(r, RowKind::Stat { .. }));

        if is_stats {
            self.render_stats_box()
        } else {
            self.render_message_box()
        }
    }
}

// -------------------------------------------- Private Helper Functions --------------------------------------------

impl FormattedBox {
    /// Renders a statistics-style box with fixed width.
    fn render_stats_box(&self) -> String {
        let mut out = String::new();

        // Top border
        out.push_str("┌──────────────────────────────────────────────────┐\n");

        // Title (centered)
        let title_width = UnicodeWidthStr::width(self.title.as_str());
        let total_width = 51;
        let padding = (total_width - title_width) / 2;

        out.push_str(&format!(
            "│{}{}{}│\n",
            " ".repeat(padding),
            self.title,
            " ".repeat(total_width - padding - title_width - 1)
        ));

        // Separator
        out.push_str("├──────────────────────────────────────────────────┤\n");

        // Rows
        let label_width = 18;
        let value_width = 25;

        for row in &self.rows {
            if let RowKind::Stat { label, value } = row {
                out.push_str(&format!(
                    "│  {}  {}  │\n",
                    pad_left(label, label_width),
                    pad_right(value, value_width + 1)
                ));
            }
        }

        // Bottom border
        out.push_str("└──────────────────────────────────────────────────┘");
        out
    }

    /// Renders a message-style box with dynamic width.
    fn render_message_box(&self) -> String {
        let border = border_chars(self.theme.border);
        let pad = self.theme.padding;

        // Calculate maximum width needed
        let mut max_width = UnicodeWidthStr::width(self.title.as_str());
        for row in &self.rows {
            if let RowKind::Message(line) = row {
                max_width = max_width.max(UnicodeWidthStr::width(line.as_str()));
            }
        }

        let inner_width = max_width + pad * 2;

        let mut out = String::new();

        // Top border
        out.push_str(&format!(
            "{}{}{}\n",
            border.top_left,
            border.h.repeat(inner_width),
            border.top_right
        ));

        // Title
        out.push_str(&format!(
            "{}{}{}\n",
            border.v,
            align_text(
                &format!("{}{}", " ".repeat(pad), self.title),
                inner_width,
                self.theme.align
            ),
            border.v
        ));

        // Message lines
        for row in &self.rows {
            if let RowKind::Message(line) = row {
                let content = format!("{}{}", " ".repeat(pad), line);
                out.push_str(&format!(
                    "{}{}{}\n",
                    border.v,
                    align_text(&content, inner_width, self.theme.align),
                    border.v
                ));
            }
        }

        // Bottom border
        out.push_str(&format!(
            "{}{}{}",
            border.bottom_left,
            border.h.repeat(inner_width),
            border.bottom_right
        ));

        out
    }
}

/// Returns border characters for the specified style.
fn border_chars(style: BorderStyle) -> BorderChars {
    match style {
        BorderStyle::Sharp => BorderChars {
            top_left: "┌",
            top_right: "┐",
            bottom_left: "└",
            bottom_right: "┘",
            h: "─",
            v: "│",
        },
        BorderStyle::Rounded => BorderChars {
            top_left: "╭",
            top_right: "╮",
            bottom_left: "╰",
            bottom_right: "╯",
            h: "─",
            v: "│",
        },
        BorderStyle::Double => BorderChars {
            top_left: "╔",
            top_right: "╗",
            bottom_left: "╚",
            bottom_right: "╝",
            h: "═",
            v: "║",
        },
    }
}

/// Left-pads a string to the specified visible width.
fn pad_left(s: &str, width: usize) -> String {
    let w = UnicodeWidthStr::width(s);
    format!("{}{}", s, " ".repeat(width.saturating_sub(w)))
}

/// Right-pads a string to the specified visible width.
fn pad_right(s: &str, width: usize) -> String {
    let w = UnicodeWidthStr::width(s);
    format!("{}{}", " ".repeat(width.saturating_sub(w)), s)
}

/// Aligns text within the specified width according to alignment mode.
fn align_text(s: &str, width: usize, align: Align) -> String {
    let w = UnicodeWidthStr::width(s);

    match align {
        Align::Left => format!("{}{}", s, " ".repeat(width - w)),
        Align::Center => {
            let left = (width - w) / 2;
            let right = width - w - left;
            format!("{}{}{}", " ".repeat(left), s, " ".repeat(right))
        }
    }
}

#[cfg(test)]
mod table_tests {
    use super::*;

    #[test]
    fn test_formatted_box_creation() {
        let box_formatter = FormattedBox::new("Test Title");
        assert_eq!(box_formatter.title, "Test Title");
        assert!(box_formatter.rows.is_empty());
    }

    #[test]
    fn test_formatted_box_with_rows() {
        let box_formatter = FormattedBox::new("Statistics")
            .row("Label 1:", "Value 1")
            .row("Label 2:", "Value 2");

        assert_eq!(box_formatter.rows.len(), 2);
    }

    #[test]
    fn test_renders_properly_aligned_box() {
        let output = FormattedBox::new("Content Statistics")
            .row("📝 Characters:", "1")
            .row("📄 Lines:", "100")
            .row("💬 Words:", "1,000")
            .row("💾 Size:", "976.6 KB")
            .render();

        assert!(output.contains("Content Statistics"));
        assert!(output.contains("📝 Characters:"));
        assert!(output.contains("📄 Lines:"));
        assert!(output.contains("💬 Words:"));
        assert!(output.contains("💾 Size:"));
    }

    #[test]
    fn test_renders_message_box() {
        let banner = FormattedBox::new("✨  T R E E C L I P  ✨")
            .message_line("Your friendly code extraction companion!")
            .message_line("")
            .message_line("♡( ◡‿◡ )  Ready to explore your files~")
            .render();

        assert!(banner.contains("T R E E C L I P"));
        assert!(banner.contains("friendly code extraction companion"));
        assert!(banner.contains("Ready to explore"));
    }

    #[test]
    fn test_message_box_with_border_styles() {
        let banner_rounded = FormattedBox::new("Test")
            .border_style(BorderStyle::Rounded)
            .message_line("Test message")
            .render();

        assert!(banner_rounded.contains("╭"));
        assert!(banner_rounded.contains("╯"));

        let banner_double = FormattedBox::new("Test")
            .border_style(BorderStyle::Double)
            .message_line("Test message")
            .render();

        assert!(banner_double.contains("╔"));
        assert!(banner_double.contains("╝"));
    }

    #[test]
    fn test_padding_configuration() {
        let box_small = FormattedBox::new("Title")
            .padding(1)
            .message_line("Message")
            .render();

        let box_large = FormattedBox::new("Title")
            .padding(5)
            .message_line("Message")
            .render();

        // Larger padding should result in wider box
        assert!(box_large.lines().next().unwrap().len() > box_small.lines().next().unwrap().len());
    }

    #[test]
    fn test_align_configuration() {
        let box_center = FormattedBox::new("Title")
            .align(Align::Center)
            .message_line("Message")
            .render();

        let box_left = FormattedBox::new("Title")
            .align(Align::Left)
            .message_line("Message")
            .render();

        // Both should render successfully
        assert!(!box_center.is_empty());
        assert!(!box_left.is_empty());
    }

    #[test]
    fn test_unicode_width_handling() {
        // Test with emoji and unicode characters
        let output = FormattedBox::new("📊 Statistics 統計")
            .row("🔥 Hot:", "🌟")
            .row("😀 Happy:", "✨")
            .render();

        // Should not panic and should contain the unicode content
        assert!(output.contains("Statistics"));
        assert!(output.contains("🔥"));
        assert!(output.contains("😀"));
    }
}
