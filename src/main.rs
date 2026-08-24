//! main - Entry point for the TreeClip CLI application.

use crate::{
    commands::{args, run},
    core::tui,
    core::ui::messages::Messages,
};
use clap::Parser;
use cli::*;
use std::{env, path::PathBuf};

mod cli;
mod commands;
mod core;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Run(run_args)) => run::execute(run_args)?,
        Some(Commands::Init(args)) => commands::init_handler::handle(args)?,
        None => {
            // 🌳 No subcommand? Launch the Interactive TUI!
            let root = env::current_dir()?;
            match tui::run_tui(&root)? {
                Some(tui_result) => {
                    if tui_result.input_paths.is_empty() {
                        println!("{}", Messages::tui_cancelled());
                        return Ok(());
                    }

                    // Construct RunArgs from TUI selections
                    let run_args = args::RunArgs {
                        input_paths: tui_result.input_paths,
                        output_path: Some(PathBuf::from("./treeclip_temp.txt")),
                        root: Some(root),
                        exclude: tui_result.exclude_patterns,
                        clipboard: true, // Default to true for TUI users
                        stats: true,     // Show them the stats!
                        editor: false,
                        delete: false,
                        verbose: false,
                        skip_hidden: true,
                        no_skip_hidden: false,
                        raw: true,
                        fast_mode: false, // Let them see the cute animations
                        tree: true,       // Include tree structure
                    };
                    run::execute(run_args)?;
                }
                None => {
                    println!("{}", Messages::tui_cancelled());
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
}
