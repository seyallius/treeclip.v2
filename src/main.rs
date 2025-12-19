use crate::commands::run::*;
use clap::Parser;
use cli::*;

mod cli;
mod commands;
mod core;

fn main() -> anyhow::Result<()> {
    // Small delay for dramatic effect
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
    fn test_main_success() -> anyhow::Result<()> {
        let temp_dir = tempfile::tempdir()?;

        // Test with a valid directory
        std::env::set_current_dir(&temp_dir)?;
        let args = vec!["treeclip", "run", "."];

        // This would test the full execution path
        // For now, just test that it doesn't panic
        let result = std::panic::catch_unwind(|| {
            let _ = Cli::parse_from(args);
        });

        assert!(result.is_ok());
        Ok(())
    }
}
