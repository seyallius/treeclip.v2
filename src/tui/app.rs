//! app - Stateful TUI state struct.
//!
//! `App` owns every piece of mutable state the TUI needs: the file tree,
//! the cursor position, the active panel, modal inputs, and the option
//! toggles that mirror `RunArgs` flags.
//!
//! Mutators here are deliberately *trivial* — they only mutate. All
//! decisions about *what* to mutate are made by the stateless event layer
//! (`event::handle`), which returns an `Action`. `App::apply` is then the
//! only place state changes.
//!
//! This is the "be mindful of stateful and stateless" rule made concrete:
//! - Stateful: `App` and its mutators.
//! - Stateless: `event::handle` (pure), `ui::render` (pure), `theme::*`
//!   (constants), `help::*` (constants).

use std::{
    cell::Cell,
    path::{Path, PathBuf},
    time::Instant
};
use crate::tui::file_tree::FileTree;

/// Which panel the user is currently interacting with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Panel {
    Tree,
    Options,
}

/// Which modal mode the TUI is in. `Normal` means no modal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Help,
    InputIncludeGlob,
    InputExcludeGlob,
    InputOutputPath,
    ConfirmRun,
    Message, // generic transient message popup
}

/// All the booleans the user can flip in the Options panel. They map 1:1 to
/// fields on `RunArgs` so the hand-off to `run::execute` is a straight copy.
#[derive(Debug, Clone, Copy)]
pub struct Toggles {
    pub clipboard: bool,
    pub stats: bool,
    pub editor: bool,
    pub delete: bool,
    pub tree: bool,
    pub skip_hidden: bool,
    pub fast_mode: bool,
    pub verbose: bool,
}
impl Default for Toggles {
    fn default() -> Self {
        Self {
            clipboard: true,   // most common use case: bundle + paste
            stats: false,
            editor: false,
            delete: false,
            tree: false,
            skip_hidden: true, // TreeClip default
            fast_mode: true,   // the TUI itself is the "interactive" part - skip CLI animations
            verbose: false,
        }
    }
}

/// A pure list of (label, key) entries for the options panel, in display
/// order. The `key` is the field name used by `toggle_option_by_key` so
/// keyboard nav stays in sync with rendering.
pub const OPTION_ROWS: &[(&str, &str)] = &[
    ("Copy to clipboard", "clipboard"),
    ("Show statistics", "stats"),
    ("Open output in editor", "editor"),
    ("Delete output after editor", "delete"),
    ("Include directory tree", "tree"),
    ("Skip hidden files", "skip_hidden"),
    ("Fast mode (no CLI animations)", "fast_mode"),
    ("Verbose", "verbose"),
];

/// The whole TUI state.
pub struct App {
    /// Rooted file tree model.
    pub tree: FileTree,

    /// Visible cursor row in the tree (index into `tree.nodes`, NOT into the
    /// visible-indices list). Kept stable across collapse/expand by the event
    /// layer's navigation actions.
    pub cursor: usize,

    /// Vertical scroll offset in the tree panel (in lines). Held as a `Cell`
    /// so the (otherwise pure) renderer can update it without taking
    /// `&mut App`. The renderer reads `self.scroll.get()` and writes
    /// `self.scroll.set(new)` each frame.
    pub scroll: Cell<usize>,

    /// Currently focused panel.
    pub panel: Panel,

    /// Cursor position in the options panel.
    pub option_cursor: usize,

    /// All the booleans.
    pub toggles: Toggles,

    /// The user-entered output path. Defaults to TreeClip's "treeclip_temp.txt".
    pub output_path: String,

    /// Modal state.
    pub mode: Mode,

    /// Shared text input buffer used by all modal input modes.
    pub input: String,

    /// Caret position inside `input`.
    pub caret: usize,

    /// Set by the user pressing `r` / Enter. When `true`, the TUI loop will
    /// tear down the terminal and `run::execute` will be called with the
    /// constructed `RunArgs`.
    pub should_run: bool,

    /// Set by `q` / Ctrl-C / Esc-in-normal.
    pub should_quit: bool,

