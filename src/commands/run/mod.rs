pub(crate) mod run;

#[derive(clap::Args)]
pub(crate) struct RunArgs {
    /// Path to traverse (defaults to current directory)
    #[arg(default_value_t = String::from("."))]
    pub input_path: String,

    /// Output path for extracted file (defaults to current directory)
    #[arg(default_value_t = String::from("."))]
    pub output_path: String,

    /// Exclude files/folders matching these patterns
    #[arg(short, long)]
    pub exclude: Vec<String>,

    /// Copy output to clipboard
    #[arg(long, default_value_t = true)]
    pub clipboard: bool,

    /// Show clipboard content statistics
    #[arg(long, default_value_t = false)]
    pub stats: bool,

    /// Open output file in the default text editor
    #[arg(long, default_value_t = false)]
    pub editor: bool,

    /// Delete the output file after editor is closed
    #[arg(long, default_value_t = false)]
    pub delete: bool,

    /// Verbose output
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}
