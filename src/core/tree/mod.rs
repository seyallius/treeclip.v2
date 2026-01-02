//! tree - Handles directory tree rendering operations with support for merged paths.

use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

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

    /// Writes a unified tree structure for multiple input paths
    pub fn write_unified_tree(
        inputs: &[PathBuf],
        file: &mut File,
        tree: &mut TreeState,
    ) -> anyhow::Result<()> {
        if inputs.is_empty() {
            return Ok(());
        }

        // If single input, use simple approach
        if inputs.len() == 1 {
            writeln!(file, "{}", inputs[0].display())?;
            return Self::write_tree(&inputs[0], file, tree);
        }

        // Find common ancestor for multiple inputs
        let common_ancestor = find_common_ancestor(inputs);

        // Build a tree structure from all inputs
        let tree_map = build_tree_map(inputs, &common_ancestor);

        // Write the common root
        writeln!(file, "{}", common_ancestor.display())?;

        // Render the merged tree
        Self::write_tree_from_map(&tree_map, file, tree)?;

        Ok(())
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

/// Finds the common ancestor path for multiple input paths
fn find_common_ancestor(paths: &[PathBuf]) -> PathBuf {
    if paths.is_empty() {
        return PathBuf::from(".");
    }

    if paths.len() == 1 {
        return paths[0].clone();
    }

    // Canonicalize all paths first (or use as-is if canonicalization fails)
    let canonical_paths: Vec<PathBuf> = paths
        .iter()
        .map(|p| p.canonicalize().unwrap_or_else(|_| p.clone()))
        .collect();

    // Split paths into components
    let components: Vec<Vec<_>> = canonical_paths
        .iter()
        .map(|p| p.components().collect())
        .collect();

    if components.is_empty() {
        return PathBuf::from(".");
    }

    // Find common prefix
    let mut common = PathBuf::new();
    let min_len = components.iter().map(|c| c.len()).min().unwrap_or(0);

    for i in 0..min_len {
        let component = &components[0][i];

        // Check if all paths have the same component at this position
        if components.iter().all(|c| &c[i] == component) {
            common.push(component);
        } else {
            break;
        }
    }

    // If no common ancestor found, return "."
    if common.as_os_str().is_empty() {
        PathBuf::from(".")
    } else {
        common
    }
}

/// Builds a tree map structure from input paths relative to common ancestor
fn build_tree_map(inputs: &[PathBuf], common_ancestor: &Path) -> BTreeMap<String, TreeNode> {
    let mut root = BTreeMap::new();

    for input in inputs {
        // Get path relative to common ancestor
        let relative = if let Ok(rel) = input.strip_prefix(common_ancestor) {
            rel.to_path_buf()
        } else {
            input.clone()
        };

        // Walk the directory structure
        for entry in WalkDir::new(input)
            .min_depth(1)
            .into_iter()
            .filter_map(Result::ok)
        {
            let entry_path = entry.path();

            // Get relative path from common ancestor
            let relative_entry = if let Ok(rel) = entry_path.strip_prefix(common_ancestor) {
                rel.to_path_buf()
            } else if let Ok(rel) = entry_path.strip_prefix(input) {
                relative.join(rel)
            } else {
                continue;
            };

            // Insert into tree structure
            insert_into_tree(&mut root, &relative_entry);
        }
    }

    root
}

/// Inserts a path into the tree structure
fn insert_into_tree(root: &mut BTreeMap<String, TreeNode>, path: &Path) {
    let components: Vec<_> = path.components().collect();

    if components.is_empty() {
        return;
    }

    let mut current = root;

    for component in components {
        let name = component.as_os_str().to_string_lossy().to_string();
        current.entry(name).or_insert_with(TreeNode::new);

        let node = current
            .get_mut(&component.as_os_str().to_string_lossy().to_string())
            .unwrap();
        current = &mut node.children;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufWriter;

    #[test]
    fn test_find_common_ancestor_same_parent() {
        let paths = vec![
            PathBuf::from("pkg/auth/pipeline/event"),
            PathBuf::from("pkg/eventhandler"),
        ];

        let common = find_common_ancestor(&paths);
        assert_eq!(common, PathBuf::from("pkg"));
    }

    #[test]
    fn test_find_common_ancestor_no_common() {
        let paths = vec![
            PathBuf::from("/home/user/project"),
            PathBuf::from("/var/log"),
        ];

        let common = find_common_ancestor(&paths);
        // Should find root or a common ancestor
        assert!(common.components().count() > 0);
    }

    #[test]
    fn test_find_common_ancestor_single_path() {
        let paths = vec![PathBuf::from("src/main.rs")];
        let common = find_common_ancestor(&paths);
        assert_eq!(common, PathBuf::from("src/main.rs"));
    }

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