    /// Last status-bar message + when it was set. The renderer dims it after
    /// a few seconds (TTL handled by `ui.rs`).
    pub status: Option<(String, Instant)>,
}
impl App {
    /// Constructs a new App rooted at `root`. The tree is built synchronously
    /// (this is the only blocking I/O the TUI performs during construction).
    pub fn new(root: &Path) -> std::io::Result<Self> {
        let tree = FileTree::build(root)?;
        Ok(Self {
            tree,
            cursor: 0,
            scroll: Cell::new(0),
            panel: Panel::Tree,
            option_cursor: 0,
            toggles: Toggles::default(),
            output_path: "treeclip_temp.txt".to_string(),
            mode: Mode::Normal,
            input: String::new(),
            caret: 0,
            should_run: false,
            should_quit: false,
            status: Some((
                "Welcome! Press ? for help, r to bundle.".to_string(),
                Instant::now(),
            )),
        })
    }

    /// Apply an `Action` returned by the stateless event layer.
    pub fn apply(&mut self, action: Action) {
        match action {
            Action::Quit => self.should_quit = true,
            Action::Run => self.should_run = true,
            Action::Noop => {}
            Action::SetMode(m) => {
                self.mode = m;
                // When entering an input mode, clear the buffer + caret.
                if matches!(
                    m,
                    Mode::InputIncludeGlob
                        | Mode::InputExcludeGlob
                        | Mode::InputOutputPath
                ) {
                    if matches!(m, Mode::InputOutputPath) {
                        self.input = self.output_path.clone();
                    } else {
                        self.input.clear();
                    }
                    self.caret = self.input.len();
                }
            }
            Action::CursorUp(n) => self.move_cursor(-(n as isize)),
            Action::CursorDown(n) => self.move_cursor(n as isize),
            Action::PageUp => self.page_cursor(-1),
            Action::PageDown => self.page_cursor(1),
            Action::CursorTop => self.cursor = 0,
            Action::CursorBottom => self.cursor = self.tree.nodes.len().saturating_sub(1),
            Action::PanelNext => {
                self.panel = match self.panel {
                    Panel::Tree => Panel::Options,
                    Panel::Options => Panel::Tree,
                };
            }
            Action::OptionUp => {
                if self.option_cursor > 0 {
                    self.option_cursor -= 1;
                }
            }
            Action::OptionDown => {
                // +1 to include the Output-path row at the bottom.
                if self.option_cursor + 1 <= OPTION_ROWS.len() {
                    self.option_cursor += 1;
                }
            }
            Action::ToggleOption => {
                if let Some((_, key)) = OPTION_ROWS.get(self.option_cursor) {
                    self.toggle_option_by_key(key);
                }
                // If the cursor is on the Output row, the event layer is
                // responsible for routing Space/Enter to
                // `Action::SetMode(Mode::InputOutputPath)` instead of
                // `ToggleOption` — see `event::handle_options_panel`.
            }
            Action::ExpandToggle => {
                self.tree.toggle_expand(self.cursor);
            }
            Action::ExpandSet(v) => {
                self.tree.set_expand(self.cursor, v);
            }
            Action::ToggleSelect => {
                self.tree.toggle_select(self.cursor);
                self.flash(format!("Toggled selection on: {}",
                    self.tree.nodes[self.cursor].path.display()));
            }
            Action::ToggleExclude => {
                self.tree.toggle_exclude(self.cursor);
                self.flash(format!("Toggled exclusion on: {}",
                    self.tree.nodes[self.cursor].path.display()));
            }
            Action::SelectAll => self.tree.select_all(),
            Action::SelectNone => self.tree.select_none(),
            Action::ExpandAll => self.tree.expand_all(),
            Action::CollapseAll => {
                self.tree.collapse_all();
                // The cursor may now point at a node whose ancestor got
                // collapsed, leaving it invisible. Snap to the nearest
                // visible ancestor (or the root).
                self.snap_cursor_to_visible();
            }
            Action::GlobInclude(pattern) => {
                self.apply_glob(true, &pattern);
            }
            Action::GlobExclude(pattern) => {
                self.apply_glob(false, &pattern);
            }
            Action::SetOutputPath(p) => {
                self.output_path = p.clone();
                self.flash(format!("Output path set to: {p}"));
            }
            Action::ClearStatus => self.status = None,
        }
        // The renderer recomputes the visible-window scroll offset from the
        // current cursor every frame, so we don't need to maintain a scroll
        // invariant here. See `ui::render_tree_panel`.
    }

