use crate::core::ui::table::{Align, BorderStyle, FormattedBox};
use colored::Colorize;
use rand::Rng;
use std::sync::LazyLock;
use std::vec;

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

pub fn print_welcome() {
    let mut rng = rand::rng();
    let banner = &BANNERS[rng.random_range(0..BANNERS.len())];
    println!("{}", banner.bright_magenta());
}

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

pub fn print_goodbye() {
    println!("\n{}", "━".repeat(55).bright_cyan());

    let mut rng = rand::rng();
    let message = GOODBYE_MESSAGES[rng.random_range(0..GOODBYE_MESSAGES.len())];

    println!("    {}", message.bright_green().bold());
    println!(
        "    {} {}",
        get_random_kaomoji(),
        "Have a wonderful day!".bright_yellow()
    );
    println!("{}\n", "━".repeat(55).bright_cyan());
}

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

pub fn get_random_kaomoji() -> &'static str {
    let mut rng = rand::rng();
    KAOMOJIS[rng.random_range(0..KAOMOJIS.len())]
}

#[cfg(test)]
mod banner_test {
    use crate::core::utils;
    use unicode_width::UnicodeWidthStr;

    #[test]
    fn test() {
        // expected output:
        // ┌─────────────────────────────────────────────────┐
        // │                Content Statistics               │
        // ├─────────────────────────────────────────────────┤
        // │  📝 Characters:                               1 │
        // │  📄 Lines:                                  100 │
        // │  💬 Words:                                1,000 │
        // │  💾 Size:                              976.6 KB │
        // └─────────────────────────────────────────────────┘
        assert_eq!(
            render_stats(1, 100, 1000, 1_000_000),
            r#"┌─────────────────────────────────────────────────┐
│                Content Statistics               │
├─────────────────────────────────────────────────┤
│  📝 Characters:                               1 │
│  📄 Lines:                                  100 │
│  💬 Words:                                1,000 │
│  💾 Size:                              976.6 KB │
└─────────────────────────────────────────────────┘"#
        );
    }

    fn render_stats(chars: i64, lines: i64, words: i64, bytes: u64) -> String {
        let label_width = 18;
        let value_width = 25;

        let rows = vec![
            ("📝 Characters:", utils::format_number(chars)),
            ("📄 Lines:", utils::format_number(lines)),
            ("💬 Words:", utils::format_number(words)),
            ("💾 Size:", utils::format_bytes(bytes as usize)),
        ];

        let mut out = String::new();
        out.push_str("┌─────────────────────────────────────────────────┐\n");
        out.push_str("│                Content Statistics               │\n");
        out.push_str("├─────────────────────────────────────────────────┤\n");

        for (label, value) in rows {
            out.push_str(&format!(
                "│  {}  {}  │\n",
                pad(label, label_width),
                pad_right_align(&value, value_width + 1)
            ));
        }

        out.push_str("└─────────────────────────────────────────────────┘");
        out
    }

    fn pad(s: &str, width: usize) -> String {
        let w = UnicodeWidthStr::width(s);
        format!("{}{}", s, " ".repeat(width.saturating_sub(w)))
    }

    fn pad_right_align(s: &str, width: usize) -> String {
        let w = UnicodeWidthStr::width(s);
        format!("{}{}", " ".repeat(width.saturating_sub(w)), s)
    }
}
