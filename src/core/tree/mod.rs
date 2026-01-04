//! tree - Handles directory tree rendering operations with support for merged paths.

use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;
use std::path::{Path, PathBuf};

/// Tracks tree rendering state across directory traversal.
pub struct TreeState {
    /// For each depth, whether that level was the last child
    ancestors_last: Vec<bool>,
}

impl TreeState {
    pub fn new() -> Self {
        Self {
            ancestors_last: Vec::new(),
        }
    }

    /// Writes a tree structure from a list of actual traversed file paths
    pub fn write_tree_from_paths<W: Write>(
        traversed_paths: &[PathBuf],
        root: &Path,
        file: &mut W,
        tree: &mut TreeState,
    ) -> anyhow::Result<()> {
        if traversed_paths.is_empty() {
            return Ok(());
        }

        // Build a set of all directories that contain traversed files
        let mut directory_tree: BTreeSet<PathBuf> = BTreeSet::new();

        for path in traversed_paths {
            // Get relative path from root
            let relative = if let Ok(rel) = path.strip_prefix(root) {
                rel.to_path_buf()
            } else {
                path.clone()
            };

            // Add all parent directories of this file
            let mut current = relative.clone();
            while let Some(parent) = current.parent() {
                if parent.as_os_str().is_empty() {
                    break;
                }
                directory_tree.insert(parent.to_path_buf());
                current = parent.to_path_buf();
            }

            // Add the file itself
            directory_tree.insert(relative);
        }

        // Build tree map from the directory set
        let tree_map = build_tree_map_from_set(&directory_tree);

        // Write the root
        writeln!(file, "{}", root.display())?;

        // Render the tree
        Self::write_tree_from_map(&tree_map, file, tree)?;

        Ok(())
    }

    /// Writes tree from a pre-built map structure
    fn write_tree_from_map<W: Write>(
        tree_map: &BTreeMap<String, TreeNode>,
        file: &mut W,
        tree: &mut TreeState,
    ) -> anyhow::Result<()> {
        let entries: Vec<_> = tree_map.iter().collect();

        for (idx, (name, node)) in entries.iter().enumerate() {
            let is_last = idx == entries.len() - 1;
            let prefix = tree.prefix();
            let connector = if is_last { "└── " } else { "├── " };

            writeln!(file, "{}{}{}", prefix, connector, name)?;

            if !node.children.is_empty() {
                tree.enter(is_last);
                Self::write_tree_from_map(&node.children, file, tree)?;
                tree.exit();
            }
        }

        Ok(())
    }

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

/// Represents a node in the tree structure
#[derive(Debug)]
struct TreeNode {
    children: BTreeMap<String, TreeNode>,
}

impl TreeNode {
    fn new() -> Self {
        Self {
            children: BTreeMap::new(),
        }
    }
}

/// Builds a tree map from a set of paths (only includes paths in the set)
fn build_tree_map_from_set(paths: &BTreeSet<PathBuf>) -> BTreeMap<String, TreeNode> {
    let mut root = BTreeMap::new();

    for path in paths {
        let components: Vec<_> = path.components().collect();

        if components.is_empty() {
            continue;
        }

        let mut current = &mut root;

        for component in components {
            let name = component.as_os_str().to_string_lossy().to_string();
            current.entry(name.clone()).or_insert_with(TreeNode::new);

            let node = current.get_mut(&name).unwrap();
            current = &mut node.children;
        }
    }

    root
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufWriter;

    #[test]
    fn test_build_tree_map_from_set() {
        let mut paths = BTreeSet::new();
        paths.insert(PathBuf::from("src/main.rs"));
        paths.insert(PathBuf::from("src"));

        let tree = build_tree_map_from_set(&paths);

        // Should have 'src' as root node
        assert!(tree.contains_key("src"));

        // 'src' should have 'main.rs' as child
        let src_node = tree.get("src").unwrap();
        assert!(src_node.children.contains_key("main.rs"));
    }

    #[test]
    fn test_write_tree_from_paths_excludes_untracked() -> anyhow::Result<()> {
        use tempfile::TempDir;

        let temp_dir = TempDir::new()?;
        let root = temp_dir.path();

        // Only track src/main.rs, not src/utils/helper.rs
        let tracked = vec![root.join("src/main.rs")];

        let mut output = BufWriter::new(Vec::new());
        let mut tree_state = TreeState::new();

        TreeState::write_tree_from_paths(&tracked, root, &mut output, &mut tree_state)?;

        let result = String::from_utf8(output.into_inner()?)?;

        // Should contain src and main.rs
        assert!(result.contains("src"));
        assert!(result.contains("main.rs"));

        // Should NOT contain utils or helper.rs
        assert!(!result.contains("utils"));
        assert!(!result.contains("helper.rs"));

        Ok(())
    }
}