    /// Convenience mutator for keyboard-driven toggles in the options panel.
    fn toggle_option_by_key(&mut self, key: &str) {
        let t = &mut self.toggles;
        match key {
            "clipboard"   => t.clipboard = !t.clipboard,
            "stats"       => t.stats = !t.stats,
            "editor"      => t.editor = !t.editor,
            "delete"      => t.delete = !t.delete,
            "tree"        => t.tree = !t.tree,
            "skip_hidden" => t.skip_hidden = !t.skip_hidden,
            "fast_mode"   => t.fast_mode = !t.fast_mode,
            "verbose"     => t.verbose = !t.verbose,
            _ => {}
        }
    }

    fn apply_glob(&mut self, include: bool, pattern: &str) {
        // Defensive: clear prior glob marks so the user can see fresh matches.
        self.tree.clear_glob_marks();
        let result = if include {
            self.tree.apply_glob_include(pattern)
        } else {
            self.tree.apply_glob_exclude(pattern)
        };
        match result {
            Ok(stats) => {
                let verb = if include { "Included" } else { "Excluded" };
                self.flash(format!(
                    "{verb} {} file(s) matching '{pattern}'",
                    stats.matched
                ));
            }
            Err(e) => self.flash(e),
        }
    }

    /// Move the cursor by `delta` lines, staying within visible rows and
    /// clamping to the tree bounds.
    fn move_cursor(&mut self, delta: isize) {
        let visible = self.tree.visible_indices();
        if visible.is_empty() {
            return;
        }
        // Find the position of self.cursor inside `visible`. If it's not
        // there (e.g. it was hidden by a collapse), snap to the nearest
        // visible entry at-or-below the cursor.
        let pos = visible
            .iter()
            .position(|&i| i == self.cursor)
            .or_else(|| visible.iter().position(|&i| i >= self.cursor))
            .unwrap_or(visible.len() - 1);

        let new_pos = (pos as isize + delta)
            .clamp(0, visible.len() as isize - 1) as usize;
        self.cursor = visible[new_pos];
    }

    fn page_cursor(&mut self, dir: i32) {
        // Approximate: jump 10 rows at a time.
        self.move_cursor((dir * 10) as isize);
    }

    /// After an action that may have hidden the cursor's node (e.g.
    /// `CollapseAll`), walk up to the nearest visible ancestor and snap the
    /// cursor there. Cheap O(depth) scan.
    fn snap_cursor_to_visible(&mut self) {
        let visible = self.tree.visible_indices();
        if visible.is_empty() {
            return;
        }
        if visible.contains(&self.cursor) {
            return;
        }
        // Walk up the parent chain until we hit a visible node.
        let mut cursor = self.cursor;
        while let Some(parent) = self.tree.parent_of(cursor) {
            if visible.contains(&parent) {
                self.cursor = parent;
                return;
            }
            cursor = parent;
        }
        // Fallback: first visible node.
        self.cursor = visible[0];
    }

    fn flash(&mut self, msg: String) {
        self.status = Some((msg, Instant::now()));
    }

    // -------------------------------------------- Read-only queries used by the renderer / handoff --------------------------------------------

    /// The currently visible tree rows (indices into `tree.nodes`).
    pub fn visible(&self) -> Vec<usize> {
        self.tree.visible_indices()
    }

    /// Counts of selected & excluded files (for the options panel display).
    pub fn file_counts(&self) -> (usize, usize) {
        self.tree.file_counts()
    }

    /// Build the absolute paths to hand to `run::execute` as `input_paths`.
    /// If the user excluded the root or selected nothing specific and made
    /// no exclusions, returns `[root]` to preserve TreeClip's "bundle the
    /// whole dir" default behavior.
    pub fn selected_input_paths(&self) -> Vec<PathBuf> {
        self.tree.effectively_included_paths()
    }

    /// The `RunArgs`-shaped snapshot of the toggles, for handoff.
    pub fn toggles_snapshot(&self) -> Toggles {
        self.toggles
    }
}

