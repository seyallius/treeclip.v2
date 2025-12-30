//! tree - Handles directory tree rendering operations.
//!
//! Based on what ChatGPT said (cuz of course I'm not some genius lol),
//! the overall logic is like this for tree printing in a dir structure:
//!
//! 1. Enter directory
//! 2. List children
//! 3. For each child:
//!     - Is it last? (lookahead)
//!     - Print prefix
//!     - If directory:
//!         - Remember whether it was last
//!         - Recurse
//!         - Forget it
//! ## ✏️ Let’s start with a thought experiment (no code)
//!
//! Imagine this tree:
//!
//! ```
//! root
//! ├── a
//! │   ├── x
//! │   └── y
//! └── b
//! ```
//!
//! Now answer **without coding**:
//!
//! ### Question
//!
//! When printing `y`, why do we NOT print `│` for `root`?
//!
//! 👉 Because `root` was the **last sibling** at its level.
//!
//! So already we know:
//!
//! > **Indentation depends on ancestor state**, not current node.
//!
//! That’s the key insight.
use std::fs::File;
use std::io::Write;
use std::path::Path;
use walkdir::WalkDir;

/// Tracks tree rendering state across directory traversal.
pub struct TreeState {
    /// For each depth, whether that level was the last child
    ancestors_last: Vec<bool>,
}

// ------------------------------------------------- Public Functions -------------------------------------------------

impl TreeState {
    pub fn new() -> Self {
        Self {
            ancestors_last: Vec::new(),
        }
    }

    pub fn write_tree(dir: &Path, file: &mut File, tree: &mut TreeState) -> anyhow::Result<()> {
        let mut entries: Vec<_> = WalkDir::new(dir)
            .max_depth(1)
            .min_depth(1)
            .into_iter()
            .filter_map(Result::ok)
            .collect();

        entries.sort_by_key(|e| e.path().to_path_buf());

        let mut iter = entries.into_iter().peekable();

        while let Some(entry) = iter.next() {
            let is_last = iter.peek().is_none();
            let path = entry.path();

            let prefix = tree.prefix();
            let connector = if is_last { "└── " } else { "├── " };

            writeln!(
                file,
                "{}{}{}",
                prefix,
                connector,
                path.file_name().unwrap().to_string_lossy()
            )?;

            if path.is_dir() {
                tree.enter(is_last);
                Self::write_tree(path, file, tree)?;
                tree.exit();
            }
        }

        Ok(())
    }
}

// ------------------------------------------------- Private Functions -------------------------------------------------

impl TreeState {
    /// Builds the tree prefix (`│   `, `    `, etc.)
    fn prefix(&self) -> String {
        self.ancestors_last
            .iter()
            .map(|is_last| if *is_last { "    " } else { "│   " })
            .collect()
    }

    /// Push a new depth level
    fn enter(&mut self, is_last: bool) {
        self.ancestors_last.push(is_last);
    }

    /// Leave the current depth level
    fn exit(&mut self) {
        self.ancestors_last.pop();
    }
}
