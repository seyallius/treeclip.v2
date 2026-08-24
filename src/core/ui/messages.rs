//! messages - Centralized user-facing message definitions for consistent UI.

use colored::Colorize;
use std::path::Path;

/// Messages provides a centralized location for all user-facing messages.
pub struct Messages;
impl Messages {
    // -------------------- Startup Messages --------------------

    /// Returns the starting adventure message.
    pub fn starting_adventure() -> String {
        "🌳 Starting the tree adventure..."
            .bright_white()
            .bold()
            .to_string()
    }

    /// Returns the scanning files message.
    pub fn scanning_files() -> String {
        "🔍 Scanning files".bright_blue().to_string()
    }

    // -------------------- Progress Messages --------------------

    /// Returns the traversing tree message.
    pub fn traversing_tree() -> String {
        "Traversing directory tree".bright_blue().to_string()
    }

    /// Returns the gathering leaves success message.
    pub fn gathering_leaves() -> String {
        "🎉 Successfully gathered all the leaves!"
            .bright_green()
            .bold()
            .to_string()
    }

    // -------------------- Action Messages --------------------

    /// Returns the copying to clipboard message.
    pub fn copying_clipboard() -> String {
        "Copying to clipboard".bright_blue().to_string()
    }

    /// Returns the clipboard ready message.
    pub fn clipboard_ready() -> String {
        format!(
            "{} {}",
            "📋".bright_green(),
            "Clipboard updated! Ready to paste anywhere~".bright_green()
        )
    }

    /// Returns the clipboard skipped message.
    pub fn clipboard_skipped() -> String {
        format!(
            "{} {}",
            "😴".bright_yellow(),
            "Clipboard nap time - skipping copy"
                .bright_yellow()
                .dimmed()
        )
    }

    /// Returns the opening editor message.
    pub fn opening_editor() -> String {
        "✏️  Opening your treasure chest..."
            .bright_blue()
            .bold()
            .to_string()
    }

    /// Returns the editor opened message.
    pub fn editor_opened() -> String {
        "👀 Hope you like what you see!".bright_blue().to_string()
    }

    /// Returns the cleaning up message.
    pub fn cleaning_up() -> String {
        "🗑️  Cleaning up after the party..."
            .bright_blue()
            .bold()
            .to_string()
    }

    /// Returns the cleaned up message.
    pub fn cleaned_up() -> String {
        "✨ All cleaned up! No traces left behind~"
            .bright_green()
            .to_string()
    }

    /// Returns the showing stats message.
    pub fn showing_stats() -> String {
        "📊 Let's see what we've collected!"
            .bright_magenta()
            .bold()
            .to_string()
    }

    /// Returns the ready to launch message.
    pub fn ready_to_launch() -> String {
        format!(
            "\n{}\n{}",
            "🚀 Ready to launch!".bright_green().bold(),
            "─".repeat(55).bright_green()
        )
    }

    // -------------------- Ignore File Messages --------------------

    /// Returns a formatted message for finding an ignore file.
    pub fn found_ignore_file(path: &str) -> String {
        format!(
            "  {} {:<width$} {}",
            "🔍".bright_cyan(),
            "Found ignore file:".bold(),
            path.bright_blue(),
            width = 20
        )
    }

    pub fn tree_structure_enabled() -> String {
        "Tree structure enabled. See the end of output file for the directory structure."
            .bright_white()
            .bold()
            .to_string()
    }

    // -------------------- Init Command Messages --------------------

    /// Returns the init starting message.
    pub fn init_starting() -> String {
        "🌱 Initializing treeclip configuration..."
            .bright_green()
            .bold()
            .to_string()
    }

    /// Returns the init success message with the file path.
    pub fn init_success(path: &Path) -> String {
        format!(
            "{} {}",
            "✅".bright_green(),
            format!("Successfully created {}", path.display())
                .bright_green()
                .bold()
        )
    }

