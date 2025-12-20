//! editor - Provides functionality to open and delete files using system editors.
//!
//! Source Inspired - https://stackoverflow.com/a/56012454
//! Posted by Peter Varo, modified by community. See post 'Timeline' for change history.
//! Retrieved 2025-12-16, License - CC BY-SA 4.0

use std::path::Path;
use std::{env, fs, process};

/// Opens the file in the system's default text editor.
///
/// Falls back to nano if the default editor is not found.
///
/// # Platform-specific behavior
///
/// - **Windows**: Uses `start` command
/// - **macOS**: Uses `open` command
/// - **Unix/Linux**: Uses `xdg-open` command
///
/// If the graphical editor fails, attempts to use the CLI editor specified
/// in the `EDITOR` environment variable, or `/bin/nano` as final fallback.
///
/// # Errors
///
/// Returns an error if neither the default editor nor the fallback editor can be executed.
pub fn open(path: &Path) -> anyhow::Result<()> {
    let command = get_platform_open_command();

    match process::Command::new(command)
        .arg(path.canonicalize()?)
        .status()
    {
        Ok(status) if status.success() => Ok(()),
        Ok(_) | Err(_) => {
            eprintln!("Error opening file with default editor. Attempting CLI editor...");
            open_with_cli_editor(path)
        }
    }
}

/// Deletes the specified file from the filesystem.
///
/// # Note
///
/// There is no guarantee that the file is immediately deleted. Depending on
/// platform and open file descriptors, removal may be delayed.
///
/// # Errors
///
/// Returns an error if the file cannot be deleted.
pub fn delete(path: &Path) -> anyhow::Result<()> {
    fs::remove_file(path)?;
    Ok(())
}

// -------------------------------------------- Private Helper Functions --------------------------------------------

/// Returns the platform-specific command for opening files.
fn get_platform_open_command() -> &'static str {
    if cfg!(windows) {
        "start"
    } else if cfg!(target_os = "macos") {
        "open"
    } else if cfg!(unix) {
        "xdg-open"
    } else {
        ""
    }
}

/// Opens the file using a CLI text editor.
fn open_with_cli_editor(path: &Path) -> anyhow::Result<()> {
    let default_cli_editor = env::var("EDITOR").unwrap_or_else(|err| {
        eprintln!("Error reading EDITOR environment variable: {err}");
        "/bin/nano".to_string()
    });

    let status = process::Command::new(default_cli_editor)
        .arg(path)
        .status()?;

    if !status.success() {
        anyhow::bail!("Editor process failed with status: {status}");
    }

    Ok(())
}

#[cfg(test)]
mod editor_tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_get_platform_open_command() {
        let command = get_platform_open_command();
        assert!(!command.is_empty() || cfg!(not(any(windows, unix, target_os = "macos"))));
    }

    #[test]
    fn test_delete_file() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "test content")?;

        assert!(file_path.exists());

        delete(&file_path)?;

        assert!(!file_path.exists());

        Ok(())
    }

    #[test]
    fn test_delete_nonexistent_file() {
        let result = delete(Path::new("/nonexistent/file.txt"));
        assert!(result.is_err());
    }

    #[test]
    fn test_open_with_nonexistent_file() {
        let result = open(Path::new("/nonexistent/file.txt"));
        // This will fail because canonicalize fails on non-existent paths
        assert!(result.is_err());
    }
}
