//! filter - Provides filtering functions for directory traversal operations.

/// Checks if a directory entry is hidden (starts with a dot).
///
/// # Arguments
///
/// * `entry` - The directory entry to check
/// * `verbose` - If true, logs hidden entries to stdout
///
/// # Returns
///
/// Returns `true` if the entry is hidden, `false` otherwise.
pub fn is_hidden(entry: &walkdir::DirEntry, verbose: bool) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|str| {
            let hidden_entry = str.starts_with('.');
            if hidden_entry && verbose {
                println!("Hidden entry '{}' was skipped", entry.path().display());
            }
            hidden_entry
        })
        .unwrap_or(false)
}

#[cfg(test)]
mod filter_tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;
    use walkdir::WalkDir;

    #[test]
    fn test_is_hidden_with_hidden_file() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let hidden_file = temp_dir.path().join(".hidden");
        fs::write(&hidden_file, "")?;

        let mut walker = WalkDir::new(temp_dir.path()).into_iter();

        // Skip root directory
        walker.next();

        if let Some(Ok(entry)) = walker.next() {
            assert!(is_hidden(&entry, false));
        }

        Ok(())
    }

    #[test]
    fn test_is_hidden_with_regular_file() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let regular_file = temp_dir.path().join("visible.txt");
        fs::write(&regular_file, "")?;

        let mut walker = WalkDir::new(temp_dir.path()).into_iter();

        // Skip root directory
        walker.next();

        if let Some(Ok(entry)) = walker.next() {
            assert!(!is_hidden(&entry, false));
        }

        Ok(())
    }

    #[test]
    fn test_is_hidden_with_hidden_directory() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let hidden_dir = temp_dir.path().join(".git");
        fs::create_dir(&hidden_dir)?;

        let mut walker = WalkDir::new(temp_dir.path()).into_iter();

        // Skip root directory
        walker.next();

        if let Some(Ok(entry)) = walker.next() {
            assert!(is_hidden(&entry, false));
        }

        Ok(())
    }

    #[test]
    fn test_is_hidden_verbose_mode() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let hidden_file = temp_dir.path().join(".hidden");
        fs::write(&hidden_file, "")?;

        let mut walker = WalkDir::new(temp_dir.path()).into_iter();

        // Skip root directory
        walker.next();

        if let Some(Ok(entry)) = walker.next() {
            // Should print message in verbose mode
            assert!(is_hidden(&entry, true));
        }

        Ok(())
    }
}
