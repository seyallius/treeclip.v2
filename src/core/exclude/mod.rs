//! exclude - Handles file and directory exclusion patterns using gitignore-style rules.

use crate::commands::args::RunArgs;
use crate::core::errors::{FileSystemError, PatternError};
use crate::core::ui::messages::Messages;
use anyhow::Context;
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// ExcludeMatcher determines whether paths should be excluded from traversal.
pub struct ExcludeMatcher {
    inner: Gitignore,
}

impl ExcludeMatcher {
    /// Creates a new ExcludeMatcher with patterns from various ignore files and CLI arguments.
    ///
    /// # Arguments
    ///
    /// * `root` - Root directory to search for ignore files
    /// * `args` - Run arguments containing ignore file preferences and CLI patterns
    ///
    /// # Errors
    ///
    /// Returns `PatternError` if:
    /// - The gitignore builder fails to compile patterns
    /// - Invalid pattern syntax is provided
    pub fn new(root: &Path, args: &RunArgs) -> anyhow::Result<Self> {
        let mut builder = GitignoreBuilder::new(root);

        // Always respect .treeclipignore if it exists
        Self::add_ignore_file(&mut builder, root, ".treeclipignore")?;
        Self::add_ignore_file(&mut builder, root, ".gitignore")?;
        Self::add_ignore_file(&mut builder, root, ".dockerignore")?;
        Self::add_ignore_file(&mut builder, root, ".npmignore")?;
        Self::add_ignore_file(&mut builder, root, ".terraformignore")?;

        // Add CLI patterns
        Self::add_cli_patterns(&mut builder, &args.exclude)
            .with_context(|| "Failed to process command-line exclusion patterns")?;

        let inner = builder
            .build()
            .map_err(|e| PatternError::BuildFailed { source: e })
            .with_context(|| {
                format!(
                    "Failed to build exclusion matcher for root: {}",
                    root.display()
                )
            })?;

        Ok(Self { inner })
    }

    /// Checks if a path should be excluded based on configured patterns.
    pub fn is_excluded(&self, path: &Path) -> bool {
        self.inner.matched(path, path.is_dir()).is_ignore()
    }
}

// -------------------------------------------- Private Helper Functions --------------------------------------------

impl ExcludeMatcher {
    /// Adds patterns from an ignore file if it exists in the root directory.
    fn add_ignore_file(
        builder: &mut GitignoreBuilder,
        root: &Path,
        filename: &str,
    ) -> anyhow::Result<()> {
        let ignore_file = root.join(filename);
        if ignore_file.exists() {
            Self::add_ignore_file_from_path(builder, &ignore_file)?;
            println!(
                "{}",
                Messages::found_ignore_file(&ignore_file.display().to_string())
            );
        }
        Ok(())
    }

    /// Adds patterns from an ignore file at the specified path.
    fn add_ignore_file_from_path(
        builder: &mut GitignoreBuilder,
        path: &Path,
    ) -> anyhow::Result<()> {
        let file = File::open(path)
            .map_err(|e| FileSystemError::ReadFailed {
                path: path.to_path_buf(),
                source: e,
            })
            .with_context(|| format!("Failed to open ignore file: {}", path.display()))?;

        let reader = BufReader::new(file);

        for (line_num, line) in reader.lines().enumerate() {
            let line = line
                .map_err(|e| FileSystemError::ReadFailed {
                    path: path.to_path_buf(),
                    source: e,
                })
                .with_context(|| {
                    format!(
                        "Failed to read line {} from ignore file: {}",
                        line_num + 1,
                        path.display()
                    )
                })?;

            // Skip empty lines and comments
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            // Add the pattern
            builder
                .add_line(Some(path.to_path_buf()), &line)
                .map_err(|e| PatternError::InvalidPattern {
                    pattern: line.clone(),
                    source: e,
                })
                .with_context(|| {
                    format!(
                        "Invalid pattern at line {} in {}: '{}'",
                        line_num + 1,
                        path.display(),
                        line
                    )
                })?;
        }

        Ok(())
    }