/// Every possible state transition the TUI can perform. Produced by the
/// stateless event layer, consumed by `App::apply`.
#[derive(Debug, Clone)]
pub enum Action {
    Noop,
    Quit,
    Run,
    SetMode(Mode),
    CursorUp(usize),
    CursorDown(usize),
    PageUp,
    PageDown,
    CursorTop,
    CursorBottom,
    PanelNext,
    OptionUp,
    OptionDown,
    ToggleOption,
    ExpandToggle,
    ExpandSet(bool),
    ToggleSelect,
    ToggleExclude,
    SelectAll,
    SelectNone,
    ExpandAll,
    CollapseAll,
    GlobInclude(String),
    GlobExclude(String),
    SetOutputPath(String),
    ClearStatus,
}

/// Edit action inside a modal input box. Also pure - the event layer decides
/// what edit happened, `App::apply_input_edit` applies it.
#[derive(Debug, Clone, Copy)]
pub enum InputEdit {
    InsertChar(char),
    Backspace,
    DeleteForward,
    CaretLeft,
    CaretRight,
    CaretStart,
    CaretEnd,
    Clear,
    Submit, // Enter
    Cancel, // Esc / Ctrl-C
}

impl App {
    /// Apply an `InputEdit` to the current modal input buffer.
    pub fn apply_input_edit(&mut self, edit: InputEdit) -> InputOutcome {
        match edit {
            InputEdit::InsertChar(c) => {
                self.input.insert(self.caret, c);
                self.caret += c.len_utf8();
                InputOutcome::Continue
            }
            InputEdit::Backspace => {
                if self.caret > 0 {
                    // Find the previous char boundary.
                    let prev = self.input[..self.caret]
                        .char_indices()
                        .next_back()
                        .map(|(i, _)| i)
                        .unwrap_or(0);
                    self.input.replace_range(prev..self.caret, "");
                    self.caret = prev;
                }
                InputOutcome::Continue
            }
            InputEdit::DeleteForward => {
                if self.caret < self.input.len() {
                    let next = self.input[self.caret..]
                        .char_indices()
                        .nth(1)
                        .map(|(i, _)| self.caret + i)
                        .unwrap_or(self.input.len());
                    self.input.replace_range(self.caret..next, "");
                }
                InputOutcome::Continue
            }
            InputEdit::CaretLeft => {
                if self.caret > 0 {
                    let prev = self.input[..self.caret]
                        .char_indices()
                        .next_back()
                        .map(|(i, _)| i)
                        .unwrap_or(0);
                    self.caret = prev;
                }
                InputOutcome::Continue
            }
            InputEdit::CaretRight => {
                if self.caret < self.input.len() {
                    let next = self.input[self.caret..]
                        .char_indices()
                        .nth(1)
                        .map(|(i, _)| self.caret + i)
                        .unwrap_or(self.input.len());
                    self.caret = next;
                }
                InputOutcome::Continue
            }
            InputEdit::CaretStart => {
                self.caret = 0;
                InputOutcome::Continue
            }
            InputEdit::CaretEnd => {
                self.caret = self.input.len();
                InputOutcome::Continue
            }
            InputEdit::Clear => {
                self.input.clear();
                self.caret = 0;
                InputOutcome::Continue
            }
            InputEdit::Submit => {
                let value = self.input.clone();
                let mode = self.mode;
                // Reset modal state first so the renderer doesn't flash.
                self.input.clear();
                self.caret = 0;
                self.mode = Mode::Normal;
                InputOutcome::Submit(mode, value)
            }
            InputEdit::Cancel => {
                self.input.clear();
                self.caret = 0;
                self.mode = Mode::Normal;
                InputOutcome::Cancel
            }
        }
    }
}

/// Result of an `InputEdit` — drives the outer state machine.
#[derive(Debug, Clone)]
pub enum InputOutcome {
    /// The user is still editing; keep the modal open.
    Continue,
    /// The user submitted the modal. Caller maps `(mode, value)` to the
    /// appropriate `Action`.
    Submit(Mode, String),
    /// The user cancelled (Esc / Ctrl-C).
    Cancel,
}
