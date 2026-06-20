//! args - Defines command-line arguments and their validation logic.

use clap::{ArgAction, ValueHint};
use std::path::PathBuf;

/// Arguments for the `run` command.
#[derive(clap::Args)]
pub struct RunArgs {
    /// Paths to traverse (defaults to current directory)
    ///
    /// Specify which directories to scan and extract files from.
    /// Use '.' for current directory or provide any valid paths.
    /// Multiple paths can be specified.
    ///
    /// Glob patterns are supported and expanded shell-independently
    /// using git-style matching (the same engine behind .gitignore),
    /// so they work identically whether or not your shell already
    /// expanded them - handy on Windows or when quoted on Unix.
    ///
    /// Examples:
    ///   treeclip run .
    ///   treeclip run ./src
    ///   treeclip run ~/projects/my-app
    ///   treeclip run . src some/other/input/path
    ///   treeclip run object/*
    ///   treeclip run "object*"
    ///   treeclip run "object/*.go"
    #[arg(
        default_value = ".",
        value_parser = validate_path,
        value_hint = ValueHint::DirPath,
        verbatim_doc_comment,
        num_args = 1..,
    )]
    pub input_paths: Vec<PathBuf>,

    /// Output file path for the extracted content
    ///
    /// Where to save the bundled output. If not specified,
    /// creates 'treeclip_temp.txt' in the current directory.
    ///
    /// Examples:
    ///   -o output.txt
    ///   --output-path ./exports/bundle.txt
    #[arg(
        short,
        long,
        default_value = ".",
        value_parser = validate_path,
        value_name = "FILE",
        value_hint = ValueHint::FilePath,
        verbatim_doc_comment
    )]
    pub output_path: Option<PathBuf>,

    /// Root directory for .treeclipignore file lookup
    ///
    /// Specifies where to search for the .treeclipignore file.
    /// Usually you don't need to change this.
    #[arg(
        long,
        default_value = ".",
        value_parser = validate_path,
        value_name = "DIR",
        value_hint = ValueHint::DirPath,
        hide = true  // Hide from help unless --help is used
    )]
    pub root: Option<PathBuf>,

    /// Exclude files/folders matching these glob patterns
    ///
    /// Can be specified multiple times. Supports glob patterns
    /// like '*.log', 'node_modules', 'target/**', etc.
    ///
    /// Common patterns:
    ///   -e node_modules      (exclude directory)
    ///   -e '*.log'           (exclude all .log files)
    ///   -e 'target'          (exclude Rust build dir)
    ///   -e '__pycache__'     (exclude Python cache)
    ///
    /// Tip: Use .treeclipignore file for permanent exclusions!
    #[arg(
        short,
        long,
        value_name = "PATTERN",
        action = ArgAction::Append,
        verbatim_doc_comment
    )]
    pub exclude: Vec<String>,

    /// Copy the output to system clipboard
    ///
    /// After extraction, automatically copies the entire
    /// output to your clipboard for easy pasting into AI chats.
    ///
    /// Platform notes:
    ///   • Windows/macOS: Works out of the box
    ///   • Linux: Requires xclip or wl-clipboard
    #[arg(short, long, default_value_t = false, verbatim_doc_comment)]
    pub clipboard: bool,

    /// Show detailed statistics about the extracted content
    ///
    /// Displays:
    ///   • Total lines, words, and characters
    ///   • File size in human-readable format
    ///   • Fun emoji feedback based on size 🐣🐘🐋
    #[arg(long, default_value_t = false, verbatim_doc_comment)]
    pub stats: bool,

    /// Open the output file in your default text editor
    ///
    /// After extraction, opens the file for review/editing.
    /// Respects $EDITOR environment variable on Unix systems.
    ///
    /// Combine with --delete to auto-cleanup after closing.
    #[arg(long, default_value_t = false, verbatim_doc_comment)]
    pub editor: bool,

    /// Delete the output file after closing the editor
    ///
    /// Only works when used with --editor flag.
    /// Perfect for temporary reviews without leaving files behind.
    ///
    /// Example:
    ///   treeclip run --editor --delete  (review then cleanup)
    #[arg(
        long,
        default_value_t = false,
        requires = "editor",
        verbatim_doc_comment
    )]
    pub delete: bool,

    /// Enable verbose output with detailed progress information
    ///
    /// Shows:
    ///   • File-by-file processing updates
    ///   • Progress counters with cute emojis
    ///   • Detailed operation logging
    ///
    /// Useful for debugging or understanding what's included.
    #[arg(short, long, default_value_t = false, verbatim_doc_comment)]
    pub verbose: bool,

    /// Skip hidden files and folders (starting with '.')
    ///
    /// Enabled by default. Use --no-skip-hidden to include
    /// hidden files like .env.example, .editorconfig, etc.
    ///
    /// Examples of skipped files:
    ///   • .git/
    ///   • .env
    ///   • .DS_Store
    ///   • .vscode/
    ///
    /// Note: Files listed in .treeclipignore or .gitignore are
    /// still excluded regardless of this flag.
    #[arg(
        short = 'H',
        long = "skip-hidden",
        default_value_t = true,
        action = ArgAction::SetTrue,
        overrides_with("no_skip_hidden"),
        verbatim_doc_comment
    )]
    pub skip_hidden: bool,

    /// Include hidden files and folders (negates --skip-hidden)
    ///
    /// This is a hidden flag that clap uses to allow users to disable
    /// the default `--skip-hidden` behavior.
    #[arg(
        long = "no-skip-hidden",
        action = ArgAction::SetTrue,
        overrides_with("skip_hidden"),
        hide = true
    )]
    pub no_skip_hidden: bool,

    /// Extract raw content without additional metadata
    ///
    /// Currently always enabled. Future versions may add
    /// metadata like file timestamps, sizes, or checksums.
    #[arg(
        short,
        long,
        default_value_t = true,
        hide = true  // Hide until we implement non-raw mode
    )]
    pub raw: bool,

    /// Fast mode: skip animations and execute instantly
    ///
    /// Disables:
    ///   • Welcome banner
    ///   • Progress spinners and animations
    ///   • Cute tree emojis 🌳 (sorry!)
    ///
    /// Perfect for:
    ///   • CI/CD pipelines
    ///   • Shell scripts
    ///   • Large projects where speed matters
    ///   • When you're in a hurry!
    #[arg(short, long, default_value_t = false, verbatim_doc_comment)]
    pub fast_mode: bool,

    /// Tree structure: show and write the tree structure of traversed dir(s)/file(s).
    #[arg(short, long, default_value_t = false, verbatim_doc_comment)]
    pub tree: bool,
}

