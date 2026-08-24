//! tui - Public entry point for the TreeClip TUI.
//!
//! `run_tui(root)` is called from `main.rs` when the user invokes
//! `treeclip` with no subcommand. It opens the alternate screen, runs the
//! ratatui loop until the user quits or confirms a run, and returns either:
//!
//! - `Ok(Some(args))` — user pressed `r` / Enter; the caller should now
//!   run `commands::run::execute(args)` to bundle the selected files.
//! - `Ok(None)` — user pressed `q` / Esc; nothing to do.
//! - `Err(_)` — terminal setup failed.
//!
//! All stateful mutation lives in `App::apply`. All event handling lives in
//! `event::handle` (pure). All rendering lives in `ui::render` (pure).

use crate::{
    commands::args::RunArgs,
    tui::{app::App, event::Command},
};
use crossterm::{
    event as crossterm_event,
    event::Event as CtEvent,
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::{
    io::{self, IsTerminal},
    path::{Path, PathBuf},
    time::Duration,
};

pub mod app;
pub mod event;
pub mod file_tree;
pub mod help;
pub mod theme;
pub mod ui;

/// Polling interval for the input loop. We poll rather than block so we can
/// also expire the status-bar message TTL.
const POLL_INTERVAL: Duration = Duration::from_millis(50);

/// Entry point. See module docs.
pub fn run_tui(root: &Path) -> anyhow::Result<Option<RunArgs>> {
    if !io::stdout().is_terminal() {
        return Err(anyhow::anyhow!(
            "TreeClip TUI requires an interactive terminal. (Detected non-TTY stdout.)\n\
             Hint: pipe or redirect nothing, or use `treeclip run` for headless mode."
        ));
    }

    setup_terminal()?;
    let result = run_loop(root);
    teardown_terminal();

    result.map_err(|e| anyhow::anyhow!("TUI error: {e}"))
}

fn run_loop(root: &Path) -> anyhow::Result<Option<RunArgs>> {
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(root).map_err(|e| anyhow::anyhow!("Failed to build file tree: {e}"))?;

    loop {
        // Clear the previous frame and render the current state.
        terminal.draw(|f| ui::render(&app, f))?;

        // Poll for input. We poll with a short interval so the status-bar
        // message TTL can expire without user input.
        if !crossterm_event::poll(POLL_INTERVAL)? {
            continue;
        }

        let ev = match crossterm_event::read() {
            Ok(e) => e,
            Err(e) => {
                return Err(anyhow::anyhow!("Failed to read event: {e}"));
            }
        };

        // We only care about key events; ignore resize / mouse for now.
        let CtEvent::Key(key) = ev else {
            continue;
        };

        let cmd = event::handle(key, &app);

        // Possibly drive modal edits / app actions / break-outs.
        let mut should_break: Option<Option<RunArgs>> = None;
        match cmd {
            Command::Edit(edit) => {
                let outcome = app.apply_input_edit(edit);
                // For Submit / Cancel, `apply_input_edit` has already reset
                // `mode`/`input`/`caret`. The action returned here maps the
                // submitted value to the proper side-effect (e.g. apply a
                // glob). `Continue` returns `Action::Noop`, which is harmless.
                let action = event::submit_to_action(&outcome);
                app.apply(action);
            }
            Command::AppAction(action) => {
                app.apply(action);
            }
            Command::BreakOut(action) => {
                app.apply(action);
                if app.should_run {
                    should_break = Some(Some(build_run_args(&app)));
                } else if app.should_quit {
                    should_break = Some(None);
                }
            }
        }

        // Status TTL: clear stale messages.
        if let Some((_, ts)) = app.status {
            if ts.elapsed() > Duration::from_secs(5) {
                app.status = None;
            }
        }

        if let Some(breakout) = should_break {
            return Ok(breakout);
        }
    }
}

/// Builds the `RunArgs` snapshot from the current `App` state. Called once
/// when the user confirms a run.
fn build_run_args(app: &App) -> RunArgs {
    let raw_inputs = app.selected_input_paths();
    // If the user has explicitly excluded the root, we still pass `[root]`
    // so the walker can apply the excludes and surface a clean
    // `NoFilesFound` error. In every other case we pass the explicit list
    // of effectively-included files (which may be empty if the user
    // deselected everything — that produces an empty bundle, which is the
    // user's explicit intent).
    let inputs = if raw_inputs.is_empty() && app.tree.is_root_excluded() {
        vec![app.tree.root.clone()]
    } else {
        raw_inputs
    };

    // Output path: empty string falls back to TreeClip's default (handled
    // inside `run::execute::normalize_paths`).
    let output_path = if app.output_path.trim().is_empty() {
        PathBuf::from(".")
    } else {
        PathBuf::from(&app.output_path)
    };

    let root = app.tree.root.clone();

    let t = app.toggles;

    // Emit literal exclude patterns as RELATIVE, leading-slash-anchored
    // paths from the walker's root. The walker's gitignore-style matcher
    // interprets a leading slash as "anchored to the matcher's root", so
    // `/target/` matches only the `target` directory at the project root,
    // not any other `target` directories that might exist deeper in the
    // tree. This is the most faithful translation of the TUI's per-node
    // exclusion state back into the CLI's exclusion vocabulary.
    let mut exclude: Vec<String> = Vec::new();
    for node in &app.tree.nodes {
        if !node.excluded {
            continue;
        }
        let relative = node
            .path
            .strip_prefix(&app.tree.root)
            .unwrap_or(node.path.as_path());
        // `strip_prefix` returns "" for the root itself, which the matcher
        // would interpret as "match every path" — guard against that.
        if relative.as_os_str().is_empty() {
            continue;
        }
        let mut pat = relative.display().to_string();
        if node.is_dir && !pat.ends_with('/') {
            pat.push('/');
        }
        exclude.push(format!("/{pat}"));
    }

    RunArgs {
        input_paths: inputs,
        output_path: Some(output_path),
        root: Some(root),
        exclude,
        clipboard: t.clipboard,
        stats: t.stats,
        editor: t.editor,
        delete: t.delete,
        verbose: t.verbose,
        skip_hidden: t.skip_hidden,
        // The negatable-flag pair on RunArgs: `--no-skip-hidden` flips this off.
        // We just mirror `skip_hidden`'s value here; the post-processing in
        // `run::execute` will XOR them.
        no_skip_hidden: !t.skip_hidden,
        raw: true,
        fast_mode: t.fast_mode,
        tree: t.tree,
    }
}

fn setup_terminal() -> anyhow::Result<()> {
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;
    Ok(())
}

fn teardown_terminal() {
    let _ = execute!(io::stdout(), LeaveAlternateScreen);
    let _ = disable_raw_mode();
}
