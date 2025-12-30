//! messages - Centralized user-facing message definitions for consistent UI.

use colored::Colorize;

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
        "🔍 Scanning files".blue().to_string()
    }

    // -------------------- Progress Messages --------------------

    /// Returns the traversing tree message.
    pub fn traversing_tree() -> String {
        "Traversing directory tree".to_string()
    }

    /// Returns the gathering leaves success message.
    pub fn gathering_leaves() -> String {
        "🎉 Successfully gathered all the leaves!"
            .green()
            .bold()
            .to_string()
    }

    // -------------------- Action Messages --------------------

    /// Returns the copying to clipboard message.
    pub fn copying_clipboard() -> String {
        "Copying to clipboard".to_string()
    }

    /// Returns the clipboard ready message.
    pub fn clipboard_ready() -> String {
        format!(
            "{} {}",
            "📋".green(),
            "Clipboard updated! Ready to paste anywhere~".green()
        )
    }

    /// Returns the clipboard skipped message.
    pub fn clipboard_skipped() -> String {
        format!(
            "{} {}",
            "😴".yellow(),
            "Clipboard nap time - skipping copy".yellow().dimmed()
        )
    }

    /// Returns the opening editor message.
    pub fn opening_editor() -> String {
        "✏️  Opening your treasure chest..."
            .blue()
            .bold()
            .to_string()
    }

    /// Returns the editor opened message.
    pub fn editor_opened() -> String {
        "👀 Hope you like what you see!".blue().to_string()
    }

    /// Returns the cleaning up message.
    pub fn cleaning_up() -> String {
        "🗑️  Cleaning up after the party..."
            .blue()
            .bold()
            .to_string()
    }

    /// Returns the cleaned up message.
    pub fn cleaned_up() -> String {
        "✨ All cleaned up! No traces left behind~"
            .green()
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
            "🚀 Ready to launch!".green().bold(),
            "─".repeat(55).green()
        )
    }

    // -------------------- Ignore File Messages --------------------

    /// Returns a formatted message for finding an ignore file.
    pub fn found_ignore_file(path: &str) -> String {
        format!(
            "  {} {:<width$} {}",
            "🔍".cyan(),
            "Found ignore file:".bold(),
            path.blue(),
            width = 20
        )
    }

    /// Returns the applying ignore rules message.
    pub fn applying_ignore_rules() -> String {
        "  📝 Applying rules from .treeclipignore"
            .dimmed()
            .to_string()
    }

    pub fn tree_structure_enabled() -> String {
        "Tree structure enabled. See the end of output file for the directory structure."
            .bright_white()
            .bold()
            .to_string()
    }
}

#[cfg(test)]
mod messages_tests {
    use super::*;

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

        assert!(!Messages::applying_ignore_rules().is_empty());
    }

    #[test]
    fn test_found_ignore_file_formatting() {
        let path = "test/path/.treeclipignore";
        let message = Messages::found_ignore_file(path);
        assert!(message.contains("Found ignore file:"));
        assert!(message.contains(path));
    }
}
