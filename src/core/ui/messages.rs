use colored::Colorize;

pub struct Messages;

impl Messages {
    // Startup messages
    pub fn starting_adventure() -> String {
        "🌳 Starting the tree adventure..."
            .bright_cyan()
            .bold()
            .to_string()
    }

    pub fn scanning_files() -> String {
        "🔍 Scanning files".bright_yellow().to_string()
    }

    // Progress messages
    pub fn traversing_tree() -> String {
        "Traversing directory tree".to_string()
    }

    pub fn gathering_leaves() -> String {
        "🎉 Successfully gathered all the leaves!"
            .bright_green()
            .bold()
            .to_string()
    }

    // Action messages
    pub fn copying_clipboard() -> String {
        "Copying to clipboard".to_string()
    }

    pub fn clipboard_ready() -> String {
        format!(
            "{} {}",
            "📋".green(),
            "Clipboard updated! Ready to paste anywhere~".bright_green()
        )
    }

    pub fn clipboard_skipped() -> String {
        format!(
            "{} {}",
            "😴".yellow(),
            "Clipboard nap time - skipping copy".yellow().dimmed()
        )
    }

    pub fn opening_editor() -> String {
        "✏️  Opening your treasure chest..."
            .bright_cyan()
            .bold()
            .to_string()
    }

    pub fn editor_opened() -> String {
        "👀 Hope you like what you see!".bright_cyan().to_string()
    }

    pub fn cleaning_up() -> String {
        "🗑️  Cleaning up after the party..."
            .bright_yellow()
            .bold()
            .to_string()
    }

    pub fn cleaned_up() -> String {
        "✨ All cleaned up! No traces left behind~"
            .bright_green()
            .to_string()
    }

    pub fn showing_stats() -> String {
        "📊 Let's see what we've collected!"
            .bright_magenta()
            .bold()
            .to_string()
    }

    pub fn ready_to_launch() -> String {
        format!(
            "\n{}\n{}",
            "🚀 Ready to launch!".bright_green().bold(),
            "─".repeat(55).bright_green()
        )
    }

    // Ignore file messages
    pub fn found_ignore_file(path: &str) -> String {
        format!(
            "  {} {:<width$} {}",
            "🔍".cyan(),
            "Found ignore file:".bold(),
            path.bright_cyan(),
            width = 20
        )
    }

    pub fn applying_ignore_rules() -> String {
        "  📝 Applying rules from .treeclipignore"
            .dimmed()
            .to_string()
    }
}
