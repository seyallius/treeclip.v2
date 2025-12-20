//! run - Main execution logic for the run command, orchestrating all operations.

use super::args::RunArgs;
use crate::core::ui::{animations, banner, formatter, messages};
use crate::core::{clipboard, editor, traversal::walker};
use std::path::{Path, PathBuf};
use std::{env, fs};

/// Executes the main treeclip run command with the provided arguments.
///
/// This orchestrates the entire flow: configuration, traversal, clipboard, stats, and editor.
pub fn execute(mut args: RunArgs) -> anyhow::Result<()> {
    // Display welcome banner (respects fast mode)
    if !args.fast_mode {
        banner::print_welcome();
    }

    // Normalize paths to absolute paths
    normalize_paths(&mut args)?;

    let root = args.root.as_ref().unwrap();
    let input = &args.input_path;
    let output = args.output_path.as_ref().unwrap();

    // Log configuration
    log_config(&args)?;

    // Execute traversal
    execute_traversal(&args, root, input, output)?;

    // Handle clipboard operations
    handle_clipboard(&args, output)?;

    // Show statistics if requested
    if args.stats {
        show_stats_section(&args, output)?;
    }

    // Handle editor operations
    handle_editor(&args, output)?;

    // Display goodbye message (respects fast mode)
    if !args.fast_mode {
        banner::print_goodbye();
    }

    Ok(())
}

// -------------------------------------------- Private Helper Functions --------------------------------------------

/// Normalizes all path arguments to absolute paths.
fn normalize_paths(args: &mut RunArgs) -> anyhow::Result<()> {
    // Normalize input path
    args.input_path = if args.input_path == Path::new(".") || args.input_path == Path::new("./") {
        env::current_dir()?
    } else {
        args.input_path.clone()
    };

    // Normalize output path
    args.output_path = match &args.output_path {
        Some(path) if path == Path::new(".") => Some(PathBuf::from("./treeclip_temp.txt")),
        Some(path) => Some(path.clone()),
        None => Some(PathBuf::from("./treeclip_temp.txt")),
    };

    // Normalize root path
    args.root = match &args.root {
        Some(path) if path == Path::new(".") => Some(env::current_dir()?),
        Some(path) => Some(path.to_path_buf()),
        None => Some(env::current_dir()?),
    };

    Ok(())
}

/// Executes the directory traversal operation.
fn execute_traversal(
    args: &RunArgs,
    root: &Path,
    input: &Path,
    output: &Path,
) -> anyhow::Result<()> {
    println!("\n{}", messages::Messages::starting_adventure());

    if !args.fast_mode {
        animations::animated_dots(&messages::Messages::scanning_files(), 3, 300);
    }

    let walker = walker::Walker::new(root, input, output, &args.exclude);

    if !args.fast_mode {
        let spinner = animations::Spinner::new_tree();
        spinner.spin(&messages::Messages::traversing_tree(), 1200);
    }

    walker.process_dir(args)?;

    println!("\n{}", messages::Messages::gathering_leaves());

    Ok(())
}

/// Handles clipboard copy operations.
fn handle_clipboard(args: &RunArgs, output: &Path) -> anyhow::Result<()> {
    let mut clip = clipboard::Clipboard::new(output)?;

    if args.clipboard {
        if !args.fast_mode {
            let spinner = animations::Spinner::new_loading();
            spinner.spin(&messages::Messages::copying_clipboard(), 800);
        }

        clip.set_clipboard()?;
        println!("{}", messages::Messages::clipboard_ready());
    } else {
        println!("{}", messages::Messages::clipboard_skipped());
    }

    Ok(())
}

/// Shows statistics section with formatted output.
fn show_stats_section(args: &RunArgs, output: &Path) -> anyhow::Result<()> {
    if !args.fast_mode {
        println!("\n{}", messages::Messages::showing_stats());
    }

    show_stats(output)?;
    Ok(())
}

