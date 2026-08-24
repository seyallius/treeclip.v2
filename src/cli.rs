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

Stop copy-pasting files one by one. Let TreeClip do the heavy lifting! (◕‿◕✿)

TIP: Run `treeclip` with no arguments to launch the interactive TUI -
pick files with multi-selection, exclude paths, set globs, and toggle
options with a beautiful, stateful interface. Pass a subcommand for the
headless CLI flow (great for CI/CD pipelines).

    treeclip                       # launch the TUI (interactive)
    treeclip run --clipboard       # CLI mode - bundle + copy to clipboard
    treeclip run 'src/**/*.rs'      # CLI mode - bundle by glob pattern

For more examples and usage patterns, visit:
https://github.com/seyallius/treeclip.v2?tab=readme-ov-file#how-to-use-it-

Made with ♡ by someone tired of copy-pasting code files!",
    next_line_help = true,
    disable_help_subcommand = true,
    styles = get_styles(),
    verbatim_doc_comment
)]
pub struct Cli {
    /// Subcommand. `None` means the user typed bare `treeclip` and we should
    /// launch the TUI.
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Available subcommands for TreeClip.
#[derive(Subcommand)]
pub enum Commands {
    /// Run TreeClip to extract and bundle code files.
    ///
    /// This is the headless CLI command that traverses your directory,
    /// extracts all file contents, and bundles them into a single
    /// output file with proper headers showing file paths.
    ///
    /// For interactive selection, just run `treeclip` with no subcommand.
    #[command(
        verbatim_doc_comment,
        after_help = "QUICK EXAMPLES:
    treeclip run                          # Extract current dir to treeclip_temp.txt
    treeclip run --clipboard              # Also copy to clipboard
    treeclip run ./src -o bundle.txt      # Custom input and output
    treeclip run -e node_modules -e .git  # Exclude patterns
    treeclip run \"src/**/*.go\"            # Glob input (git-style, shell-independent)

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
            Some(Commands::Run(args)) => {
                assert_eq!(args.input_paths, vec![PathBuf::from("test_dir")]);
            }
            _ => panic!("expected Run command"),
        }
    }

    #[test]
    fn test_cli_parse_multiple_input_paths() {
        let cli = Cli::parse_from(&["treeclip", "run", "dir1", "dir2", "dir3"]);
        match cli.command {
            Some(Commands::Run(args)) => {
                assert_eq!(args.input_paths.len(), 3);
                assert_eq!(args.input_paths[0], PathBuf::from("dir1"));
                assert_eq!(args.input_paths[1], PathBuf::from("dir2"));
                assert_eq!(args.input_paths[2], PathBuf::from("dir3"));
            }
            _ => panic!("expected Run command"),
        }
    }

    #[test]
    fn test_cli_parse_glob_input_path() {
        // Glob strings must parse through clap untouched; expansion happens
        // later in run::normalize_paths, not at the CLI parsing layer.
        let cli = Cli::parse_from(&["treeclip", "run", "object/*.go"]);
        match cli.command {
            Some(Commands::Run(args)) => {
                assert_eq!(args.input_paths, vec![PathBuf::from("object/*.go")]);
            }
            _ => panic!("expected Run command"),
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
            Some(Commands::Run(args)) => {
                assert_eq!(args.exclude, vec!["node_modules", ".git"]);
                assert_eq!(args.input_paths, vec![PathBuf::from(".")]);
            }
            _ => panic!("expected Run command"),
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
            Some(Commands::Run(args)) => {
                assert!(args.clipboard);
                assert!(args.editor);
                assert!(args.verbose);
            }
            _ => panic!("expected Run command"),
        }
    }

    #[test]
    fn test_cli_parse_with_fast_mode() {
        let cli = Cli::parse_from(&["treeclip", "run", ".", "--fast-mode"]);

        match cli.command {
            Some(Commands::Run(args)) => {
                assert!(args.fast_mode);
            }
            _ => panic!("expected Run command"),
        }
    }

    #[test]
    fn test_cli_no_subcommand_enters_tui_mode() {
        // NEW: bare `treeclip` now parses successfully and yields `None` for
        // the subcommand, which `main.rs` interprets as "launch the TUI".
        let cli = Cli::parse_from(&["treeclip"]);
        assert!(cli.command.is_none(), "bare treeclip should yield no subcommand");
    }

    #[test]
    fn test_cli_version_flag() {
        // Should still print version and exit (clap will Err with a DisplayHelp/Version).
        let _ = Cli::try_parse_from(&["treeclip", "--version"]);
    }

    #[test]
    fn test_cli_help_flag() {
        // Should still print help and exit.
        let _ = Cli::try_parse_from(&["treeclip", "--help"]);
    }
}
