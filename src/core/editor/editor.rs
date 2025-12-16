// Source Inspired - https://stackoverflow.com/a/56012454
// Posted by Peter Varo, modified by community. See post 'Timeline' for change history
// Retrieved 2025-12-16, License - CC BY-SA 4.0

use std::path::PathBuf;
use std::{env, process};

/// Opens the file in the default text editor
/// If the default editor is not found, uses nano as fallback.
pub fn open(path: &PathBuf) -> anyhow::Result<()> {
    let arg = if cfg!(windows) {
        "start"
    } else if cfg!(unix) {
        "xdg-open"
    } else if cfg!(target_os = "macos") {
        "open"
    } else {
        ""
    };

    match process::Command::new(arg.to_string()).args(path.canonicalize()).status() {
        Ok(status) => {
            assert!(status.success());
        }
        Err(err) => {
            eprintln!("Error opening file: {err}. Will attempt to use default CLI text editor...");
            let default_cli_editor = env::var("EDITOR").unwrap_or_else(|err| {
                eprintln!("Error reading env variable: {err}");
                "/bin/nano".to_string()
            });

            let status = process::Command::new(default_cli_editor)
                .args(path)
                .status()?;
            println!("Editor process status: {status}");
            assert!(status.success());
        }
    }

    Ok(())
}
