//! run - Main execution logic for the run command, orchestrating all operations.

use super::args::RunArgs;
use crate::core::{
    clipboard, editor,
    errors::FileSystemError,
    glob,
    traversal::walker,
    ui::{animations, banner, formatter, messages},
};
use anyhow::Context;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

/// Executes the main treeclip run command with the provided arguments.
///
/// This orchestrates the entire flow: configuration, traversal, clipboard, stats, and editor.
pub fn execute(mut args: RunArgs) -> anyhow::Result<()> {
    // Resolve negatable flags before processing.
    // If the user explicitly passes `--no-skip-hidden`, `no_skip_hidden` becomes true,
    // which forces our final `skip_hidden` evaluation to false.
    args.skip_hidden = args.skip_hidden && !args.no_skip_hidden;

    // Display welcome banner (respects fast mode)
    if !args.fast_mode {
        //note: Always fast mode
        // banner::print_welcome();
        args.fast_mode = true
    }

    // Normalize paths to absolute paths
    normalize_paths(&mut args)?;

    let root = args.root.as_ref().unwrap();
    let inputs = &args.input_paths.clone();
    let output = args.output_path.as_ref().unwrap();

    // Log configuration
    log_config(&args)?;

    // Execute traversal for each input path
    execute_traversal(&args, root, inputs, output)?;

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
///
/// Glob patterns in `input_paths` (e.g. `object/*`, `object*`, `object/*.go`)
/// are expanded into their concrete matches first, shell-independently and
/// using git-style glob semantics - so the same command behaves identically
/// whether the shell already expanded the glob or not.
fn normalize_paths(args: &mut RunArgs) -> anyhow::Result<()> {
    // Expand any glob-pattern input paths into concrete matches first.
    let raw_inputs: Vec<String> = args
        .input_paths
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect();
    let expanded_inputs =
        glob::expand_inputs(&raw_inputs).with_context(|| "Failed to expand glob input paths")?;
    args.input_paths = expanded_inputs;

    // Normalize input paths
    let mut normalized_input_paths = Vec::new();
    for input_path in &args.input_paths {
        let normalized_path = if input_path == Path::new(".") || input_path == Path::new("./") {
            env::current_dir()?
        } else {
            input_path.clone()
        };
        normalized_input_paths.push(normalized_path);
    }
    args.input_paths = normalized_input_paths;

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
    inputs: &[PathBuf],
    output: &Path,
) -> anyhow::Result<()> {
    println!("\n{}", messages::Messages::starting_adventure());

    if !args.fast_mode {
        animations::animated_dots(&messages::Messages::scanning_files(), 3, 300);
    }

    let walker = walker::Walker::new(root, inputs, output);

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

    let content = fs::read_to_string(output)
        .map_err(|e| FileSystemError::ReadFailed {
            path: output.to_path_buf(),
            source: e,
        })
        .with_context(|| format!("Failed to read file for stats: {}", output.display()))?;

    let lines = content.split('\n').count();
    let chars = content.chars().count();
    let words = content.split_whitespace().count();
    let bytes = content.len();

    let stats = formatter::StatsBox::new(lines, chars, words, bytes);
    println!("{}", stats.render());

    let (emoji, message) = stats.get_size_message();
    println!("  {} {}", emoji.bright_cyan(), message);

    Ok(())
}

/// Logs the current configuration settings to stdout.
#[rustfmt::skip]
fn log_config(args: &RunArgs) -> anyhow::Result<()> {
    let (root, inputs, output) = (
        args.root.as_ref(),
        &args.input_paths,
        args.output_path.as_ref(),
    );

    println!(
        "{}",
        formatter::ConfigFormatter::format_section_header("Paths to traverse", "📂")
    );
    for path in inputs {
        println!(
            "{}",
            formatter::ConfigFormatter::format_list_item("▸", &path.display().to_string())
        );
    }
    println!();
    println!(
        "{}",
        formatter::ConfigFormatter::format_section_header("Configuration Settings", "🔧")
    );
    let config_items = vec![
        ("🌍", "Root Path", formatter::ConfigFormatter::format_path(root.expect("root path must be supplied"))),
        ("💾", "Output Path", formatter::ConfigFormatter::format_path(output.expect("output path must be supplied"))),
        ("✏️", "Editor", formatter::ConfigFormatter::format_bool(args.editor)),
        ("🗑️", "Cleanup", formatter::ConfigFormatter::format_bool(args.delete)),
        ("📋", "Clipboard", formatter::ConfigFormatter::format_bool(args.clipboard)),
        ("📊", "Stats", formatter::ConfigFormatter::format_bool(args.stats)),
        ("👻", "Skip Hidden", formatter::ConfigFormatter::format_bool(args.skip_hidden)),
        ("⚡", "Fast Mode", formatter::ConfigFormatter::format_bool(args.fast_mode)),
        ("🌳", "Tree", formatter::ConfigFormatter::format_bool(args.tree)),
    ];

    for (icon, label, value) in config_items {
        println!(
            "{}",
            formatter::ConfigFormatter::format_config_line(icon, label, value)
        );
    }

    // Show ignore file settings
    println!();
    println!(
        "{}",
        formatter::ConfigFormatter::format_section_header("Ignore File Settings", "🔇")
    );
    let ignore_items = vec![
        ("📝", ".treeclipignore", formatter::ConfigFormatter::format_bool(true)), // Always respected
        ("🔧", "Respect Ignore Files", formatter::ConfigFormatter::format_bool(true)),
    ];

    for (icon, label, value) in ignore_items {
        println!(
            "{}",
            formatter::ConfigFormatter::format_config_line(icon, label, value)
        );
    }

    if !args.exclude.is_empty() {
        println!();
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

    println!();
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
            input_paths: vec![PathBuf::from(".")],
            output_path: Some(PathBuf::from(".")),
            root: Some(PathBuf::from(".")),
            exclude: vec![],
            clipboard: false,
            stats: false,
            editor: false,
            delete: false,
            verbose: false,
            skip_hidden: true,
            no_skip_hidden: false,
            raw: true,
            fast_mode: false,
            tree: false,
        };

        normalize_paths(&mut args)?;

        assert_ne!(args.input_paths[0], PathBuf::from("."));
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

    #[test]
    fn test_normalize_paths_expands_glob_input() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let object_dir = temp_dir.path().join("object");
        fs::create_dir(&object_dir)?;
        fs::write(object_dir.join("a.go"), "package main")?;
        fs::write(object_dir.join("b.go"), "package main")?;
        fs::write(object_dir.join("c.txt"), "not go")?;

        let original_dir = env::current_dir()?;
        env::set_current_dir(temp_dir.path())?;

        let mut args = RunArgs {
            input_paths: vec![PathBuf::from("object/*.go")],
            output_path: Some(PathBuf::from(".")),
            root: Some(PathBuf::from(".")),
            exclude: vec![],
            clipboard: false,
            stats: false,
            editor: false,
            delete: false,
            verbose: false,
            skip_hidden: true,
            no_skip_hidden: false,
            raw: true,
            fast_mode: false,
            tree: false,
        };

        let result = normalize_paths(&mut args);

        env::set_current_dir(original_dir)?;

        result?;
        assert_eq!(args.input_paths.len(), 2);
        assert!(
            args.input_paths
                .iter()
                .all(|p| p.extension().unwrap() == "go")
        );

        Ok(())
    }

    #[test]
    fn test_normalize_paths_glob_with_no_matches_errors() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;

        let original_dir = env::current_dir()?;
        env::set_current_dir(temp_dir.path())?;

        let mut args = RunArgs {
            input_paths: vec![PathBuf::from("nonexistent/*.go")],
            output_path: Some(PathBuf::from(".")),
            root: Some(PathBuf::from(".")),
            exclude: vec![],
            clipboard: false,
            stats: false,
            editor: false,
            delete: false,
            verbose: false,
            skip_hidden: true,
            no_skip_hidden: false,
            raw: true,
            fast_mode: false,
            tree: false,
        };

        let result = normalize_paths(&mut args);

        env::set_current_dir(original_dir)?;

        assert!(result.is_err());

        Ok(())
    }
}
