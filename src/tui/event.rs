//! event - Stateless event handler.
//!
//! The whole point of this module is that it owns *no* state and never mutates
//! anything. `handle(key, app)` reads from `app` (immutable) and returns
//! either an `Action` (for normal-mode keys) or an `InputEdit` (for modal
//! keys). The TUI loop in `mod.rs` then feeds that to `App::apply` /
//! `App::apply_input_edit` — the only place where mutation happens.
//!
//! Why split it this way? Because the rendering layer (`ui.rs`) and the input
//! layer (`event.rs`) are now trivially testable in isolation: render a fake
//! `App`, call `handle` on a fake key, assert the action. No terminal, no
//! crossterm magic.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::tui::app::{Action, App, InputEdit, InputOutcome, Mode, Panel, OPTION_ROWS};
use crate::tui::file_tree::FileNode;

/// What `handle` returns.
#[derive(Debug, Clone)]
pub enum Command {
    AppAction(Action),
    Edit(InputEdit),
    /// The user pressed Ctrl-C in any context, or `q` in normal mode, etc.
    /// The outer loop is responsible for translating this to `should_quit`.
    BreakOut(Action),
}

/// Top-level dispatcher. Routes to `handle_modal` when in an input mode, to
/// `handle_help` when in help mode, otherwise to `handle_normal`.
pub fn handle(key: KeyEvent, app: &App) -> Command {
    // Global: Ctrl-C always quits, Esc is contextual.
    if key.modifiers.contains(KeyModifiers::CONTROL) {
        if matches!(key.code, KeyCode::Char('c') | KeyCode::Char('C')) {
            return Command::BreakOut(Action::Quit);
        }
    }

    match app.mode {
        Mode::Help => handle_help(key),
        Mode::InputIncludeGlob | Mode::InputExcludeGlob | Mode::InputOutputPath => {
            handle_modal(key)
        }
        Mode::ConfirmRun => handle_confirm(key),
        Mode::Message => handle_message(key),
        Mode::Normal => handle_normal(key, app),
    }
}

// -------------------------------------------- Normal mode --------------------------------------------

fn handle_normal(key: KeyEvent, app: &App) -> Command {
    use Action::*;

    // Cross-panel keys first.
    let code = key.code;
    match code {
        KeyCode::Tab => return Command::AppAction(PanelNext),
        KeyCode::BackTab => return Command::AppAction(PanelNext),
        KeyCode::Char('?') => return Command::AppAction(SetMode(Mode::Help)),
        KeyCode::Char('q') | KeyCode::Esc => return Command::BreakOut(Quit),
        // `r` is the universal "go to the confirm-run popup" key in normal
        // mode. `Enter` is panel-specific (run from tree, toggle/edit from
        // options), so it's handled below.
        KeyCode::Char('r') => return Command::AppAction(SetMode(Mode::ConfirmRun)),
        KeyCode::Char('I') => return Command::AppAction(SetMode(Mode::InputIncludeGlob)),
        KeyCode::Char('X') => return Command::AppAction(SetMode(Mode::InputExcludeGlob)),
        KeyCode::Char('a') => return Command::AppAction(SelectAll),
        KeyCode::Char('A') => return Command::AppAction(SelectNone),
        KeyCode::Char('*') => return Command::AppAction(ExpandAll),
        KeyCode::Char('_') => return Command::AppAction(CollapseAll),
        _ => {}
    }

    // Panel-specific.
    match app.panel {
        Panel::Tree => handle_tree_panel(code, app),
        Panel::Options => handle_options_panel(code, app),
    }
}

fn handle_tree_panel(code: KeyCode, app: &App) -> Command {
    use Action::*;
    let node: Option<&FileNode> = app.tree.nodes.get(app.cursor);

    match code {
        KeyCode::Up | KeyCode::Char('k') => Command::AppAction(CursorUp(1)),
        KeyCode::Down | KeyCode::Char('j') => Command::AppAction(CursorDown(1)),
        KeyCode::PageUp => Command::AppAction(PageUp),
        KeyCode::PageDown => Command::AppAction(PageDown),
        KeyCode::Home => Command::AppAction(CursorTop),
        KeyCode::End => Command::AppAction(CursorBottom),
        KeyCode::Char('g') => Command::AppAction(CursorTop),
        KeyCode::Char('G') => Command::AppAction(CursorBottom),
        // `Enter` from the tree panel triggers the confirm-run popup.
        KeyCode::Enter => Command::AppAction(SetMode(Mode::ConfirmRun)),
        KeyCode::Left | KeyCode::Char('h') => {
            if node.is_some_and(|n| n.is_dir && n.expanded) {
                Command::AppAction(ExpandSet(false))
            } else {
                Command::AppAction(PanelNext)
            }
        }
        KeyCode::Right | KeyCode::Char('l') => {
            if node.is_some_and(|n| n.is_dir && !n.expanded) {
                Command::AppAction(ExpandSet(true))
            } else {
                Command::AppAction(ExpandToggle)
            }
        }
        KeyCode::Char(' ') => Command::AppAction(ToggleSelect),
        KeyCode::Char('x') => Command::AppAction(ToggleExclude),
        KeyCode::Char('+') => Command::AppAction(ExpandSet(true)),
        KeyCode::Char('-') => Command::AppAction(ExpandSet(false)),
        _ => Command::AppAction(Noop),
    }
}

