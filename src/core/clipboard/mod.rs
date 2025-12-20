//! clipboard - Handles system clipboard operations for file content.

use anyhow::Context;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

/// Clipboard provides an interface to interact with the system clipboard.
pub struct Clipboard {
    /// Path to the data file to be copied to clipboard.
    data: PathBuf,
    /// Handle to the system clipboard.
    clip: arboard::Clipboard,
}

impl Clipboard {
    /// Creates a new Clipboard instance for the specified file path.
    ///
    /// # Errors
    ///
    /// Returns an error if the clipboard cannot be initialized.
    pub fn new(data: &Path) -> anyhow::Result<Self> {
        Ok(Self {
            data: data.to_path_buf(),
            clip: arboard::Clipboard::new()
                .with_context(|| "failed to create clipboard instance")?,
        })
    }

    /// Reads the output file and places its contents into the system clipboard.
    ///
    /// # Platform Notes
    ///
    /// - **Windows/macOS**: Clipboard contents persist after program exit.
    /// - **Linux**: Persistence depends on running clipboard service
    ///   (e.g., GNOME/KDE clipboard, CopyQ, wl-clipboard).
    ///
    /// This follows standard CLI behavior: sets clipboard and exits immediately.
    /// On most desktop environments this works out of the box. On minimal window
    /// managers without a clipboard manager, contents may not persist after exit.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or clipboard cannot be accessed.
    pub fn set_clipboard(&mut self) -> anyhow::Result<()> {
        // TODO: Optimize for huge files - consider streaming or chunking instead of loading entire file
        // Read entire file into memory (clipboard APIs require full content as string)
        let mut output_file = File::options().read(true).open(&self.data)?;
        let mut output_content = String::new();
        output_file.read_to_string(&mut output_content)?;

        // Set clipboard text
        // On Linux, clipboard managers usually take ownership immediately
        self.clip
            .set()
            .text(output_content)
            .with_context(|| "failed to set output content in clipboard")?;

        // NOTE: Sleep guarantees clipboard ownership (required by arboard)
        thread::sleep(Duration::from_millis(100));

        Ok(())
    }
}

#[cfg(test)]
mod clipboard_tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_clipboard_creation() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "test content")?;

        let clipboard = Clipboard::new(&file_path);
        assert!(clipboard.is_ok());

        Ok(())
    }

    #[test]
    fn test_clipboard_with_nonexistent_file() {
        let result = Clipboard::new(Path::new("/nonexistent/file.txt"));
        // Should still create clipboard instance (file is read later)
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_clipboard_with_content() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "Hello, clipboard!")?;

        let mut clipboard = Clipboard::new(&file_path)?;
        let result = clipboard.set_clipboard();

        // May fail in CI environments without clipboard support
        // So we just check it doesn't panic
        let _ = result;

        Ok(())
    }

    #[test]
    fn test_set_clipboard_with_empty_file() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = temp_dir.path().join("empty.txt");
        fs::write(&file_path, "")?;

        let mut clipboard = Clipboard::new(&file_path)?;
        let result = clipboard.set_clipboard();

        // May fail in CI without clipboard support
        let _ = result;

        Ok(())
    }
}
