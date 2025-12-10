use super::args::RunArgs;
use crate::core::traversal::walker;
use std::path;
use std::path::PathBuf;

pub fn execute(args: RunArgs) -> anyhow::Result<()> {
    if args.verbose {
        log_startup(&args);
    }

    // Run core logic
    let default_path = PathBuf::from(".");
    let output = args.output_path.as_ref().unwrap_or(&default_path);
    let walker = walker::Walker::new(&args.input_path, &output, &args.exclude);
    walker.process_dir(&args)?;

    if args.clipboard {
        // Will implement later
        println!("(Would copy to clipboard)");
    }

    if args.editor {
        println!("(Would open in editor)");
    }

    if args.delete && args.editor {
        println!("(Would delete after editor closes)");
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
