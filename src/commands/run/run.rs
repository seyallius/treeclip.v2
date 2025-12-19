use super::args::RunArgs;
use crate::core::constants;
use crate::core::{clipboard::clipboard, editor::editor, traversal::walker, utils};
use colored::{Colorize, CustomColor};
use rand::Rng;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{env, fs, thread, time};

pub fn execute(args: RunArgs) -> anyhow::Result<()> {
    print_welcome_banner();

    let input = if &args.input_path == Path::new(".") {
        env::current_dir()?
    } else {
        args.input_path.clone()
    };

    let output = match &args.output_path {
        Some(path) if path == Path::new(".") => PathBuf::from("./treeclip_temp.txt"),
        Some(path) => path.clone(),
        None => PathBuf::from("./treeclip_temp.txt"),
    };

    let root = match &args.root {
        Some(path) if path == Path::new(".") => env::current_dir()?,
        Some(path) => path.to_path_buf(),
        None => env::current_dir()?,
    };

    log_info(&args, &root, &input, &output)?;

    println!(
        "\n{}",
        "🌳 Starting the tree adventure...".bright_cyan().bold()
    );

    // Animated loading
    print!("{}", "🔍 Scanning files".bright_yellow());
    for _ in 0..3 {
        print!(".");
        std::io::stdout().flush().unwrap();
        thread::sleep(time::Duration::from_millis(300));
    }
    println!();

    // Run core logic
    let walker = walker::Walker::new(&root, &input, &output, &args.exclude);

    // Simulate progress
    show_spinner("Traversing directory tree".to_string());
    walker.process_dir(&args)?;

    println!(
        "\n{}",
        "🎉 Successfully gathered all the leaves!"
            .bright_green()
            .bold()
    );

    let mut clip = clipboard::Clipboard::new(&output)?;

    if args.clipboard {
        show_spinner("Copying to clipboard".to_string());
        clip.set_clipboard()?;
        println!(
            "{} {}",
            "📋".green(),
            "Clipboard updated! Ready to paste anywhere~".bright_green()
        );
    } else {
        println!(
            "{} {:<width$}",
            "😴",
            "Clipboard nap time - skipping copy"
                .bold()
                .custom_color(CustomColor::from(constants::WARNING_COLOR)),
            width = constants::RIGHT_PADDING
        );
    }

    if args.stats {
        println!(
            "\n{}",
            "📊 Let's see what we've collected!".bright_magenta().bold()
        );
        show_stats(&output)?;
    }

    if args.editor {
        println!(
            "\n{}",
            "✏️  Opening your treasure chest...".bright_cyan().bold()
        );
        editor::open(&output)?;
        println!("{}", "👀 Hope you like what you see!".bright_cyan());
    }

    if args.delete && args.editor {
        println!(
            "\n{}",
            "🗑️  Cleaning up after the party...".bright_yellow().bold()
        );
        editor::delete(&output)?;
        println!(
            "{}",
            "✨ All cleaned up! No traces left behind~".bright_green()
        );
    }

    print_goodbye_message();
    Ok(())
}

fn print_welcome_banner() {
    let banner = r#"
    ╔══════════════════════════════════════════════╗
    ║   🌳  T R E E C L I P  🌳                    ║
    ║    Traverse & Extract with Cuteness!         ║
    ║                                              ║
    ║    (づ｡◕‿‿◕｡)づ Let's gather some leaves!   ║
    ╚══════════════════════════════════════════════╝
    "#;

    println!("{}", banner.bright_magenta());
}

fn print_goodbye_message() {
    println!("\n{}", "━".repeat(50).bright_cyan());

    let messages = vec![
        "✨ Mission accomplished! ✨",
        "🎯 All done! Time for a cookie break~ 🍪",
        "🌟 Great work! Your code is ready to shine!",
        "💫 TreeClip adventure complete! Until next time~",
    ];

    let mut rng = rand::rng();
    let message = messages[rng.random_range(0..messages.len())];

    println!("{}", message.bright_green().bold());
    println!(
        "{} {}",
        get_random_kaomoji(),
        "Have a wonderful day!".bright_yellow()
    );
    println!("{}", "━".repeat(50).bright_cyan());
}