/// Arguments for the init command.
#[derive(clap::Args, Debug)]
pub struct InitArgs {
    /// Target directory where .treeclipignore will be created.
    ///
    /// Defaults to current directory if not specified.
    #[arg(short = 'd', long, default_value = ".")]
    pub directory: PathBuf,

    /// Force overwrite existing .treeclipignore without prompting.
    #[arg(short = 'f', long)]
    pub force: bool,
}

// -------------------------------------------- Private Helper Functions --------------------------------------------

impl Default for InitArgs {
    fn default() -> Self {
        Self {
            directory: PathBuf::from("."),
            force: false,
        }
    }
}

impl InitArgs {
    /// Validates the init arguments.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The specified directory does not exist
    /// - The directory is not writable
    pub fn validate(&self) -> anyhow::Result<()> {
        if !self.directory.exists() {
            anyhow::bail!("Directory does not exist: {}", self.directory.display());
        }

        if !self.directory.is_dir() {
            anyhow::bail!("Path is not a directory: {}", self.directory.display());
        }

        Ok(())
    }
}

/// Validates that a path string is not empty.
///
/// Returns an error message if validation fails.
fn validate_path(s: &str) -> Result<PathBuf, String> {
    if s.trim().is_empty() {
        return Err("Path cannot be empty".to_string());
    }
    Ok(PathBuf::from(s))
}

#[cfg(test)]
mod args_tests {
    use super::*;
    use crate::cli::{Cli, Commands};
    use clap::Parser;