fn handle_options_panel(code: KeyCode, app: &App) -> Command {
    use Action::*;
    // If the cursor is on the Output row (== OPTION_ROWS.len()), Space
    // and Enter should open the edit modal instead of toggling a boolean.
    let on_output_row = app.option_cursor == OPTION_ROWS.len();
    match code {
        KeyCode::Up | KeyCode::Char('k') => Command::AppAction(OptionUp),
        KeyCode::Down | KeyCode::Char('j') => Command::AppAction(OptionDown),
        KeyCode::Char(' ') | KeyCode::Enter => {
            if on_output_row {
                Command::AppAction(SetMode(Mode::InputOutputPath))
            } else {
                Command::AppAction(ToggleOption)
            }
        }
        KeyCode::Char('o') => Command::AppAction(SetMode(Mode::InputOutputPath)),
        _ => Command::AppAction(Noop),
    }
}

// -------------------------------------------- Modal input modes --------------------------------------------

fn handle_modal(key: KeyEvent) -> Command {
    let code = key.code;
    match code {
        KeyCode::Esc => Command::Edit(InputEdit::Cancel),
        KeyCode::Enter => Command::Edit(InputEdit::Submit),
        KeyCode::Backspace => Command::Edit(InputEdit::Backspace),
        KeyCode::Delete => Command::Edit(InputEdit::DeleteForward),
        KeyCode::Left => Command::Edit(InputEdit::CaretLeft),
        KeyCode::Right => Command::Edit(InputEdit::CaretRight),
        KeyCode::Home => Command::Edit(InputEdit::CaretStart),
        KeyCode::End => Command::Edit(InputEdit::CaretEnd),
        KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            Command::Edit(InputEdit::Clear)
        }
        KeyCode::Char(c) => Command::Edit(InputEdit::InsertChar(c)),
        _ => Command::AppAction(Action::Noop),
    }
}

// -------------------------------------------- Help overlay --------------------------------------------

fn handle_help(key: KeyEvent) -> Command {
    match key.code {
        KeyCode::Esc | KeyCode::Char('?') | KeyCode::Char('q') | KeyCode::Enter => {
            Command::AppAction(Action::SetMode(Mode::Normal))
        }
        _ => Command::AppAction(Action::Noop),
    }
}

// -------------------------------------------- Confirm-run / Message overlays --------------------------------------------

fn handle_confirm(key: KeyEvent) -> Command {
    match key.code {
        KeyCode::Char('y') | KeyCode::Enter => Command::BreakOut(Action::Run),
        KeyCode::Char('n')
        | KeyCode::Esc
        | KeyCode::Char('q')
        | KeyCode::Char('c')
        | KeyCode::Backspace => Command::AppAction(Action::SetMode(Mode::Normal)),
        _ => Command::AppAction(Action::Noop),
    }
}

fn handle_message(key: KeyEvent) -> Command {
    match key.code {
        KeyCode::Esc | KeyCode::Enter | KeyCode::Char('q') => {
            Command::AppAction(Action::SetMode(Mode::Normal))
        }
        _ => Command::AppAction(Action::Noop),
    }
}

// -------------------------------------------- Post-input router --------------------------------------------

/// Called by the TUI loop after `apply_input_edit` returns `Submit`. Maps
/// `(mode, value)` to the right `Action` so the rest of the state machine
/// can stay simple. Takes `&InputOutcome` so the caller can inspect the
/// outcome both for branching and for routing without a move.
pub fn submit_to_action(outcome: &InputOutcome) -> Action {
    match outcome {
        InputOutcome::Continue => Action::Noop,
        InputOutcome::Cancel => Action::SetMode(Mode::Normal),
        InputOutcome::Submit(mode, value) => match mode {
            Mode::InputIncludeGlob => Action::GlobInclude(value.clone()),
            Mode::InputExcludeGlob => Action::GlobExclude(value.clone()),
            Mode::InputOutputPath => Action::SetOutputPath(value.clone()),
            // Other modes shouldn't submit; treat as no-op.
            _ => Action::SetMode(Mode::Normal),
        },
    }
}
