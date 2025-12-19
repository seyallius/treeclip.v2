use crate::core::utils;
use colored::{ColoredString, Colorize};
use std::path::PathBuf;
use crate::core::ui::table::FormattedBox;

const LABEL_WIDTH: usize = 18;
const BOX_WIDTH: usize = 55;

pub struct ConfigFormatter;

impl ConfigFormatter {
    pub fn format_section_header(title: &str, icon: &str) -> String {
        format!(
            "\n{} {}\n{}",
            icon,
            title.bright_blue().bold(),
            "─".repeat(BOX_WIDTH).bright_blue()
        )
    }

    pub fn format_config_line(icon: &str, label: &str, value: ColoredString) -> String {
        format!(
            "  {} {:<width$} {}",
            icon,
            label.bright_white(),
            value,
            width = LABEL_WIDTH
        )
    }

    pub fn format_path(path: &PathBuf) -> ColoredString {
        match path.canonicalize() {
            Ok(p) => p.display().to_string().cyan().bold(),
            Err(_) => path.display().to_string().yellow(),
        }
    }

    pub fn format_bool(val: bool) -> ColoredString {
        if val {
            "✓ Yes".green().bold()
        } else {
            "✗ No".red().dimmed()
        }
    }

    pub fn format_list_item(icon: &str, text: &str) -> String {
        format!("  {} {}", icon.dimmed(), text.dimmed())
    }
}

pub struct StatsBox {
    lines: usize,
    chars: usize,
    words: usize,
    bytes: usize,
}

impl StatsBox {
    pub fn new(lines: usize, chars: usize, words: usize, bytes: usize) -> Self {
        Self {
            lines,
            chars,
            words,
            bytes,
        }
    }

    pub fn render(&self) -> String {
        // format!(
        //     "┌──────────────────────────────────────────────┐\n\
        //      │           📊 Content Statistics               │\n\
        //      ├──────────────────────────────────────────────┤\n\
        //      │  📝 Characters:    {:>25}  │\n\
        //      │  📄 Lines:         {:>25}  │\n\
        //      │  💬 Words:         {:>25}  │\n\
        //      │  💾 Size:          {:>25}  │\n\
        //      └──────────────────────────────────────────────┘",
        //     utils::format_number(self.chars as i64).bright_white(),
        //     utils::format_number(self.lines as i64).bright_white(),
        //     utils::format_number(self.words as i64).bright_white(),
        //     utils::format_bytes(self.bytes).bright_white()
        // );
        FormattedBox::new("Content Statistics")
            .row("📝 Characters:", utils::format_number(self.chars as i64).bright_white().to_string())
            .row("📄 Lines:", utils::format_number(self.lines as i64).bright_white().to_string())
            .row("💬 Words:", utils::format_number(self.words as i64).bright_white().to_string())
            .row("💾 Size:", utils::format_bytes(self.bytes).bright_white().to_string())
            .render()
    }

    pub fn get_size_message(&self) -> (String, String) {
        match self.bytes {
            0..=1023 => (
                "🐣".to_string(),
                "Tiny but mighty!".bright_yellow().to_string(),
            ),
            1024..=102399 => (
                "🐇".to_string(),
                "Perfect size! Easy to handle~".bright_green().to_string(),
            ),
            102400..=1048575 => (
                "🐘".to_string(),
                "That's a big one! Impressive~".bright_cyan().to_string(),
            ),
            _ => (
                "🐋".to_string(),
                "Whoa! You've got a whale of content!"
                    .bright_blue()
                    .to_string(),
            ),
        }
    }
}