    #[test]
    fn test_validate_path_valid() {
        let result = validate_path(".");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PathBuf::from("."));
    }

    #[test]
    fn test_validate_path_empty() {
        let result = validate_path("");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot be empty"));
    }

    #[test]
    fn test_validate_path_whitespace() {
        let result = validate_path("   ");
        assert!(result.is_err());
    }

    #[test]
    fn test_run_args_default_values() {
        let cli = Cli::parse_from(&["treeclip", "run"]);
        match cli.command {
            Commands::Run(args) => {
                assert_eq!(args.input_paths, vec![PathBuf::from(".")]);
                assert!(args.output_path.is_some());
                assert!(!args.clipboard);
                assert!(!args.stats);
                assert!(!args.editor);
                assert!(!args.delete);
                assert!(!args.verbose);
                assert!(!args.fast_mode);
                assert!(!args.skip_hidden);
                assert!(args.exclude.is_empty());
            }
            _ => {}
        }
    }

    #[test]
    fn test_multiple_input_paths() {
        let cli = Cli::parse_from(&["treeclip", "run", ".", "src", "some/other/input/path"]);
        match cli.command {
            Commands::Run(args) => {
                assert_eq!(args.input_paths.len(), 3);
                assert_eq!(args.input_paths[0], PathBuf::from("."));
                assert_eq!(args.input_paths[1], PathBuf::from("src"));
                assert_eq!(args.input_paths[2], PathBuf::from("some/other/input/path"));
            }
            _ => {}
        }
    }

    #[test]
    fn test_fast_mode_flag() {
        let cli = Cli::parse_from(&["treeclip", "run", ".", "--fast-mode"]);
        match cli.command {
            Commands::Run(args) => {
                assert!(args.fast_mode);
            }
            _ => {}
        }
    }

    #[test]
    fn test_multiple_exclude_patterns() {
        let cli = Cli::parse_from(&[
            "treeclip",
            "run",
            ".",
            "-e",
            "node_modules",
            "-e",
            "target",
            "-e",
            "*.log",
        ]);
        match cli.command {
            Commands::Run(args) => {
                assert_eq!(args.exclude.len(), 3);
                assert!(args.exclude.contains(&"node_modules".to_string()));
                assert!(args.exclude.contains(&"target".to_string()));
                assert!(args.exclude.contains(&"*.log".to_string()));
            }
            _ => {}
        }
    }

    #[test]
    fn test_delete_requires_editor() {
        // This should fail because --delete requires --editor
        let result = Cli::try_parse_from(&["treeclip", "run", ".", "--delete"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_with_editor_works() {
        let cli = Cli::parse_from(&["treeclip", "run", ".", "--editor", "--delete"]);
        match cli.command {
            Commands::Run(args) => {
                assert!(args.editor);
                assert!(args.delete);
            }
            _ => {}
        }
    }

    #[test]
    fn test_verbose_and_fast_mode_combination() {
        // These can both be enabled (verbose will be ignored in fast mode)
        let cli = Cli::parse_from(&["treeclip", "run", ".", "--verbose", "--fast-mode"]);
        match cli.command {
            Commands::Run(args) => {
                assert!(args.verbose);
                assert!(args.fast_mode);
            }
            _ => {}
        }
    }

    #[test]
    fn test_clipboard_and_stats_combination() {
        let cli = Cli::parse_from(&["treeclip", "run", ".", "--clipboard", "--stats"]);
        match cli.command {
            Commands::Run(args) => {
                assert!(args.clipboard);
                assert!(args.stats);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod init_args_tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_validate_with_valid_directory() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let args = InitArgs {
            directory: temp_dir.path().to_path_buf(),
            force: false,
        };

        assert!(args.validate().is_ok());
        Ok(())
    }

    #[test]
    fn test_validate_with_nonexistent_directory() {
        let args = InitArgs {
            directory: PathBuf::from("/nonexistent/directory"),
            force: false,
        };

        assert!(args.validate().is_err());
    }

    #[test]
    fn test_validate_with_file_instead_of_directory() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = temp_dir.path().join("file.txt");
        fs::write(&file_path, "test")?;

        let args = InitArgs {
            directory: file_path,
            force: false,
        };

        assert!(args.validate().is_err());
        Ok(())
    }
}

