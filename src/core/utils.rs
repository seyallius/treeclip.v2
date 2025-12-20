//! utils - Provides utility functions for path validation and formatting.

use std::path::Path;

/// Validates that a path exists on the filesystem.
///
/// # Errors
///
/// Returns an error if the path does not exist.
pub fn validate_path_exists(path: &Path) -> anyhow::Result<()> {
    if !path.exists() {
        anyhow::bail!("Path does not exist: {}", path.display());
    }
    Ok(())
}

/// Formats a number with thousand separators for improved readability.
///
/// # Examples
///
/// ```
/// assert_eq!(format_number(1000), "1,000");
/// assert_eq!(format_number(1234567), "1,234,567");
/// ```
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

/// Converts bytes to human-readable format (B, KB, MB, GB, TB, PB).
///
/// # Examples
///
/// ```
/// assert_eq!(format_bytes(1024), "1.0 KB");
/// assert_eq!(format_bytes(1048576), "1.0 MB");
/// ```
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
    use super::*;
    use std::path::Path;
    use tempfile::TempDir;

    #[test]
    fn test_validate_path_exists_valid() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let result = validate_path_exists(temp_dir.path());
        assert!(result.is_ok());
        Ok(())
    }

    #[test]
    fn test_validate_path_exists_invalid() {
        let result = validate_path_exists(Path::new("/nonexistent/path"));
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("does not exist"));
    }

    #[test]
    fn test_format_number_small() {
        assert_eq!(format_number(0), "0");
        assert_eq!(format_number(123), "123");
        assert_eq!(format_number(999), "999");
    }

    #[test]
    fn test_format_number_with_separators() {
        assert_eq!(format_number(1_000), "1,000");
        assert_eq!(format_number(12_345), "12,345");
        assert_eq!(format_number(1_234_567), "1,234,567");
        assert_eq!(format_number(1_234_567_890), "1,234,567,890");
    }

    #[test]
    fn test_format_number_negative() {
        assert_eq!(format_number(-1_000), "-1,000");
        assert_eq!(format_number(-1_234_567), "-1,234,567");
    }

    #[test]
    fn test_format_bytes_zero() {
        assert_eq!(format_bytes(0), "0 B");
    }

    #[test]
    fn test_format_bytes_small() {
        assert_eq!(format_bytes(512), "512 B");
        assert_eq!(format_bytes(1023), "1023 B");
    }

    #[test]
    fn test_format_bytes_kilobytes() {
        assert_eq!(format_bytes(1_024), "1.0 KB");
        assert_eq!(format_bytes(2_048), "2.0 KB");
        assert_eq!(format_bytes(10_240), "10.0 KB");
    }

    #[test]
    fn test_format_bytes_megabytes() {
        assert_eq!(format_bytes(1_048_576), "1.0 MB");
        assert_eq!(format_bytes(5_242_880), "5.0 MB");
    }

    #[test]
    fn test_format_bytes_gigabytes() {
        assert_eq!(format_bytes(1_073_741_824), "1.0 GB");
        assert_eq!(format_bytes(5_368_709_120), "5.0 GB");
    }

    #[test]
    fn test_format_bytes_terabytes() {
        assert_eq!(format_bytes(1_099_511_627_776), "1.0 TB");
    }

    #[test]
    fn test_format_bytes_decimal_precision() {
        assert_eq!(format_bytes(1_536), "1.5 KB");
        assert_eq!(format_bytes(1_572_864), "1.5 MB");
    }
}
