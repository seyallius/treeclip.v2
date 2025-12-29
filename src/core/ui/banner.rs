//! banner - Provides welcome and goodbye banner displays for the application.

use crate::core::ui::table::{Align, BorderStyle, FormattedBox};
use colored::Colorize;
use rand::Rng;
use std::sync::LazyLock;

// -------------------------------------------- Constants --------------------------------------------

/// Available banner designs for welcome screen.
pub static BANNERS: LazyLock<Vec<String>> = LazyLock::new(|| {
    vec![
        FormattedBox::new("🌳  T R E E C L I P  🌳")
            .border_style(BorderStyle::Double)
            .padding(3)
            .align(Align::Center)
            .message_line("Traverse & Extract with Style!")
            .message_line("")
            .message_line("(づ｡◕‿‿◕｡)づ  Let's gather some leaves!")
            .render(),
        FormattedBox::new("✨  T R E E C L I P  ✨")
            .border_style(BorderStyle::Rounded)
            .padding(3)
            .align(Align::Center)
            .message_line("Your friendly code extraction companion!")
            .message_line("")
            .message_line("♡( ◡‿◡ )  Ready to explore your files~")
            .render(),
        FormattedBox::new("🎄  T R E E C L I P  🎄")
            .border_style(BorderStyle::Sharp)
            .padding(3)
            .align(Align::Center)
            .message_line("Fast • Simple • Cute")
            .message_line("")
            .message_line("ヾ(⌐■_■)ノ♪  Time to clip that tree!")
            .render(),
    ]
});

/// Goodbye messages to display on exit.
const GOODBYE_MESSAGES: &[&str] = &[
    "✨ Mission accomplished! Time to shine!",
    "🎯 All done! Maybe grab a cookie? 🍪",
    "🌟 Great work! Your code is ready for takeoff!",
    "💫 TreeClip adventure complete! See you next time~",
    "🎉 Perfect! Everything extracted successfully!",
    "✅ Nailed it! Your files are all bundled up!",
    "🚀 Launch ready! Your code awaits!",
    "🎊 Fantastic! Another tree successfully clipped!",
];

/// Collection of kaomojis for various messages.
const KAOMOJIS: &[&str] = &[
    "ʕ•ᴥ•ʔ",
    "(◕‿◕✿)",
    "(ﾉ◕ヮ◕)ﾉ*:･ﾟ✧",
    "✧･ﾟ: *✧･ﾟ:*",
    "(づ｡◕‿‿◕｡)づ",
    "(っ◕‿◕)っ",
    "♡( ◡‿◡ )",
    "(●´ω｀●)",
    "٩(◕‿◕｡)۶",
    "ヽ(•‿•)ノ",
    "(ﾉ´ з `)ノ",
    "(´｡• ω •｡`)",
    "☆ﾟ･*:.｡.☆(￣ω￣)/",
    "(๑˃ᴗ˂)ﻭ",
    "╰( ´・ω・)つ──☆",
    "ヾ(⌐■_■)ノ♪",
    "ヾ(☆▽☆)",
    "(ﾉ>ω<)ﾉ",
    "(◠‿◠✿)",
    "(ﾉ^ヮ^)ﾉ*:・ﾟ✧",
];

/// Displays a randomly selected welcome banner.
pub fn print_welcome() {
    let mut rng = rand::rng();
    let banner = &BANNERS[rng.random_range(0..BANNERS.len())];
    println!("{}", banner.magenta());
}

/// Displays a goodbye message with a random kaomoji.
pub fn print_goodbye() {
    println!("\n{}", "━".repeat(55).cyan());

    let mut rng = rand::rng();
    let message = GOODBYE_MESSAGES[rng.random_range(0..GOODBYE_MESSAGES.len())];

    println!("    {}", message.green().bold());
    println!(
        "    {} {}",
        get_random_kaomoji(),
        "Have a wonderful day!".yellow()
    );
    println!("{}\n", "━".repeat(55).cyan());
}

/// Returns a random kaomoji from the collection.
pub fn get_random_kaomoji() -> &'static str {
    let mut rng = rand::rng();
    KAOMOJIS[rng.random_range(0..KAOMOJIS.len())]
}

#[cfg(test)]
mod banner_tests {
    use super::*;

    #[test]
    fn test_banners_not_empty() {
        assert!(!BANNERS.is_empty());
        assert_eq!(BANNERS.len(), 3);
    }

    #[test]
    fn test_each_banner_contains_treeclip() {
        for banner in BANNERS.iter() {
            assert!(banner.contains("T R E E C L I P"));
        }
    }

    #[test]
    fn test_goodbye_messages_not_empty() {
        assert!(!GOODBYE_MESSAGES.is_empty());
        assert_eq!(GOODBYE_MESSAGES.len(), 8);
    }

    #[test]
    fn test_kaomojis_not_empty() {
        assert!(!KAOMOJIS.is_empty());
        assert_eq!(KAOMOJIS.len(), 20);
    }

    #[test]
    fn test_get_random_kaomoji_returns_valid() {
        let kaomoji = get_random_kaomoji();
        assert!(KAOMOJIS.contains(&kaomoji));
    }

    #[test]
    fn test_get_random_kaomoji_multiple_calls() {
        // Test that function can be called multiple times
        for _ in 0..10 {
            let kaomoji = get_random_kaomoji();
            assert!(KAOMOJIS.contains(&kaomoji));
        }
    }
}
