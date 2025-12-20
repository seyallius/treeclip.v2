//! main - Entry point for the TreeClip CLI application.

use crate::commands::run;
use clap::Parser;
use cli::*;

mod cli;
mod commands;
mod core;

fn main() -> anyhow::Result<()> {
    // NOTE: Small delay for dramatic effect - consider removing in production
    std::thread::sleep(std::time::Duration::from_millis(100));

    let cli = Cli::parse();
    match cli.command {
        Commands::Run(run_args) => run::execute(run_args)?,
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
