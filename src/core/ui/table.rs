//! A Unicode‑aware box formatter.
//!
//! This utility renders a clean, aligned, emoji‑safe table using box‑drawing
//! characters. It automatically measures visible width using `unicode-width`
//! so labels and values always align correctly, regardless of emoji or
//! multi‑byte characters.
//!
//! # Example
//!
//! ```
//! use your_crate::stats_box::StatsBox;
//!
//! let box_output = StatsBox::new("Content Statistics")
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
//! ┌───────────────────────────────────────────────────┐
//! │                Content Statistics                 │
//! ├───────────────────────────────────────────────────┤
//! │  📝 Characters:                             1,234 │
//! │  📄 Lines:                                    456 │
//! │  💬 Words:                                  7,890 │
//! │  💾 Size:                                 12.3 MB │
//! └───────────────────────────────────────────────────┘
//! ```

use unicode_width::UnicodeWidthStr;

pub struct FormattedBox {
    title: String,
    rows: Vec<RowKind>,
    theme: BoxTheme,
}

enum RowKind {
    Stat { label: String, value: String },
    Message(String),
}

#[derive(Clone, Copy)]
pub enum BorderStyle {
    Sharp,   // ┌ ┐ └ ┘ ─ │
    Rounded, // ╭ ╮ ╰ ╯ ─ │
    Double,  // ╔ ╗ ╚ ╝ ═ ║
}

#[derive(Clone, Copy)]
pub enum Align {
    Left,
    Center,
}

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

struct BorderChars {
    top_left: &'static str,
    top_right: &'static str,
    bottom_left: &'static str,
    bottom_right: &'static str,
    h: &'static str,
    v: &'static str,
}

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

// Associated functions.
impl FormattedBox {
    /// Create a new stats box with a given title.
    pub fn new<T>(title: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            title: title.into(),
            rows: Vec::new(),
            theme: BoxTheme::default(),
        }
    }

    /// Add a label/value row to the box.
    pub fn row<L: Into<String>, V: Into<String>>(mut self, label: L, value: V) -> Self {
        self.rows.push(RowKind::Stat {
            label: label.into(),
            value: value.into(),
        });
        self
    }

    /// Add message to the box.
    pub fn message_line<S: Into<String>>(mut self, line: S) -> Self {
        self.rows.push(RowKind::Message(line.into()));
        self
    }

    pub fn theme(mut self, theme: BoxTheme) -> Self {
        self.theme = theme;
        self
    }

    pub fn border_style(mut self, style: BorderStyle) -> Self {
        self.theme.border = style;
        self
    }

    pub fn padding(mut self, pad: usize) -> Self {
        self.theme.padding = pad;
        self
    }

    pub fn align(mut self, align: Align) -> Self {
        self.theme.align = align;
        self
    }
}

// Methods.
impl FormattedBox {
    /// Render the final formatted box as a string.
    pub fn render(&self) -> String {
        let is_stats = self.rows.iter().any(|r| matches!(r, RowKind::Stat { .. }));

        if is_stats {
            self.render_stats_box()
        } else {
            self.render_message_box()
        }
    }

    fn render_stats_box(&self) -> String {
        let mut out = String::new();

        out.push_str("┌──────────────────────────────────────────────────┐\n");

        let title_width = UnicodeWidthStr::width(self.title.as_str());
        let total_width = 51;
        let padding = (total_width - title_width) / 2;

        out.push_str(&format!(
            "│{}{}{}│\n",
            " ".repeat(padding),
            self.title,
            " ".repeat(total_width - padding - title_width - 1)
        ));

        out.push_str("├──────────────────────────────────────────────────┤\n");

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

        out.push_str("└──────────────────────────────────────────────────┘");
        out
    }

    fn render_message_box(&self) -> String {
        let border = border_chars(self.theme.border);
        let pad = self.theme.padding;

        // Compute max width
        let mut max_width = UnicodeWidthStr::width(self.title.as_str());
        for row in &self.rows {
            if let RowKind::Message(line) = row {
                max_width = max_width.max(UnicodeWidthStr::width(line.as_str()));
            }
        }

        let inner_width = max_width + pad * 2;

        let mut out = String::new();
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

        out.push_str(&format!(
            "{}{}{}",
            border.bottom_left,
            border.h.repeat(inner_width),
            border.bottom_right
        ));

        out
    }
}

/// Left‑pad a string to a visible width.
fn pad_left(s: &str, width: usize) -> String {
    let w = UnicodeWidthStr::width(s);
    format!("{}{}", s, " ".repeat(width.saturating_sub(w)))
}

/// Right‑pad a string to a visible width.
fn pad_right(s: &str, width: usize) -> String {
    let w = UnicodeWidthStr::width(s);
    format!("{}{}", " ".repeat(width.saturating_sub(w)), s)
}

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
    fn renders_properly_aligned_box() {
        let output = FormattedBox::new("Content Statistics")
            .row("📝 Characters:", "1")
            .row("📄 Lines:", "100")
            .row("💬 Words:", "1,000")
            .row("💾 Size:", "976.6 KB")
            .render();

        println!("{}", output);
        assert_eq!(
            output,
            "┌──────────────────────────────────────────────────┐
│                Content Statistics                │
├──────────────────────────────────────────────────┤
│  📝 Characters:                               1  │
│  📄 Lines:                                  100  │
│  💬 Words:                                1,000  │
│  💾 Size:                              976.6 KB  │
└──────────────────────────────────────────────────┘"
        );
    }

    #[test]
    fn renders_properly_aligned_box_2() {
        let output = FormattedBox::new("Statistics")
            .row("📝 Characters:", "100,000,000,000")
            .row("📄 Lines:", "100,000,000")
            .row("💬 Words:", "1,000,000")
            .row("💾 Size:", "1011.6 MB")
            .render();

        println!("{}", output);
        assert_eq!(
            output,
            "┌──────────────────────────────────────────────────┐
│                    Statistics                    │
├──────────────────────────────────────────────────┤
│  📝 Characters:                 100,000,000,000  │
│  📄 Lines:                          100,000,000  │
│  💬 Words:                            1,000,000  │
│  💾 Size:                             1011.6 MB  │
└──────────────────────────────────────────────────┘"
        );
    }

    #[test]
    fn test_message_line() {
        let banner = FormattedBox::new("✨  T R E E C L I P  ✨")
            .message_line("Your friendly code extraction companion!")
            .message_line("")
            .message_line("♡( ◡‿◡ )  Ready to explore your files~")
            .render();

        println!("{}", banner);
        assert_eq!(
            banner,
            "┌────────────────────────────────────────────┐
│           ✨  T R E E C L I P  ✨          │
│   Your friendly code extraction companion! │
│                                            │
│    ♡( ◡‿◡ )  Ready to explore your files~  │
└────────────────────────────────────────────┘"
        );
    }

    #[test]
    fn test_message_line_advanced() {
        let banner = FormattedBox::new("✨  T R E E C L I P  ✨")
            .border_style(BorderStyle::Rounded)
            .padding(3)
            .align(Align::Center)
            .message_line("Your friendly code extraction companion!")
            .message_line("")
            .message_line("♡( ◡‿◡ )  Ready to explore your files~")
            .render();

        println!("{}", banner);
        assert_eq!(
            banner,
            "╭──────────────────────────────────────────────╮
│             ✨  T R E E C L I P  ✨          │
│    Your friendly code extraction companion!  │
│                                              │
│     ♡( ◡‿◡ )  Ready to explore your files~   │
╰──────────────────────────────────────────────╯"
        );
    }
}
