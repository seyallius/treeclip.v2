//! errors - Custom error types for TreeClip using thiserror for better error handling.
//!
//! # Quick Reference Card - Error Handling Patterns (◕‿◕✿)
//!
//! ## Import Pattern
//!
//! ```rust
//! use crate::core::errors::{TreeClipError, ClipboardError, FileSystemError};
//! use anyhow::Context;
//! ```
//!
//! ## Common Patterns
//!
//! ### 1. File Read with Context
//!
//! ```rust
//! let content = fs::read_to_string(path)
//!     .map_err(|e| FileSystemError::ReadFailed {
//!         path: path.to_path_buf(),
//!         source: e,
//!     })
//!     .with_context(|| format!("Failed to read file: {}", path.display()))?;
//! ```
//!
//! ### 2. File Write with Context
//!
//! ```rust
//! fs::write(path, content)
//!     .map_err(|e| FileSystemError::WriteFailed {
//!         path: path.to_path_buf(),
//!         source: e,
//!     })
//!     .with_context(|| format!("Failed to write to: {}", path.display()))?;
//! ```
//!
//! ### 3. Path Validation
//!
//! ```rust
//! utils::validate_path_exists(path)
//!     .with_context(|| format!("Path validation failed: {}", path.display()))?;
//! ```
//!
//! ### 4. Clipboard Operations
//!
//! ```rust
//! clipboard
//!     .set_clipboard()
//!     .with_context(|| "Failed to copy content to clipboard")?;
//! ```
//!
//! ### 5. Editor Operations
//!
//! ```rust
//! editor::open(path)
//!     .with_context(|| format!("Failed to open editor for: {}", path.display()))?;
//! ```
//!
//! ### 6. Directory Traversal
//!
//! ```rust
//! for entry in walker {
//!     let entry = entry
//!         .map_err(|e| TraversalError::WalkFailed {
//!             path: root.clone(),
//!             source: e,
//!         })
//!         .with_context(|| format!("Failed to traverse: {}", root.display()))?;
//! }
//! ```
//!
//! ### 7. Pattern Matching
//!
//! ```rust
//! matcher
//!     .add_pattern(pattern)
//!     .map_err(|e| PatternError::InvalidPattern {
//!         pattern: pattern.clone(),
//!         source: e,
//!     })
//!     .with_context(|| format!("Invalid pattern: '{}'", pattern))?;
//! ```
//!
//! ## Function Signatures
//!
//! ### Library Functions (Return Custom Errors)
//!
//! ```rust
//! // For library code that might be reused
//! pub fn process() -> Result<String, MyError> {
//!     // ...
//! }
//! ```
//!
//! ### Application Functions (Return anyhow)
//!
//! ```rust
//! // For application-level code
//! pub fn execute(args: Args) -> anyhow::Result<()> {
//!     // ...
//! }
//! ```
//!
//! ## Creating New Error Types
//!
//! ```rust
//! #[derive(Error, Debug)]
//! pub enum MyError {
//!     // Simple variant
//!     #[error("Something failed: {0}")]
//!     SimpleFailed(String),
//!
//!     // With source
//!     #[error("Failed to process {path}")]
//!     ProcessFailed {
//!         path: PathBuf,
//!         #[source]
//!         source: std::io::Error,
//!     },
//!
//!     // With multiple fields
//!     #[error("Invalid config: {reason} at line {line}")]
//!     InvalidConfig {
//!         reason: String,
//!         line: usize,
//!     },
//! }
//! ```
//!
//! ## Error Conversion
//!
//! ```rust
//! // Manual conversion
//! let my_error = MyError::Failed("oops".into());
//! let anyhow_error: anyhow::Error = my_error.into();
//!
//! // Automatic with #[from]
//! #[derive(Error, Debug)]
//! pub enum MainError {
//!     #[error("Sub error: {0}")]
//!     Sub(#[from] SubError),  // Automatic From impl
//! }
//! ```
//!
//! ## Context Best Practices
//!
//! ### ✅ Good Context
//!
//! ```rust
//! .with_context(|| format!("Failed to read config from: {}", path.display()))?;
//! .with_context(|| "Failed to initialize clipboard - clipboard may not be available")?;
//! .with_context(|| format!("Invalid pattern #{}: '{}'", index, pattern))?;
//! ```
//!
//! ### ❌ Bad Context
//!
//! ```rust
//! .context("Error")?;  // Too generic
//! .context(format!("Error: {}", expensive()))?;  // Not lazy!
//! .with_context(|| "Failed")?;  // Still too generic
//! ```
//!
//! ## Testing Errors
//!
//! ```rust
//! fn test_error_handling() -> anyhow::Result<()> {
//!     let result = risky_operation();
//!
//!     assert!(result.is_err());
//!
//!     let error = result.unwrap_err();
//!     let error_msg = format!("{:?}", error);
//!
//!     // Check error chain
//!     assert!(error_msg.contains("specific error text"));
//!     assert!(error_msg.contains("context message"));
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Debugging Errors
//!
//! ### View Full Error Chain
//!
//! ```rust
//! match result {
//!     Err(e) => {
//!         eprintln!("Error: {:?}", e);  // Debug format shows full chain
//!         // Or
//!         eprintln!("Error: {:#}", e);  // Alternative format
//!     }
//!     Ok(_) => {}
//! }
//! ```
//!
//! ### Get Source Errors
//!
//! ```rust
//! use std::error::Error as _;
//!
//! let mut source = error.source();
//! while let Some(e) = source {
//!     eprintln!("Caused by: {}", e);
//!     source = e.source();
//! }
//! ```
//!
//! ## Common Scenarios
//!
//! ### Scenario 1: Function Chain
//!
//! ```rust
//! fn top_level() -> anyhow::Result<()> {
//!     middle_level()
//!         .with_context(|| "Top level context")?;
//!     Ok(())
//! }
//!
//! fn middle_level() -> anyhow::Result<()> {
//!     bottom_level()
//!         .map_err(|e| MyError::Wrapped { source: e })
//!         .with_context(|| "Middle level context")?;
//!     Ok(())
//! }
//!
//! fn bottom_level() -> Result<(), std::io::Error> {
//!     // ...
//! }
//! ```
//!
//! ### Scenario 2: Early Returns
//!
//! ```rust
//! fn process(path: &Path) -> anyhow::Result<String> {
//!     // Check 1
//!     if !path.exists() {
//!         return Err(FileSystemError::PathNotFound(path.to_path_buf()).into());
//!     }
//!
//!     // Check 2
//!     let content = fs::read_to_string(path)
//!         .with_context(|| format!("Failed to read: {}", path.display()))?;
//!
//!     // Check 3
//!     if content.is_empty() {
//!         anyhow::bail!("File is empty: {}", path.display());
//!     }
//!
//!     Ok(content)
//! }
//! ```
//!
//! ### Scenario 3: Multiple Operations
//!
//! ```rust
//! fn complex_operation() -> anyhow::Result<()> {
//!     // Operation 1
//!     let data = read_data()
//!         .with_context(|| "Failed to read input data")?;
//!
//!     // Operation 2
//!     let processed = process_data(data)
//!         .with_context(|| "Failed to process data")?;
//!
//!     // Operation 3
//!     write_data(processed)
//!         .with_context(|| "Failed to write output")?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Cheat Sheet
//!
//! | Goal | Pattern |
//! |------|---------|
//! | Convert error | `.map_err(\|e\| MyError { source: e })` |
//! | Add context | `.with_context(\|\| "message")` |
//! | Propagate | `?` operator |
//! | Create error | `Err(MyError::Variant { ... }.into())` |
//! | Bail early | `anyhow::bail!("message")` |
//! | Ensure condition | `anyhow::ensure!(cond, "message")` |
//!
//! ## Pro Tips 💡
//!
//! 1. **Always use `.with_context()` with closure**: `|| format!(...)`
//! 2. **Include relevant data in context**: paths, indices, values
//! 3. **Make error messages actionable**: what failed + why + where
//! 4. **Test error paths**: ensure errors have proper context
//! 5. **Use `#[source]` for error chains**: preserves causality

