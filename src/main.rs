use crate::commands::run::*;
use clap::Parser;

mod commands;

#[derive(clap::Parser)]
#[command(name = "treeclip")]
#[command(version = "v0.1.0")]
#[command(
    about = "Traverse directories and files and extract it's contents",
    long_about = "Traverse directories and files and extract contents into a temporary folder and/or clipboard."
)]
#[command(next_line_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Run treeclip on a directory
    #[command(arg_required_else_help = true)]
    Run(args::RunArgs),
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Run(run_args) => run::run(run_args)?,
    }

    Ok(())
}
