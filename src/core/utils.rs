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

/// Convert bytes to human-readable format (B, KB, MB, GB)
pub fn format_bytes(bytes: usize) -> String {
    const UNITS: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];

    if bytes == 0 {
        return "0 B".to_string();
    }

    let base: f64 = 1024.0;
    let bytes_f64 = bytes as f64;
    let exponent = (bytes_f64.ln() / base.ln()).floor() as usize;
    let exponent = exponent.min(UNITS.len() - 1);

    let value = bytes_f64 / base.powi(exponent as i32);

    if exponent == 0 {
        format!("{} {}", bytes, UNITS[exponent])
    } else {
        format!("{:.1} {}", value, UNITS[exponent])
    }
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
