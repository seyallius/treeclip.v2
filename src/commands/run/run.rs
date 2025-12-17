use super::args::RunArgs;
use crate::core::{clipboard::clipboard, editor::editor, traversal::walker, utils};
use std::path::{Path, PathBuf};
use std::{env, fs, path};

pub fn execute(args: RunArgs) -> anyhow::Result<()> {
    if args.verbose {
        log_startup(&args);
    }

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

    // Run core logic
    let walker = walker::Walker::new(&root, &input, &output, &args.exclude);
    walker.process_dir(&args)?;

    let mut clip = clipboard::Clipboard::new(&output)?;
    if args.clipboard {
        clip.set_clipboard()?;
    }

    if args.stats {
        let content = fs::read_to_string(&output)?; // fixme: bad. very bad. cpu and ram intensive!
        let lines = content.split("\n").count();
        let chars = content.chars().count();
        let words = content.split_whitespace().count();
        let bytes = content.len();

        println!("📊  Clipboard content stats:");
        println!("   📝  Characters: {}", utils::format_number(chars as i64));
        println!("   📄  Lines     : {}", utils::format_number(lines as i64));
        println!("   💬  Words     : {}", utils::format_number(words as i64));
        println!("   💾  Size      : {}", utils::format_bytes(bytes));
    }

    if args.editor {
        editor::open(&output)?;
    }

    if args.delete && args.editor {
        editor::delete(&output)?;
    }

    Ok(())
}

fn log_startup(args: &RunArgs) {
    println!("🚀 Starting TreeClip...");
    println!("📁 Input Path: {}", args.input_path.display());
    println!(
        "📁 Output Path: {}",
        args.output_path
            .clone()
            .unwrap_or(path::PathBuf::from("."))
            .display()
    );
    println!("📋 Clipboard: {}", args.clipboard);
    println!("📊 Stats: {}", args.stats);
    println!("✏️  Editor: {}", args.editor);
    println!("🗑️  Delete: {}", args.delete);
    if !args.exclude.is_empty() {
        println!("🚫 Exclude patterns: {:?}", args.exclude);
    }
}
