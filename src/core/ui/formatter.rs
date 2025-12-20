//! formatter - Provides formatting utilities for configuration display and statistics.

use crate::core::ui::table::FormattedBox;
use crate::core::utils;
use colored::{ColoredString, Colorize};
use std::path::Path;

const LABEL_WIDTH: usize = 18;
const BOX_WIDTH: usize = 55;

/// ConfigFormatter handles formatting of configuration settings display.
pub struct ConfigFormatter;

impl ConfigFormatter {
    /// Formats a section header with icon and title.
    pub fn format_section_header(title: &str, icon: &str) -> String {
        format!(
            "\n{} {}\n{}",
            icon,
            title.bright_blue().bold(),
            "─".repeat(BOX_WIDTH).bright_blue()
        )
    }

    /// Formats a configuration line with icon, label, and value.
    pub fn format_config_line(icon: &str, label: &str, value: ColoredString) -> String {
        format!(
            "  {} {:<width$} {}",
            icon,
            label.bright_white(),
            value,
            width = LABEL_WIDTH
        )
    }

    /// Formats a path with proper coloring based on validity.
    pub fn format_path(path: &Path) -> ColoredString {
        match path.canonicalize() {
            Ok(p) => p.display().to_string().cyan().bold(),
            Err(_) => path.display().to_string().yellow(),
        }
    }

    /// Formats a boolean value with appropriate symbols and colors.
    pub fn format_bool(val: bool) -> ColoredString {
        if val {
            "✓ Yes".green().bold()
        } else {
            "✗ No".red().dimmed()
        }
    }

    /// Formats a list item with icon and text.
    pub fn format_list_item(icon: &str, text: &str) -> String {
        format!("  {} {}", icon.dimmed(), text.dimmed())
    }
}

/// StatsBox displays content statistics in a formatted box.
pub struct StatsBox {
    lines: usize,
    chars: usize,
    words: usize,
    bytes: usize,
}

impl StatsBox {
    /// Creates a new StatsBox with the specified statistics.
    pub fn new(lines: usize, chars: usize, words: usize, bytes: usize) -> Self {
        Self {
            lines,
            chars,
            words,
            bytes,
        }
    }

    /// Renders the statistics box as a formatted string.
    pub fn render(&self) -> String {
        FormattedBox::new("Content Statistics")
            .row(
                "📝 Characters:",
                utils::format_number(self.chars as i64)
                    .bright_white()
                    .to_string(),
            )
            .row(
                "📄 Lines:",
                utils::format_number(self.lines as i64)
                    .bright_white()
                    .to_string(),
            )
            .row(
                "💬 Words:",
                utils::format_number(self.words as i64)
                    .bright_white()
                    .to_string(),
            )
            .row(
                "💾 Size:",
                utils::format_bytes(self.bytes).bright_white().to_string(),
            )
            .render()
    }

    /// Returns an emoji and message based on file size.
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

#[cfg(test)]
mod formatter_tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::TempDir;

    #[test]
    fn test_format_section_header() {
        let header = ConfigFormatter::format_section_header("Test Section", "🔧");
        assert!(header.contains("Test Section"));
        assert!(header.contains("🔧"));
    }

    #[test]
    fn test_format_config_line() {
        let line = ConfigFormatter::format_config_line("📂", "Input Path", "test".cyan());
        assert!(line.contains("📂"));
        assert!(line.contains("Input Path"));
    }

    #[test]
    fn test_format_path_valid() {
        let temp_dir = TempDir::new().unwrap();
        let formatted = ConfigFormatter::format_path(temp_dir.path());
        // Should not be empty
        assert!(!formatted.to_string().is_empty());
    }

    #[test]
    fn test_format_path_invalid() {
        let path = PathBuf::from("/nonexistent/path");
        let formatted = ConfigFormatter::format_path(&path);
        assert!(!formatted.to_string().is_empty());
    }

    #[test]
    fn test_format_bool_true() {
        let formatted = ConfigFormatter::format_bool(true);
        assert!(formatted.to_string().contains("Yes"));
    }

    #[test]
    fn test_format_bool_false() {
        let formatted = ConfigFormatter::format_bool(false);
        assert!(formatted.to_string().contains("No"));
    }

    #[test]
    fn test_format_list_item() {
        let item = ConfigFormatter::format_list_item("▸", "test pattern");
        assert!(item.contains("▸"));
        assert!(item.contains("test pattern"));
    }

    #[test]
    fn test_stats_box_creation() {
        let stats = StatsBox::new(100, 1000, 200, 5000);
        assert_eq!(stats.lines, 100);
        assert_eq!(stats.chars, 1000);
        assert_eq!(stats.words, 200);
        assert_eq!(stats.bytes, 5000);
    }

    #[test]
    fn test_stats_box_render() {
        let stats = StatsBox::new(1, 100, 1_000, 1_000_000);
        let rendered = stats.render();

        assert!(rendered.contains("Content Statistics"));
        assert!(rendered.contains("Characters:"));
        assert!(rendered.contains("Lines:"));
        assert!(rendered.contains("Words:"));
        assert!(rendered.contains("Size:"));
    }

    #[test]
    fn test_get_size_message_tiny() {
        let stats = StatsBox::new(1, 10, 2, 500);
        let (emoji, message) = stats.get_size_message();
        assert_eq!(emoji, "🐣");
        assert!(message.contains("Tiny but mighty!"));
    }

    #[test]
    fn test_get_size_message_small() {
        let stats = StatsBox::new(10, 100, 20, 50_000);
        let (emoji, message) = stats.get_size_message();
        assert_eq!(emoji, "🐇");
        assert!(message.contains("Perfect size!"));
    }

    #[test]
    fn test_get_size_message_medium() {
        let stats = StatsBox::new(100, 1000, 200, 500_000);
        let (emoji, message) = stats.get_size_message();
        assert_eq!(emoji, "🐘");
        assert!(message.contains("big one"));
    }

    #[test]
    fn test_get_size_message_large() {
        let stats = StatsBox::new(1000, 10000, 2000, 5_000_000);
        let (emoji, message) = stats.get_size_message();
        assert_eq!(emoji, "🐋");
        assert!(message.contains("whale"));
    }
}
