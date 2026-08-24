//! file_tree - File tree model for the TUI.
//!
//! This module owns *no* rendering or input logic. It is a pure data structure that:
//!
//! 1. Walks the filesystem once at construction time, building a flat list of
//!    `(path, depth, is_dir)` entries.
//! 2. Holds per-node selection / exclusion / expansion state.
//! 3. Provides cascade-aware mutators: toggling a directory selects/excludes
//!    every descendant, but the user can still re-toggle an individual
//!    descendant afterwards (so manual overrides win).
//! 4. Provides pure readers (`visible_indices`, `effectively_included_paths`)
//!    that the renderer and the bundle-construction code use.
//!
//! The struct mutates itself via `&mut self` methods - that's idiomatic Rust
//! for a value type. The mutation is driven by the stateless event layer
//! (`event::handle` returns an `Action`, `App::apply` calls these methods).

use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

/// Maximum number of directory entries to read in a single dir, to keep the
/// TUI snappy on huge directories. The user can still reach deep files via
/// glob input (which uses the underlying walkdir engine, not this cap).
const MAX_ENTRIES_PER_DIR: usize = 5_000;

/// One flat node in the file tree.
#[derive(Debug, Clone)]
pub struct FileNode {
    /// Absolute filesystem path.
    pub path: PathBuf,
    /// Depth from the root (`root` itself is depth 0).
    pub depth: usize,
    /// Directory or file.
    pub is_dir: bool,
    /// Expanded (only meaningful for directories). Defaults to `true` for
    /// shallow trees; controlled by `collapse_all` / `expand_all` and the
    /// `←`/`→` keys.
    pub expanded: bool,
    /// User has explicitly opted this node INTO the bundle. Default `true`
    /// for everything (matching TreeClip's default "include all" behavior).
    pub selected: bool,
    /// User has explicitly opted this node OUT of the bundle. Default `false`.
    /// Exclusion wins over selection (matches `.gitignore` semantics).
    pub excluded: bool,
    /// A glob pattern matched this node during a `apply_glob_*` operation.
    /// Purely visual; does not affect bundling.
    pub matched_glob: bool,
}

/// Result of a glob operation - used by the renderer to show counts in the
/// status bar.
#[derive(Debug, Clone, Copy)]
pub struct GlobStats {
    pub matched: usize,
    pub already_excluded: usize,
}

/// The whole tree, rooted at `root`, stored as a flat ordered list.
#[derive(Debug, Clone)]
pub struct FileTree {
    pub root: PathBuf,
    pub nodes: Vec<FileNode>,
}
impl FileTree {
    /// Builds a `FileTree` rooted at `root`. The root itself is included as
    /// the first node (depth 0). Hidden entries (starting with `.`) are
    /// skipped at this stage - the TUI shows the user-visible tree, and
    /// `--skip-hidden` will be set in `RunArgs` when we hand off.
    pub fn build(root: &Path) -> std::io::Result<Self> {
        let mut nodes: Vec<FileNode> = Vec::new();
        // The root itself.
        nodes.push(FileNode {
            path: root.to_path_buf(),
            depth: 0,
            is_dir: root.is_dir(),
            expanded: true,
            selected: true,
            excluded: false,
            matched_glob: false,
        });

        if root.is_dir() {
            Self::build_recursive(root, 1, &mut nodes)?;
        }

        Ok(Self {
            root: root.to_path_buf(),
            nodes,
        })
    }

    fn build_recursive(dir: &Path, depth: usize, out: &mut Vec<FileNode>) -> std::io::Result<()> {
        let read_dir = match fs::read_dir(dir) {
            Ok(rd) => rd,
            // Permission errors and the like are not fatal - just skip.
            Err(_) => return Ok(()),
        };

        // Collect & sort for stable ordering: directories first, then files.
        // Within each group, alphabetical.
        let mut entries: Vec<_> = read_dir
            .take(MAX_ENTRIES_PER_DIR)
            .filter_map(Result::ok)
            .collect();
        entries.sort_by(|a, b| {
            let ad = a.file_type().map(|t| t.is_dir()).unwrap_or(false);
            let bd = b.file_type().map(|t| t.is_dir()).unwrap_or(false);
            match (ad, bd) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.file_name().cmp(&b.file_name()),
            }
        });

