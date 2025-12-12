pub fn is_hidden(entry: &walkdir::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|str| {
            let hidden_entry = str.starts_with(".");
            if hidden_entry {
                println!("Hidden entry '{}' was skipped", entry.path().display());
            }
            hidden_entry
        })
        .unwrap_or(false)
}

#[cfg(test)]
mod filter_tests {
    use crate::core::traversal::filter::is_hidden;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_is_hidden() {
        // Create a mock entry
        let temp_dir = TempDir::new().unwrap();
        let hidden_file = temp_dir.path().join(".hidden");
        fs::write(&hidden_file, "").unwrap();

        let entry = walkdir::WalkDir::new(temp_dir.path())
            .into_iter()
            .next()
            .unwrap()
            .unwrap();

        assert!(is_hidden(&entry));
    }
}