    /// Returns the init cancelled message.
    pub fn init_cancelled() -> String {
        format!(
            "{} {}",
            "🚫".bright_yellow(),
            "Initialization cancelled. No changes made.".bright_yellow()
        )
    }

    /// Returns the init overwriting message.
    pub fn init_overwriting() -> String {
        format!(
            "{} {}",
            "⚠️".bright_yellow(),
            "Overwriting existing .treeclipignore...".bright_yellow()
        )
    }

    /// Returns the file exists warning message.
    pub fn init_file_exists_warning() -> String {
        format!(
            "{}\n{}",
            "⚠️  .treeclipignore already exists!".bright_yellow().bold(),
            "This will overwrite your existing file.".bright_yellow()
        )
    }

    /// Returns the importing from message.
    pub fn init_importing_from(source: &str, count: usize) -> String {
        format!(
            "  {} Importing {} patterns from {}",
            "📥".bright_cyan(),
            count.to_string().bright_white().bold(),
            source.bright_blue()
        )
    }

    /// Returns the import warning message.
    pub fn init_import_warning(source: &str, error: &str) -> String {
        format!(
            "  {} Failed to import from {}: {}",
            "⚠️".bright_yellow(),
            source.bright_blue(),
            error.dimmed()
        )
    }

    /// Returns the TUI canceled message.
    pub fn tui_cancelled() -> String {
        format!(
            "{} {}",
            "🚫".bright_yellow(),
            "TUI cancelled. No files bundled.".bright_yellow()
        )
    }
}

#[cfg(test)]
mod messages_tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_startup_messages_not_empty() {
        assert!(!Messages::starting_adventure().is_empty());
        assert!(!Messages::scanning_files().is_empty());
    }

    #[test]
    fn test_progress_messages_not_empty() {
        assert!(!Messages::traversing_tree().is_empty());
        assert!(!Messages::gathering_leaves().is_empty());
    }

    #[test]
    fn test_action_messages_not_empty() {
        assert!(!Messages::copying_clipboard().is_empty());
        assert!(!Messages::clipboard_ready().is_empty());
        assert!(!Messages::clipboard_skipped().is_empty());
        assert!(!Messages::opening_editor().is_empty());
        assert!(!Messages::editor_opened().is_empty());
        assert!(!Messages::cleaning_up().is_empty());
        assert!(!Messages::cleaned_up().is_empty());
        assert!(!Messages::showing_stats().is_empty());
        assert!(!Messages::ready_to_launch().is_empty());
    }

    #[test]
    fn test_ignore_file_messages() {
        let path = "/home/user/.treeclipignore";
        let message = Messages::found_ignore_file(path);
        assert!(message.contains(path));
        assert!(!message.is_empty());
    }

    #[test]
    fn test_found_ignore_file_formatting() {
        let path = "test/path/.treeclipignore";
        let message = Messages::found_ignore_file(path);
        assert!(message.contains("Found ignore file:"));
        assert!(message.contains(path));
    }

    #[test]
    fn test_init_messages_not_empty() {
        assert!(!Messages::init_starting().is_empty());
        assert!(!Messages::init_cancelled().is_empty());
        assert!(!Messages::init_overwriting().is_empty());
        assert!(!Messages::init_file_exists_warning().is_empty());
    }

    #[test]
    fn test_init_success_contains_path() {
        let path = PathBuf::from("/test/path/.treeclipignore");
        let message = Messages::init_success(&path);
        assert!(message.contains(".treeclipignore"));
    }

    #[test]
    fn test_init_importing_from_format() {
        let message = Messages::init_importing_from(".gitignore", 5);
        assert!(message.contains(".gitignore"));
        assert!(message.contains("5"));
    }

    #[test]
    fn test_init_import_warning_format() {
        let message = Messages::init_import_warning(".dockerignore", "File not found");
        assert!(message.contains(".dockerignore"));
        assert!(message.contains("File not found"));
    }
}
