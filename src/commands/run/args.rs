use std::path::PathBuf;

#[derive(clap::Args)]
pub struct RunArgs {
    /// Path to traverse (defaults to current directory)
    #[arg(default_value = ".", value_parser = validate_path)]
    pub input_path: PathBuf,

    /// Output path for extracted file
    #[arg(default_value = ".", value_parser = validate_path)]
    pub output_path: Option<PathBuf>,

    /// Exclude files/folders matching these patterns
    #[arg(short, long)]
    pub exclude: Vec<String>,

    /// Copy output to clipboard
    #[arg(long, default_value_t = true)]
    pub clipboard: bool,

    /// Show clipboard content statistics
    #[arg(long, default_value_t = false)]
    pub stats: bool,

    /// Open output file in the default text editor
    #[arg(long, default_value_t = false)]
    pub editor: bool,

    /// Delete the output file after editor is closed
    #[arg(long, default_value_t = false)]
    pub delete: bool,

    /// Verbose output
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,

    /// Skip hidden files and folders in Unix systems
    #[arg(short, long, default_value_t = true)]
    pub skip_hidden: bool,

    /// Indicates whether to extract raw content or with additional metadata
    /// such as path of each file, file structure (tree), markdown formatted
    /// if code, etc.
    #[arg(short, long, default_value_t = true)]
    pub raw: bool,
}

fn validate_path(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);
    // Basic validation - path doesn't need to exist yet
    if s.is_empty() {
        return Err("Path cannot be empty".to_string());
    }
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::{Cli, Commands};

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
    fn test_run_args_default_values() {
        use clap::Parser;

        let cli = Cli::parse_from(&["treeclip", "run"]);
        match cli.command {
            Commands::Run(args) => {
                assert_eq!(args.input_path, PathBuf::from("."));
                assert!(args.output_path.is_some());
                assert!(args.clipboard);
                assert!(!args.stats);
                assert!(!args.editor);
                assert!(!args.delete);
                assert!(!args.verbose);
                assert!(args.skip_hidden);
                assert!(args.exclude.is_empty());
            }
        }
    }
}
