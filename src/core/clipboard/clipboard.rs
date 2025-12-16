use anyhow::Context;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

pub struct Clipboard {
    /// data to put onto clipboard
    data: PathBuf,

    clip: arboard::Clipboard,
}

impl Clipboard {
    pub fn new(data: &PathBuf) -> anyhow::Result<Self> {
        Ok(Self {
            data: data.clone(),
            clip: arboard::Clipboard::new()
                .with_context(|| "failed to create clipboard instance")?,
        })
    }
}

impl Clipboard {
    /// Reads the output file and places its contents into the system clipboard.
    ///
    /// ## Platform notes
    ///
    /// - On Windows and macOS, clipboard contents persist after the program exits.
    /// - On Linux, clipboard persistence depends on a running clipboard service
    ///   (e.g. GNOME/KDE clipboard, CopyQ, wl-clipboard).
    ///
    /// This function follows standard CLI behavior: it sets the clipboard and
    /// exits immediately. On most desktop environments this works out of the box.
    /// On minimal window managers without a clipboard manager, the clipboard
    /// contents may not persist after exit.
    ///
    /// ## Errors
    ///
    /// Returns an error if the file cannot be read or if the clipboard cannot be
    /// accessed.
    pub fn set_clipboard(&mut self) -> anyhow::Result<()> {
        // Read the entire file into memory.
        // Clipboard APIs require owning the full contents as a string.
        let mut output_file = File::options().read(true).open(&self.data)?;
        let mut output_content = String::new();
        output_file.read_to_string(&mut output_content)?; //TODO: bad reading the full content into memory? what if it's huge?

        // Create a clipboard handle and set the text.
        // On Linux, clipboard managers usually take ownership immediately.
        self.clip
            .set()
            .text(output_content)
            .with_context(|| "failed to set output content in clipboard")?;
        // Sleep for a little while just to guarantee clipboard ownership.
        // This is intentional otherwise the warning of arboard.
        thread::sleep(Duration::from_millis(100));

        Ok(())
    }
}
