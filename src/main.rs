//! main - Entry point for the TreeClip CLI application.

use crate::commands::run;
use clap::Parser;
use cli::*;
use std::env;

mod cli;
mod commands;
mod core;
mod tui;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        // Headless CLI flow — unchanged from before.
        Some(Commands::Run(run_args)) => run::execute(run_args)?,
        Some(Commands::Init(args)) => commands::init_handler::handle(args)?,

        // Bare `treeclip` — interactive TUI flow.
        None => {
            // The TUI is rooted at the current directory, mirroring the
            // default of `treeclip run .` so the user's mental model is
            // "I'm standing in my project, let me bundle it".
            let root = env::current_dir()?;

            match tui::run_tui(&root)? {
                Some(args) => {
                    // User confirmed a run. Hand the constructed `RunArgs`
                    // to the existing pipeline so all the existing walker /
                    // exclude / clipboard / editor / stats code runs
                    // unchanged.
                    run::execute(args)?;
                }
                None => {
                    // User canceled (`q` / Esc / Ctrl-C). Nothing to do.
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod main_tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_cli_parsing_does_not_panic() -> anyhow::Result<()> {
        let temp_dir = tempfile::tempdir()?;
        std::env::set_current_dir(&temp_dir)?;

        let args = vec!["treeclip", "run", "."];
        let result = std::panic::catch_unwind(|| {
            let _ = Cli::parse_from(args);
        });

        assert!(result.is_ok());
        Ok(())
    }

    #[test]
    fn test_bare_treeclip_yields_none() {
        let cli = Cli::parse_from(&["treeclip"]);
        assert!(cli.command.is_none());
    }
}
