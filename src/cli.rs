//! cli - Defines the command-line interface structure and available commands.

use crate::commands::args;
use clap::{Parser, Subcommand};

/// Main CLI structure for TreeClip application.
#[derive(Parser)]
#[command(
    name = "treeclip",
    version = env!("CARGO_PKG_VERSION"),
    author = "Seyedali <your-email@example.com>",
    about = "🌳 TreeClip - Bundle your code for AI assistants",
    long_about = "TreeClip traverses directories and extracts all file contents into a single,
AI-friendly format with proper headers. Perfect for sharing entire codebases
with ChatGPT, Claude, or any AI assistant!

Stop copy-pasting files one by one. Let TreeClip do the heavy lifting! (◕‿◕✿)",
    after_help = "EXAMPLES:
    # Quick clipboard copy of current directory
    treeclip run --clipboard

    # Extract specific directory with exclusions
    treeclip run ./src -e node_modules -e target --clipboard

    # Review output before sharing
    treeclip run --editor --delete --stats

    # Fast mode for CI/CD
    treeclip run --fast-mode -o output.txt

For more examples and usage patterns, visit:
https://github.com/seyallius/treeclip.v2?tab=readme-ov-file#how-to-use-it-

Made with ♡ by someone tired of copy-pasting code files!",
    next_line_help = true,
    arg_required_else_help = true,
    disable_help_subcommand = true,
    styles = get_styles(),
    verbatim_doc_comment
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Available subcommands for TreeClip.
#[derive(Subcommand)]
pub enum Commands {
    /// Run TreeClip to extract and bundle code files
    ///
    /// This is the main command that traverses your directory,
    /// extracts all file contents, and bundles them into a single
    /// output file with proper headers showing file paths.
    ///
    /// Perfect for sharing codebases with AI assistants! 🤖
    #[command(
        verbatim_doc_comment,
        after_help = "QUICK EXAMPLES:
    treeclip run                          # Extract current dir to treeclip_temp.txt
    treeclip run --clipboard              # Also copy to clipboard
    treeclip run ./src -o bundle.txt      # Custom input and output
    treeclip run -e node_modules -e .git  # Exclude patterns

TIP: Create a .treeclipignore file (like .gitignore) for permanent exclusions!"
    )]
    Run(args::RunArgs),

    Init(args::InitArgs),
}

// -------------------------------------------- Private Helper Functions --------------------------------------------

/// Gets custom clap styles for colorized help output.
fn get_styles() -> clap::builder::Styles {
    use clap::builder::styling::*;

    Styles::styled()
        .header(
            Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::Cyan))),
        )
        .usage(
            Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::Cyan))),
        )
        .literal(
            Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::Green))),
        )
        .placeholder(Style::new().fg_color(Some(Color::Ansi(AnsiColor::Yellow))))
        .error(
            Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::Red))),
        )
        .valid(
            Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::Green))),
        )
        .invalid(
            Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::Red))),
        )
}

#[cfg(test)]
mod cli_tests {
    use super::*;
    use clap::Parser;
    use std::path::PathBuf;

    #[test]
    fn test_cli_parse_run_command() {
        let cli = Cli::parse_from(&["treeclip", "run", "test_dir"]);
        match cli.command {
            Commands::Run(args) => {
                assert_eq!(args.input_paths, vec![PathBuf::from("test_dir")]);
            }
            _ => {}
        }
    }

    #[test]
    fn test_cli_parse_multiple_input_paths() {
        let cli = Cli::parse_from(&["treeclip", "run", "dir1", "dir2", "dir3"]);
        match cli.command {
            Commands::Run(args) => {
                assert_eq!(args.input_paths.len(), 3);
                assert_eq!(args.input_paths[0], PathBuf::from("dir1"));
                assert_eq!(args.input_paths[1], PathBuf::from("dir2"));
                assert_eq!(args.input_paths[2], PathBuf::from("dir3"));
            }
            _ => {}
        }
    }

    #[test]
    fn test_cli_parse_run_with_exclude() {
        let cli = Cli::parse_from(&[
            "treeclip",
            "run",
            ".",
            "--exclude",
            "node_modules",
            "--exclude",
            ".git",
        ]);

        match cli.command {
            Commands::Run(args) => {
                assert_eq!(args.exclude, vec!["node_modules", ".git"]);
                assert_eq!(args.input_paths, vec![PathBuf::from(".")]);
            }
            _ => {}
        }
    }

    #[test]
    fn test_cli_parse_run_with_flags() {
        let cli = Cli::parse_from(&[
            "treeclip",
            "run",
            ".",
            "--clipboard",
            "--editor",
            "--verbose",
        ]);

        match cli.command {
            Commands::Run(args) => {
                assert!(args.clipboard);
                assert!(args.editor);
                assert!(args.verbose);
            }
            _ => {}
        }
    }

    #[test]
    fn test_cli_parse_with_fast_mode() {
        let cli = Cli::parse_from(&["treeclip", "run", ".", "--fast-mode"]);

        match cli.command {
            Commands::Run(args) => {
                assert!(args.fast_mode);
            }
            _ => {}
        }
    }

    #[test]
    fn test_cli_requires_subcommand() {
        let result = Cli::try_parse_from(&["treeclip"]);
        // Should fail because arg_required_else_help = true
        assert!(result.is_err());
    }

    #[test]
    fn test_cli_version_flag() {
        // Just ensure it doesn't panic
        let result = Cli::try_parse_from(&["treeclip", "--version"]);
        // Will fail in test but shouldn't panic
        let _ = result;
    }

    #[test]
    fn test_cli_help_flag() {
        // Just ensure it doesn't panic
        let result = Cli::try_parse_from(&["treeclip", "--help"]);
        // Will fail in test but shouldn't panic
        let _ = result;
    }
}
