//! cli - Defines the command-line interface structure and available commands.

use crate::commands::args;
use clap::Parser;

/// Main CLI structure for TreeClip application.
#[derive(Parser)]
#[command(name = "treeclip")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Traverse directories and extract contents")]
#[command(
    long_about = "Traverse directories and files, extracting contents into a temporary folder and/or clipboard."
)]
#[command(next_line_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Available subcommands for TreeClip.
#[derive(clap::Subcommand)]
pub enum Commands {
    /// Run treeclip on a directory
    Run(args::RunArgs),
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
                assert_eq!(args.input_path, PathBuf::from("test_dir"));
            }
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
                assert_eq!(args.input_path, PathBuf::from("."));
            }
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
        }
    }

    #[test]
    fn test_cli_parse_with_fast_mode() {
        let cli = Cli::parse_from(&["treeclip", "run", ".", "--fast-mode"]);

        match cli.command {
            Commands::Run(args) => {
                assert!(args.fast_mode);
            }
        }
    }
}