        for entry in entries {
            let path = entry.path();
            let name = entry.file_name();
            let name_str = name.to_string_lossy();

            // Skip hidden entries at the UI level.
            if name_str.starts_with('.') {
                continue;
            }

            let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);

            out.push(FileNode {
                path: path.clone(),
                depth,
                is_dir,
                // Default expanded for shallow trees; the user collapses at will.
                expanded: depth <= 1,
                selected: true,
                excluded: false,
                matched_glob: false,
            });

            if is_dir {
                Self::build_recursive(&path, depth + 1, out)?;
            }
        }

        Ok(())
    }

    /// Returns the indices of nodes that should be visible given current
    /// expansion state. A node is visible iff every ancestor (nodes between
    /// the root and this node) is expanded.
    pub fn visible_indices(&self) -> Vec<usize> {
        let mut out = Vec::with_capacity(self.nodes.len());
        // `ancestor_collapsed` tracks depths where an ancestor is collapsed.
        // When we see a node whose depth is greater than a collapsed-ancestor
        // depth, it's hidden.
        let mut collapsed_depths: Vec<usize> = Vec::new();

        for (i, node) in self.nodes.iter().enumerate() {
            // Pop collapsed-ancestor depths that are no longer relevant
            // (i.e. we've moved out of that subtree).
            collapsed_depths.retain(|&d| d < node.depth);

            if collapsed_depths.is_empty() {
                out.push(i);
            }

            if node.is_dir && !node.expanded {
                collapsed_depths.push(node.depth);
            }
        }

        out
    }

    /// Returns `true` iff `node[i]` is effectively included in the bundle:
    /// neither this node nor any ancestor is excluded, AND this node is
    /// selected. (Default: included.)
    pub fn is_effectively_included(&self, i: usize) -> bool {
        // Walk up: any excluded ancestor => excluded.
        // Selection cascade: this node's `selected` flag reflects the latest
        // toggle of itself or its nearest toggled ancestor.
        let node = &self.nodes[i];
        if node.excluded {
            return false;
        }
        // Walk ancestors to check exclusion.
        if let Some(parent) = self.parent_of(i) {
            if !self.is_ancestor_chain_included(parent) {
                return false;
            }
        }
        // Now apply this node's own selection flag (which has been cascaded
        // from any ancestor that was toggled).
        node.selected
    }

    /// Internal helper: returns `false` if `i` itself or any of its
    /// ancestors is excluded. Used after `node.excluded` has already been
    /// checked for the leaf node — this catches cascaded exclusion via
    /// parent/grandparent (e.g. when a glob exclude matched a directory
    /// without cascading to all descendants).
    fn is_ancestor_chain_included(&self, i: usize) -> bool {
        let mut cursor = i;
        loop {
            if self.nodes[cursor].excluded {
                return false;
            }
            match self.parent_of(cursor) {
                Some(p) => cursor = p,
                None => return true,
            }
        }
    }

    /// Returns the index of the parent of node `i`, or `None` if `i` is the
    /// root. The parent is the largest `j < i` such that `nodes[j].depth <
    /// nodes[i].depth`.
    pub fn parent_of(&self, i: usize) -> Option<usize> {
        if i == 0 {
            return None;
        }
        let target_depth = self.nodes[i].depth;
        if target_depth == 0 {
            return None;
        }
        let mut j = i;
        while j > 0 {
            j -= 1;
            let d = self.nodes[j].depth;
            if d < target_depth {
                return Some(j);
            }
        }
        None
    }

    /// Returns the half-open range `[i+1, end)` of descendants of `i`
    /// (i.e. nodes that come after `i` and have depth strictly greater than
    /// `nodes[i].depth`).
    fn descendant_range(&self, i: usize) -> std::ops::Range<usize> {
        let my_depth = self.nodes[i].depth;
        let mut end = i + 1;
        while end < self.nodes.len() && self.nodes[end].depth > my_depth {
            end += 1;
        }
        (i + 1)..end
    }

    /// Returns the absolute paths of all effectively-included files, in tree
    /// order. This is the list the TUI hands to `run::execute` as
    /// `args.input_paths`.
    ///
    /// Returns empty when the user has deselected everything (or excluded the
    /// root). The TUI loop in `mod.rs` decides what to pass to the walker
    /// based on whether the root itself was excluded (so the walker can
    /// surface a `NoFilesFound` error rather than silently writing an empty
    /// bundle).
    pub fn effectively_included_paths(&self) -> Vec<PathBuf> {
        if self.is_root_excluded() {
            return Vec::new();
        }

        let mut out = Vec::new();
        for (i, node) in self.nodes.iter().enumerate() {
            if node.is_dir {
                continue;
            }
            if self.is_effectively_included(i) {
                out.push(node.path.clone());
            }
        }

        out
    }

    /// Returns `true` iff the root node (the first node) is explicitly
    /// excluded. Used by `build_run_args` to decide whether to pass `[root]`
    /// to the walker (so it can surface `NoFilesFound`) or `[]`.
    pub fn is_root_excluded(&self) -> bool {
        self.nodes.first().is_some_and(|n| n.excluded)
    }

    /// Counts of selected / excluded files (not directories) - shown in the
    /// options panel as "Selected: N files" / "Excluded: N files".
    pub fn file_counts(&self) -> (usize, usize) {
        let mut selected = 0usize;
        let mut excluded = 0usize;
        for (i, node) in self.nodes.iter().enumerate() {
            if node.is_dir {
                continue;
            }
            if node.excluded || !self.is_ancestor_chain_included(i) {
                excluded += 1;
            } else if self.is_effectively_included(i) {
                selected += 1;
            }
        }
        (selected, excluded)
    }

    /// Toggles the expanded state of a directory.
    pub fn toggle_expand(&mut self, i: usize) {
        if !self.nodes[i].is_dir {
            return;
        }
        self.nodes[i].expanded = !self.nodes[i].expanded;
    }

    /// Sets `expanded` on a directory.
    pub fn set_expand(&mut self, i: usize, expanded: bool) {
        if !self.nodes[i].is_dir {
            return;
        }
        self.nodes[i].expanded = expanded;
    }

    /// Toggles selection on a node. For directories, the change cascades to
    /// every descendant (matching how file-tree selectors work in most
    /// editors), but a descendant's existing manual override (selected !=
    /// new value) is just overwritten - the user can re-toggle individual
    /// files afterwards.
    pub fn toggle_select(&mut self, i: usize) {
        let new = !self.nodes[i].selected;
        self.nodes[i].selected = new;
        if self.nodes[i].is_dir {
            for d in self.descendant_range(i) {
                self.nodes[d].selected = new;
            }
        }
    }

    /// Toggles exclusion on a node. Exclusion always wins over selection.
    /// Cascades to descendants just like `toggle_select`.
    pub fn toggle_exclude(&mut self, i: usize) {
        let new = !self.nodes[i].excluded;
        self.nodes[i].excluded = new;
        if self.nodes[i].is_dir {
            for d in self.descendant_range(i) {
                self.nodes[d].excluded = new;
            }
        }
    }

    /// Marks all nodes as selected and not excluded (resets to TreeClip's
    /// default "include all" state).
    pub fn select_all(&mut self) {
        for n in &mut self.nodes {
            n.selected = true;
            n.excluded = false;
            n.matched_glob = false;
        }
    }

    /// Marks all nodes as unselected.
    pub fn select_none(&mut self) {
        for n in &mut self.nodes {
            n.selected = false;
            n.matched_glob = false;
        }
    }

    /// Collapses every directory.
    pub fn collapse_all(&mut self) {
        for n in &mut self.nodes {
            if n.is_dir {
                n.expanded = false;
            }
        }
    }

    /// Expands every directory.
    pub fn expand_all(&mut self) {
        for n in &mut self.nodes {
            if n.is_dir {
                n.expanded = true;
            }
        }
    }

    /// Clears all glob-match markers (called before applying a fresh glob).
    pub fn clear_glob_marks(&mut self) {
        for n in &mut self.nodes {
            n.matched_glob = false;
        }
    }

    /// Applies a glob pattern as an *include* operation: every path that
    /// matches becomes selected (and un-excluded). Returns stats for the
    /// status bar.
    ///
    /// Matching is done with the `glob` crate using gitignore-style
    /// semantics, consistent with TreeClip's own `--exclude` engine
    /// (which is built on the `ignore` crate - same engine).
    pub fn apply_glob_include(&mut self, pattern: &str) -> Result<GlobStats, String> {
        let matcher = build_glob_matcher(pattern, &self.root)?;
        let mut matched = 0usize;
        let mut already_excluded = 0usize;

        for n in &mut self.nodes {
            let is_match = relative_match(&matcher, &self.root, &n.path);
            if is_match {
                n.matched_glob = true;
                matched += 1;
                if n.excluded {
                    already_excluded += 1;
                }
                n.selected = true;
                n.excluded = false;
            }
        }
        Ok(GlobStats {
            matched,
            already_excluded,
        })
    }

    /// Applies a glob pattern as an *exclude* operation: every path that
    /// matches becomes excluded (selected flag cleared as well, since
    /// excluded wins over selected anyway).
    pub fn apply_glob_exclude(&mut self, pattern: &str) -> Result<GlobStats, String> {
        let matcher = build_glob_matcher(pattern, &self.root)?;
        let mut matched = 0usize;
        let mut already_excluded = 0usize;

        for n in &mut self.nodes {
            let is_match = relative_match(&matcher, &self.root, &n.path);
            if is_match {
                n.matched_glob = true;
                matched += 1;
                if n.excluded {
                    already_excluded += 1;
                }
                n.excluded = true;
            }
        }
        Ok(GlobStats {
            matched,
            already_excluded,
        })
    }
}