fn get_random_kaomoji() -> String {
    let mut rng = rand::rng();
    constants::KAOMOJIS[rng.random_range(0..constants::KAOMOJIS.len())].to_string()
}

fn show_spinner(message: String) {
    let spinner_chars = vec!["🌱", "🌿", "🍃", "🍂", "🌳", "🌲"];
    for i in 0..6 {
        print!(
            "\r{}{} {}",
            spinner_chars[i % spinner_chars.len()],
            message.bright_cyan(),
            "...".bright_yellow()
        );
        std::io::stdout().flush().unwrap();
        thread::sleep(time::Duration::from_millis(200));
    }
    println!("\r{} {}", "✅".green(), "Done!".bright_green());
}

fn show_stats(output: &PathBuf) -> anyhow::Result<()> {
    let content = fs::read_to_string(output)?;
    let lines = content.split("\n").count();
    let chars = content.chars().count();
    let words = content.split_whitespace().count();
    let bytes = content.len();

    let stats_box = format!(
        "┌─────────────────────────────────────────┐\n\
         │          📊 Content Statistics          │\n\
         ├─────────────────────────────────────────┤\n\
         │  📝 Characters: {:>20}  │\n\
         │  📄 Lines:      {:>20}  │\n\
         │  💬 Words:      {:>20}  │\n\
         │  💾 Size:       {:>20}  │\n\
         └─────────────────────────────────────────┘",
        utils::format_number(chars as i64).bright_white(),
        utils::format_number(lines as i64).bright_white(),
        utils::format_number(words as i64).bright_white(),
        utils::format_bytes(bytes).bright_white()
    );

    println!("{}", stats_box.bright_cyan());

    // Fun messages based on content size
    if bytes < 1024 {
        println!("{} {}", "🐣".yellow(), "Tiny but mighty!".bright_yellow());
    } else if bytes < 1024 * 100 {
        println!(
            "{} {}",
            "🐇".green(),
            "Perfect size! Easy to handle~".bright_green()
        );
    } else if bytes < 1024 * 1024 {
        println!(
            "{} {}",
            "🐘".cyan(),
            "That's a big one! Impressive~".bright_cyan()
        );
    } else {
        println!(
            "{} {}",
            "🐋".bright_blue(),
            "Whoa! You've got a whale of content!".bright_blue()
        );
    }

    Ok(())
}

#[rustfmt::skip]
fn log_info(args: &RunArgs, root: &PathBuf, input: &PathBuf, output: &PathBuf) -> anyhow::Result<()>{
    fn colorize_bool(val: bool) -> String {
        if val {
            "✅ Yes".green().bold().to_string()
        } else {
            "❌ No".red().dimmed().to_string()
        }
    }

    fn format_path(path: &PathBuf) -> String {
        match path.canonicalize() {
            Ok(p) => p.display().to_string().cyan().bold().to_string(),
            Err(_) => path.display().to_string().yellow().to_string()
        }
    }

    println!("\n{}", "🔧 Configuration Settings".bright_blue().bold());
    println!("{}", "─".repeat(45).bright_blue());

    let config_items = vec![
        ("🌍 ", " Root Path", format_path(root)),
        ("📂 ", " Input Path", format_path(input)),
        ("💾 ", " Output Path", format_path(output)),
        ("✏️ ", " Editor", colorize_bool(args.editor)),
        ("🗑️ ", " Cleanup", colorize_bool(args.delete)),
        ("📋 ", " Clipboard", colorize_bool(args.clipboard)),
        ("📊 ", " Stats", colorize_bool(args.stats)),
        ("👻 ", " Skip Hidden", colorize_bool(args.skip_hidden)),
    ];

    for (icon, label, value) in config_items.iter() {
        println!("{} {:<18} {}", icon, label.bright_white(), value);
    }

    if !args.exclude.is_empty() {
        println!("\n{}", "🚫 Excluded Patterns".bright_red().bold());
        println!("{}", "─".repeat(45).bright_red());
        for pattern in &args.exclude {
            println!("   {} {}", "🔸".dimmed(), pattern.dimmed());
        }
    }

    println!("\n{}", "🚀 Ready to launch!".bright_green().bold());
    println!("{}", "─".repeat(45).bright_green());

    Ok(())
}
