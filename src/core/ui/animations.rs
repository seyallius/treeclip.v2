//! animations - Provides terminal animation utilities for visual feedback.

use colored::Colorize;
use std::io::{stdout, Write};
use std::{thread, time};

/// Spinner provides animated loading indicators with customizable frames and colors.
pub struct Spinner {
    frames: Vec<&'static str>,
    colors: Vec<colored::Color>,
}

impl Spinner {
    /// Creates a tree-themed spinner animation.
    pub fn new_tree() -> Self {
        Self {
            frames: vec!["🌱", "🌿", "🍃", "🌳", "🌲", "🎄"],
            colors: vec![
                colored::Color::Green,
                colored::Color::BrightGreen,
                colored::Color::Cyan,
                colored::Color::BrightCyan,
            ],
        }
    }

    /// Creates a loading-themed spinner animation.
    pub fn new_loading() -> Self {
        Self {
            frames: vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
            colors: vec![
                colored::Color::Cyan,
                colored::Color::BrightCyan,
                colored::Color::Blue,
                colored::Color::BrightBlue,
            ],
        }
    }

    /// Displays the spinner animation for the specified duration.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to display alongside the spinner
    /// * `duration_ms` - Total duration of the animation in milliseconds
    pub fn spin(&self, message: &str, duration_ms: u64) {
        let frame_duration = duration_ms / self.frames.len() as u64;

        for (i, frame) in self.frames.iter().enumerate() {
            let color = &self.colors[i % self.colors.len()];
            print!(
                "\r{} {} {}",
                frame.color(*color),
                message.bright_cyan(),
                "...".dimmed()
            );
            stdout().flush().unwrap();
            thread::sleep(time::Duration::from_millis(frame_duration));
        }

        println!(
            "\r{} {} {}",
            "✓".bright_green(),
            message.bright_green(),
            "Done!".dimmed()
        );
    }
}

/// Displays animated dots after text with specified count and delay.
///
/// # Arguments
///
/// * `text` - The text to display before the dots
/// * `count` - Number of dots to animate
/// * `delay_ms` - Delay between each dot in milliseconds
pub fn animated_dots(text: &str, count: usize, delay_ms: u64) {
    print!("{}", text.bright_yellow());
    for _ in 0..count {
        print!("{}", ".".bright_yellow());
        stdout().flush().unwrap();
        thread::sleep(time::Duration::from_millis(delay_ms));
    }
    println!();
}

/// Generates a progress counter message at specified intervals.
///
/// # Arguments
///
/// * `emoji_set` - Set of emojis to cycle through
/// * `current` - Current progress count
/// * `interval` - Interval at which to generate messages
///
/// # Returns
///
/// Returns `Some(message)` if counter should be displayed, `None` otherwise.
pub fn progress_counter(emoji_set: &[&str], current: usize, interval: usize) -> Option<String> {
    if current % interval == 0 {
        let idx = (current / interval) % emoji_set.len();
        Some(format!(
            "{} Collected {} files so far...",
            emoji_set[idx], current
        ))
    } else {
        None
    }
}

#[cfg(test)]
mod animations_tests {
    use super::*;

    #[test]
    fn test_spinner_creation_tree() {
        let spinner = Spinner::new_tree();
        assert_eq!(spinner.frames.len(), 6);
        assert_eq!(spinner.colors.len(), 4);
    }

    #[test]
    fn test_spinner_creation_loading() {
        let spinner = Spinner::new_loading();
        assert_eq!(spinner.frames.len(), 10);
        assert_eq!(spinner.colors.len(), 4);
    }

    #[test]
    fn test_progress_counter_at_interval() {
        let emojis = vec!["🌱", "🌿", "🍃"];
        let result = progress_counter(&emojis, 5, 5);
        assert!(result.is_some());
        assert!(result.unwrap().contains("5 files"));
    }

    #[test]
    fn test_progress_counter_not_at_interval() {
        let emojis = vec!["🌱", "🌿", "🍃"];
        let result = progress_counter(&emojis, 3, 5);
        assert!(result.is_none());
    }

    #[test]
    fn test_progress_counter_emoji_rotation() {
        let emojis = vec!["🌱", "🌿", "🍃"];

        // First interval
        let result1 = progress_counter(&emojis, 5, 5);
        assert!(result1.is_some());
        assert!(result1.unwrap().contains("🌱"));

        // Second interval
        let result2 = progress_counter(&emojis, 10, 5);
        assert!(result2.is_some());
        assert!(result2.unwrap().contains("🌿"));

        // Third interval
        let result3 = progress_counter(&emojis, 15, 5);
        assert!(result3.is_some());
        assert!(result3.unwrap().contains("🍃"));
    }

    #[test]
    fn test_progress_counter_zero() {
        let emojis = vec!["🌱", "🌿"];
        let result = progress_counter(&emojis, 0, 5);
        assert!(result.is_some());
    }
}