use std::path::PathBuf;
use thiserror::Error;

/// Main error type for TreeClip operations.
#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum TreeClipError {
    /// Error related to clipboard operations.
    #[error("Clipboard error: {0}")]
    Clipboard(#[from] ClipboardError),

    /// Error related to file system operations.
    #[error("File system error: {0}")]
    FileSystem(#[from] FileSystemError),

    /// Error related to traversal operations.
    #[error("Traversal error: {0}")]
    Traversal(#[from] TraversalError),

    /// Error related to editor operations.
    #[error("Editor error: {0}")]
    Editor(#[from] EditorError),

    /// Error related to pattern matching/exclusion.
    #[error("Pattern error: {0}")]
    Pattern(#[from] PatternError),

    /// Generic I/O error with context.
    #[error("I/O error: {message}")]
    Io {
        message: String,
        #[source]
        source: std::io::Error,
    },
}

/// Errors specific to clipboard operations.
#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum ClipboardError {
    #[error("Failed to initialize clipboard: {0}")]
    InitializationFailed(String),

    #[error("Failed to set clipboard content: {0}")]
    SetFailed(String),

    #[error("Failed to read file for clipboard: {path}")]
    ReadFailed {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Clipboard content too large: {size} bytes (max: {max} bytes)")]
    ContentTooLarge { size: usize, max: usize },
}

/// Errors specific to file system operations.
#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum FileSystemError {
    #[error("Path does not exist: {0}")]
    PathNotFound(PathBuf),

    #[error("Failed to canonicalize path: {path}")]
    CanonicalizeFailed {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to read file: {path}")]
    ReadFailed {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to write file: {path}")]
    WriteFailed {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to delete file: {path}")]
    DeleteFailed {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to create directory: {path}")]
    CreateDirFailed {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Permission denied: {path}")]
    PermissionDenied { path: PathBuf },
}

/// Errors specific to directory traversal.
#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum TraversalError {
    #[error("Failed to traverse directory: {path}")]
    WalkFailed {
        path: PathBuf,
        #[source]
        source: walkdir::Error,
    },

    #[error("Failed to access directory entry: {path}")]
    EntryAccessFailed { path: PathBuf },

    #[error("Failed to write output file: {path}")]
    OutputWriteFailed {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("No files found in directory: {0}")]
    NoFilesFound(PathBuf),
}

/// Errors specific to editor operations.
#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum EditorError {
    #[error("Failed to open editor for file: {path}")]
    OpenFailed {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Editor process failed with status: {status}")]
    ProcessFailed { status: std::process::ExitStatus },

    #[error("No suitable editor found (tried: {0})")]
    NoEditorFound(String),

    #[error("Failed to get EDITOR environment variable")]
    NoEditorEnvVar,
}

/// Errors specific to pattern matching and exclusion.
#[derive(Error, Debug)]
pub enum PatternError {
    #[error("Invalid exclusion pattern: {pattern}")]
    InvalidPattern {
        pattern: String,
        #[source]
        source: ignore::Error,
    },

    #[error("Failed to read ignore file: {path}")]
    #[allow(dead_code)]
    IgnoreFileReadFailed {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to build gitignore matcher")]
    BuildFailed {
        #[source]
        source: ignore::Error,
    },

    /// Returned when a glob input pattern is not valid glob syntax.
    #[error("Failed to build glob matcher for pattern: {pattern}")]
    GlobBuildFailed {
        pattern: String,
        #[source]
        source: ignore::Error,
    },

    /// Returned when a glob input pattern matches zero paths on disk.
    #[error("Glob pattern matched no files: {pattern}")]
    GlobNoMatches { pattern: String },
}

#[cfg(test)]
mod errors_tests {
    use super::*;
    use std::io;

    #[test]
    fn test_clipboard_error_display() {
        let err = ClipboardError::InitializationFailed("test error".to_string());
        assert!(err.to_string().contains("Failed to initialize clipboard"));
    }

    #[test]
    fn test_filesystem_error_display() {
        let path = PathBuf::from("/test/path");
        let err = FileSystemError::PathNotFound(path.clone());
        assert!(err.to_string().contains("/test/path"));
    }

    #[test]
    fn test_traversal_error_display() {
        let path = PathBuf::from("/test/dir");
        let err = TraversalError::NoFilesFound(path.clone());
        assert!(err.to_string().contains("No files found"));
    }

    #[test]
    fn test_editor_error_display() {
        let err = EditorError::NoEditorFound("vim, nano".to_string());
        assert!(err.to_string().contains("No suitable editor found"));
    }

    #[test]
    fn test_pattern_error_display() {
        let pattern = "**invalid**";
        let ignore_err = ignore::Error::Glob {
            glob: Some(pattern.to_string()),
            err: "test".to_string(),
        };
        let err = PatternError::InvalidPattern {
            pattern: pattern.to_string(),
            source: ignore_err,
        };
        assert!(err.to_string().contains("Invalid exclusion pattern"));
    }

    #[test]
    fn test_glob_no_matches_error_display() {
        let err = PatternError::GlobNoMatches {
            pattern: "object/*.go".to_string(),
        };
        assert!(err.to_string().contains("matched no files"));
        assert!(err.to_string().contains("object/*.go"));
    }

    #[test]
    fn test_glob_build_failed_error_display() {
        let pattern = "[invalid";
        let ignore_err = ignore::Error::Glob {
            glob: Some(pattern.to_string()),
            err: "unclosed character class".to_string(),
        };
        let err = PatternError::GlobBuildFailed {
            pattern: pattern.to_string(),
            source: ignore_err,
        };
        assert!(err.to_string().contains("Failed to build glob matcher"));
    }

    #[test]
    fn test_error_chain() {
        let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "access denied");
        let fs_err = FileSystemError::ReadFailed {
            path: PathBuf::from("/test"),
            source: io_err,
        };
        let main_err = TreeClipError::FileSystem(fs_err);

        let err_string = main_err.to_string();
        assert!(err_string.contains("File system error"));
        assert!(err_string.contains("Failed to read file"));
    }
}
