use crate::commands::run::args;
use std::env;
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn run(run_args: args::RunArgs) -> anyhow::Result<()> {
    if run_args.verbose {
        println!("🚀 Starting TreeClip...");
        println!("📁 Input Path: {}", run_args.output_path);
        println!("📋 Clipboard: {}", run_args.clipboard);
        println!("📊 Stats: {}", run_args.stats);
        println!("✏️  Editor: {}", run_args.editor);
        println!("🗑️  Delete: {}", run_args.delete);
        if !run_args.exclude.is_empty() {
            println!("🚫 Exclude patterns: {:?}", run_args.exclude);
        }
    }

    // Your core logic would run here
    run_treeclip(
        &run_args.output_path,
        &run_args.exclude,
        &run_args.output_path,
        run_args.verbose,
    )?;

    // TODO: Implement clipboard and editor logic based on the boolean flags
    if run_args.clipboard {
        println!("(Would copy to clipboard)");
    }

    if run_args.editor {
        println!("(Would open in editor)");
    };

    Ok(())
}

fn run_treeclip(
    path: &str,
    _exclude_paths: &[String],
    _output_path: &str,
    _verbose: bool,
) -> anyhow::Result<()> {
    let path_buf = PathBuf::from(path);

    // Check if path exists
    if !path_buf.exists() {
        anyhow::bail!("Path does not exist: {}", path);
    }

    if path.eq(".") {
        let cwd = env::current_dir()?;
        println!("Traversing directory: {}", cwd.display());
    } else {
        println!("Traversing directory: {}", path_buf.display());
    }

    // Skip entries we can't access
    for entry in WalkDir::new(&path_buf).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            println!("📄 {}", path.display());
        } else if path.is_dir() {
            println!("📁 {}", path.display());
        }
    }

    Ok(())
}
