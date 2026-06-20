//! main - Entry point for the TreeClip CLI application.

// use dioxus::prelude::*;
use crate::{cli::Cli, commands::run};
use clap::Parser;
use cli::*;

mod app;
mod cli;
mod commands;
mod core;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Run(run_args) => run::execute(run_args)?,
        Commands::Init(args) => commands::init_handler::handle(args)?,
    }

    //todo(GUI): GUI version commented out - Implement
    // let cfg = dioxus::desktop::Config::new().with_window(
    //     dioxus::desktop::WindowBuilder::new()
    //         .with_title("🌳 TreeClip GUI")
    //         .with_inner_size(dioxus::desktop::tao::dpi::LogicalSize::new(800.0, 600.0)),
    // );
    // LaunchBuilder::desktop().with_cfg(cfg).launch(app::App);

    Ok(())
}

#[cfg(test)]
mod main_tests {
    use super::cli::*;
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