/// Builds a single-pattern gitignore-style matcher rooted at `root`.
///
/// The `ignore` crate's `OverrideBuilder` is reused here (it's what
/// TreeClip already uses for input glob expansion), keeping semantics
/// identical across CLI and TUI.
fn build_glob_matcher(pattern: &str, root: &Path) -> Result<ignore::overrides::Override, String> {
    let mut builder = ignore::overrides::OverrideBuilder::new(root);
    builder
        .add(pattern)
        .map_err(|e| format!("Invalid glob pattern '{pattern}': {e}"))?;
    builder
        .build()
        .map_err(|e| format!("Failed to compile glob '{pattern}': {e}"))
}

/// Returns `true` iff `path` matches `matcher` under `root`.
fn relative_match(matcher: &ignore::overrides::Override, root: &Path, path: &Path) -> bool {
    let relative = path.strip_prefix(root).unwrap_or(path);
    matcher.matched(relative, path.is_dir()).is_whitelist()
}

/// Collects all *unique* directories that contain any effectively-included
/// file. Used by the run handoff to expand the input paths back into
/// directories the walker can recurse - the same way the CLI does.
#[allow(dead_code)]
pub fn unique_dirs_of(paths: &[PathBuf]) -> Vec<PathBuf> {
    let mut set: HashSet<PathBuf> = HashSet::new();
    for p in paths {
        if let Some(parent) = p.parent() {
            set.insert(parent.to_path_buf());
        }
    }
    let mut out: Vec<_> = set.into_iter().collect();
    out.sort();
    out
}
