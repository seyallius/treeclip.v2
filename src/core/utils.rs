use std::path::Path;

pub fn validate_path_exists(path: &Path) -> anyhow::Result<()> {
    if !path.exists() {
        anyhow::bail!("Path does not exist: {}", path.display());
    }
    Ok(())
}

/// adds a thousand separators to make large numbers more readable
pub fn format_number(n: i64) -> String {
    let s = n.to_string();
    if s.len() <= 3 {
        return s;
    }

    let mut result = String::new();
    for (i, char) in s.chars().enumerate() {
        if i > 0 && ((s.len() - i) % 3 == 0) {
            result.push(',');
        }
        result.push(char);
    }

    result
}

#[cfg(test)]
mod utils_tests {
    use crate::core::utils::validate_path_exists;
    use std::path::Path;
    use tempfile::TempDir;

    #[test]
    fn test_validate_path_exists_valid() {
        let temp_dir = TempDir::new().unwrap();
        let result = validate_path_exists(temp_dir.path());
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_path_exists_invalid() {
        let result = validate_path_exists(Path::new("/nonexistent/path"));
        assert!(result.is_err());
    }
}