    /// Adds CLI-provided exclusion patterns to the builder.
    fn add_cli_patterns(
        builder: &mut GitignoreBuilder,
        cli_patterns: &[String],
    ) -> anyhow::Result<()> {
        for (index, pat) in cli_patterns.iter().enumerate() {
            builder
                .add_line(None, pat)
                .map_err(|e| PatternError::InvalidPattern {
                    pattern: pat.clone(),
                    source: e,
                })
                .with_context(|| {
                    format!(
                        "Invalid exclusion pattern #{}: '{}' - check pattern syntax",
                        index + 1,
                        pat
                    )
                })?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod exclude_tests {
    use super::*;
    use crate::commands::args::RunArgs;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn create_test_args(root: PathBuf) -> RunArgs {
        RunArgs {
            input_paths: vec![root.clone()],
            output_path: Some(root.join("output.txt")),
            root: Some(root),
            exclude: vec![],
            clipboard: false,
            stats: false,
            editor: false,
            delete: false,
            verbose: false,
            skip_hidden: true,
            no_skip_hidden: false,
            raw: true,
            fast_mode: true,
            tree: false,
            interactive: false,
        }
    }

    #[test]
    fn test_exclude_matcher_creation() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let args = create_test_args(temp_dir.path().to_path_buf());
        let matcher = ExcludeMatcher::new(temp_dir.path(), &args)?;

        // Should not exclude root
        assert!(!matcher.is_excluded(temp_dir.path()));

        Ok(())
    }

    #[test]
    fn test_respects_treeclipignore() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let root = temp_dir.path();

        // Create node_modules directory
        let node_modules = root.join("node_modules");
        fs::create_dir(&node_modules)?;

        // Create .treeclipignore with exclusion pattern
        let ignore_file = root.join(".treeclipignore");
        fs::write(&ignore_file, "node_modules")?;

        let args = create_test_args(root.to_path_buf());
        let matcher = ExcludeMatcher::new(root, &args)?;

        // node_modules should be excluded
        assert!(matcher.is_excluded(&node_modules));

        Ok(())
    }

    #[test]
    fn test_respects_gitignore() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let root = temp_dir.path();

        // Create target directory
        let target = root.join("target");
        fs::create_dir(&target)?;

        // Create .gitignore
        fs::write(root.join(".gitignore"), "target\n*.log")?;

        let args = create_test_args(root.to_path_buf());
        let matcher = ExcludeMatcher::new(root, &args)?;

        // target should be excluded
        assert!(matcher.is_excluded(&target));

        // log files should be excluded
        let log_file = root.join("test.log");
        fs::write(&log_file, "")?;
        assert!(matcher.is_excluded(&log_file));

        Ok(())
    }

    #[test]
    fn test_cli_patterns_override() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let root = temp_dir.path();

        let dist = root.join("dist");
        fs::create_dir(&dist)?;

        let mut args = create_test_args(root.to_path_buf());
        args.exclude = vec!["dist".to_string(), "*.min.js".to_string()];

        let matcher = ExcludeMatcher::new(root, &args)?;

        // CLI patterns should work
        assert!(matcher.is_excluded(&dist));

        let min_js = root.join("app.min.js");
        fs::write(&min_js, "")?;
        assert!(matcher.is_excluded(&min_js));

        Ok(())
    }

    #[test]
    fn test_wildcard_cli_patterns() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let root = temp_dir.path();

        let patterns = vec!["*.log".to_string(), "*_test.rs".to_string()];

        let run_args = RunArgs {
            exclude: patterns,

            input_paths: vec![],
            output_path: None,
            root: None,
            clipboard: false,
            stats: false,
            editor: false,
            delete: false,
            verbose: false,
            skip_hidden: false,
            no_skip_hidden: false,
            raw: false,
            fast_mode: false,
            tree: false,
            interactive: false,
        };
        let matcher = ExcludeMatcher::new(root, &run_args)?;

        // Create test files/dirs
        let log_file = root.join("test.log");
        fs::write(&log_file, "")?;

        let rs_file = root.join("main.rs");
        fs::write(&rs_file, "")?;

        let test_rs_file = root.join("main_test.rs");
        fs::write(&test_rs_file, "")?;

        // .log files should be excluded
        assert!(matcher.is_excluded(&log_file));

        // .rs files should not be excluded
        assert!(!matcher.is_excluded(&rs_file));

        // *_test.rs files should be excluded
        assert!(matcher.is_excluded(&test_rs_file));

        Ok(())
    }

    #[test]
    fn test_ignores_comments_and_empty_lines() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let root = temp_dir.path();

        // Create ignore file with comments and empty lines
        fs::write(
            root.join(".gitignore"),
            "# This is a comment\nnode_modules\n\n# Another comment\n*.log\n\n",
        )?;

        let node_modules = root.join("node_modules");
        fs::create_dir(&node_modules)?;

        let log_file = root.join("test.log");
        fs::write(&log_file, "")?;

        let args = create_test_args(root.to_path_buf());
        let matcher = ExcludeMatcher::new(root, &args)?;

        // Patterns should work despite comments
        assert!(matcher.is_excluded(&node_modules));
        assert!(matcher.is_excluded(&log_file));

        Ok(())
    }
}