/// Handles editor opening and cleanup operations.
fn handle_editor(args: &RunArgs, output: &Path) -> anyhow::Result<()> {
    if args.editor {
        if !args.fast_mode {
            println!("\n{}", messages::Messages::opening_editor());
        }

        editor::open(output)?;

        if !args.fast_mode {
            println!("{}", messages::Messages::editor_opened());
        }

        // Handle file deletion after editor closes
        if args.delete {
            if !args.fast_mode {
                println!("\n{}", messages::Messages::cleaning_up());
            }

            editor::delete(output)?;

            if !args.fast_mode {
                println!("{}", messages::Messages::cleaned_up());
            }
        }
    }

    Ok(())
}

/// Displays content statistics for the output file.
fn show_stats(output: &Path) -> anyhow::Result<()> {
    use colored::Colorize;

    let content = fs::read_to_string(output)?;
    let lines = content.split('\n').count();
    let chars = content.chars().count();
    let words = content.split_whitespace().count();
    let bytes = content.len();

    let stats = formatter::StatsBox::new(lines, chars, words, bytes);
    println!("{}", stats.render().bright_cyan());

    let (emoji, message) = stats.get_size_message();
    println!("  {emoji} {message}");

    Ok(())
}

/// Logs the current configuration settings to stdout.
#[rustfmt::skip]
fn log_config(args: &RunArgs) -> anyhow::Result<()> {
    let (root, input, output) = (
        args.root.as_ref(),
        &args.input_path,
        args.output_path.as_ref(),
    );

    println!(
        "{}",
        formatter::ConfigFormatter::format_section_header("Configuration Settings", "🔧")
    );

    let config_items = vec![
        ("🌍", "Root Path", formatter::ConfigFormatter::format_path(root.expect("root path must be supplied"))),
        ("📂", "Input Path", formatter::ConfigFormatter::format_path(input)),
        ("💾", "Output Path", formatter::ConfigFormatter::format_path(output.expect("output path must be supplied"))),
        ("✏️", "Editor", formatter::ConfigFormatter::format_bool(args.editor)),
        ("🗑️", "Cleanup", formatter::ConfigFormatter::format_bool(args.delete)),
        ("📋", "Clipboard", formatter::ConfigFormatter::format_bool(args.clipboard)),
        ("📊", "Stats", formatter::ConfigFormatter::format_bool(args.stats)),
        ("👻", "Skip Hidden", formatter::ConfigFormatter::format_bool(args.skip_hidden)),
        ("⚡", "Fast Mode", formatter::ConfigFormatter::format_bool(args.fast_mode)),
    ];

    for (icon, label, value) in config_items {
        println!(
            "{}",
            formatter::ConfigFormatter::format_config_line(icon, label, value)
        );
    }

    if !args.exclude.is_empty() {
        println!(
            "{}",
            formatter::ConfigFormatter::format_section_header("Excluded Patterns", "🚫")
        );
        for pattern in &args.exclude {
            println!(
                "{}",
                formatter::ConfigFormatter::format_list_item("▸", pattern)
            );
        }
    }

    println!("{}", messages::Messages::ready_to_launch());
    Ok(())
}

#[cfg(test)]
mod run_tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_normalize_paths_current_dir() -> anyhow::Result<()> {
        let mut args = RunArgs {
            input_path: PathBuf::from("."),
            output_path: Some(PathBuf::from(".")),
            root: Some(PathBuf::from(".")),
            exclude: vec![],
            clipboard: false,
            stats: false,
            editor: false,
            delete: false,
            verbose: false,
            skip_hidden: true,
            raw: true,
            fast_mode: false,
        };

        normalize_paths(&mut args)?;

        assert_ne!(args.input_path, PathBuf::from("."));
        assert!(args.output_path.is_some());
        assert!(args.root.is_some());

        Ok(())
    }

    #[test]
    fn test_show_stats_with_content() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let output_path = temp_dir.path().join("output.txt");
        fs::write(&output_path, "Hello\nWorld\nTest content")?;

        // This should not panic
        let result = show_stats(&output_path);
        assert!(result.is_ok());

        Ok(())
    }
}
