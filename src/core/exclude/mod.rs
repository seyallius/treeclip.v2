//! exclude - Handles file and directory exclusion patterns using gitignore-style rules.

use crate::core::ui::messages::Messages;
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
    /// Returns an error if the gitignore builder fails to compile patterns.
    pub fn new(root: &Path, cli_patterns: &[String]) -> anyhow::Result<Self> {
        let mut builder = GitignoreBuilder::new(root);

        // Add .treeclipignore file patterns (if exists)
        Self::add_ignore_file(&mut builder, root);

        // Add CLI patterns
        Self::add_cli_patterns(&mut builder, cli_patterns)?;

        let inner = builder.build()?;
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
    fn add_ignore_file(builder: &mut GitignoreBuilder, root: &Path) {
        let ignore_file = root.join(".treeclipignore");

        // TODO: Path operations are not concurrent-safe - consider locking or TOCTOU handling
        // See: https://doc.rust-lang.org/stable/std/fs/index.html (TOCTOU section)
        if ignore_file.exists() {
            println!(
                "{}",
                Messages::found_ignore_file(&ignore_file.display().to_string())
            );
            println!("{}", Messages::applying_ignore_rules());
            builder.add(ignore_file);
        }
    }

    /// Adds CLI-provided exclusion patterns to the builder.
    fn add_cli_patterns(
        builder: &mut GitignoreBuilder,
        cli_patterns: &[String],
    ) -> anyhow::Result<()> {
        for pat in cli_patterns {
            builder.add_line(None, pat)?;
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
}
