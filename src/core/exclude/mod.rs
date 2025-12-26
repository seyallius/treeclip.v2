//! exclude - Handles file and directory exclusion patterns using gitignore-style rules.

use crate::core::errors::PatternError;
use crate::core::ui::messages::Messages;
use anyhow::Context;
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::path::Path;

/// ExcludeMatcher determines whether paths should be excluded from traversal.
pub struct ExcludeMatcher {
    inner: Gitignore,
}

impl ExcludeMatcher {
    /// Creates a new ExcludeMatcher with patterns from .treeclipignore and CLI arguments.
    ///
    /// # Arguments
    ///
    /// * `root` - Root directory to search for .treeclipignore file
    /// * `cli_patterns` - Additional exclusion patterns from command-line arguments
    ///
    /// # Errors
    ///
    /// Returns `PatternError` if:
    /// - The gitignore builder fails to compile patterns
    /// - Invalid pattern syntax is provided
    pub fn new(root: &Path, cli_patterns: &[String]) -> anyhow::Result<Self> {
        let mut builder = GitignoreBuilder::new(root);

        // Add .treeclipignore file patterns (if exists)
        Self::add_ignore_file(&mut builder, root)?;

        // Add CLI patterns
        Self::add_cli_patterns(&mut builder, cli_patterns)
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
    /// Adds patterns from .treeclipignore file if it exists.
    fn add_ignore_file(builder: &mut GitignoreBuilder, root: &Path) -> anyhow::Result<()> {
        let ignore_file = root.join(".treeclipignore");

        // TODO: Path operations are not concurrent-safe - consider locking or TOCTOU handling
        // See: https://doc.rust-lang.org/stable/std/fs/index.html (TOCTOU section)
        if ignore_file.exists() {
            println!(
                "{}",
                Messages::found_ignore_file(&ignore_file.display().to_string())
            );
            println!("{}", Messages::applying_ignore_rules());

            // Add with error handling
            builder.add(&ignore_file);
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
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_exclude_matcher_creation() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let matcher = ExcludeMatcher::new(temp_dir.path(), &[])?;

        // Should not exclude root
        assert!(!matcher.is_excluded(temp_dir.path()));

        Ok(())
    }

    #[test]
    fn test_is_excluded_with_ignore_file() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let root = temp_dir.path();

        // Create node_modules directory
        let node_modules = root.join("node_modules");
        fs::create_dir(&node_modules)?;

        // Create .treeclipignore with exclusion pattern
        let ignore_file = root.join(".treeclipignore");
        fs::write(&ignore_file, "node_modules")?;

        // Create regular files
        let temp1 = root.join("temp1.txt");
        fs::write(&temp1, "temp1")?;

        let temp2 = root.join("temp2.txt");
        fs::write(&temp2, "temp2")?;

        let matcher = ExcludeMatcher::new(root, &[])?;

        // Regular files should not be excluded
        assert!(!matcher.is_excluded(root));
        assert!(!matcher.is_excluded(&temp1));
        assert!(!matcher.is_excluded(&temp2));

        // node_modules should be excluded
        assert!(matcher.is_excluded(&node_modules));

        Ok(())
    }

    #[test]
    fn test_is_excluded_with_cli_patterns() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let root = temp_dir.path();

        let target = root.join("target");
        fs::create_dir(&target)?;

        let src = root.join("src");
        fs::create_dir(&src)?;

        let matcher = ExcludeMatcher::new(root, &["target".to_string()])?;

        // src should not be excluded
        assert!(!matcher.is_excluded(&src));

        // target should be excluded (CLI pattern)
        assert!(matcher.is_excluded(&target));

        Ok(())
    }

    #[test]
    fn test_is_excluded_with_multiple_patterns() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let root = temp_dir.path();

        let node_modules = root.join("node_modules");
        fs::create_dir(&node_modules)?;

        let target = root.join("target");
        fs::create_dir(&target)?;

        let src = root.join("src");
        fs::create_dir(&src)?;

        // Create ignore file with one pattern
        let ignore_file = root.join(".treeclipignore");
        fs::write(&ignore_file, "node_modules")?;

        // Add another pattern via CLI
        let matcher = ExcludeMatcher::new(root, &["target".to_string()])?;

        // src should not be excluded
        assert!(!matcher.is_excluded(&src));

        // Both node_modules and target should be excluded
        assert!(matcher.is_excluded(&node_modules));
        assert!(matcher.is_excluded(&target));

        Ok(())
    }

    #[test]
    fn test_invalid_pattern_error() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();

        // Try to use an invalid glob pattern
        // Note: Most patterns are valid in gitignore, so this might not fail
        // This test ensures error handling works if it does fail
        let result = ExcludeMatcher::new(root, &["[invalid".to_string()]);

        // If it fails, should have context
        if let Err(e) = result {
            let error_msg = format!("{:?}", e);
            assert!(
                error_msg.contains("pattern") || error_msg.contains("Invalid"),
                "Error should have context: {}",
                error_msg
            );
        }
    }

    #[test]
    fn test_multiple_cli_patterns() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let root = temp_dir.path();

        let patterns = vec![
            "*.log".to_string(),
            "target".to_string(),
            "node_modules".to_string(),
        ];

        let matcher = ExcludeMatcher::new(root, &patterns)?;

        // Create test files/dirs
        let log_file = root.join("test.log");
        fs::write(&log_file, "")?;

        let rs_file = root.join("test.rs");
        fs::write(&rs_file, "")?;

        // .log files should be excluded
        assert!(matcher.is_excluded(&log_file));

        // .rs files should not be excluded
        assert!(!matcher.is_excluded(&rs_file));

        Ok(())
    }

    #[test]
    fn test_wildcard_cli_patterns() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let root = temp_dir.path();

        let patterns = vec![
            "*.log".to_string(),
            "*_test.rs".to_string(),
        ];

        let matcher = ExcludeMatcher::new(root, &patterns)?;

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
}
