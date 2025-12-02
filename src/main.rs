use clap::Parser;
use std::path::PathBuf;

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
    command: Option<Commands>,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Process files in the specified path
    Path {
        /// Path to traverse (defaults to current directory)
        #[arg(default_value_t = String::from("."))]
        path: String,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Path { path }) => {
            run_treeclip(&path)?;
        }
        None => {
            // If no command provided, default to run in current directory
            println!("Running treeclip in current directory...");
            run_treeclip(".")?;
        }
    }

    Ok(())
}

fn run_treeclip(path: &str) -> anyhow::Result<()> {
    let path_buf = PathBuf::from(path);

    // Check if path exists
    if !path_buf.exists() {
        anyhow::bail!("Path does not exist: {}", path);
    }

    println!("Traversing directory: {}", path_buf.display());

    // Use walkdir for recursive traversal
    use walkdir::WalkDir;

    for entry in WalkDir::new(&path_buf)
        .into_iter()
        .filter_map(|e| e.ok()) // Skip entries we can't access
    {
        let path = entry.path();
        if path.is_file() {
            println!("📄 {}", path.display());
        } else if path.is_dir() {
            println!("📁 {}", path.display());
        }
    }

    Ok(())
}