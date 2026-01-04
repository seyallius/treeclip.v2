# CLI Module (`cli.rs`)

This module defines the command-line interface for TreeClip using the `clap` crate.

## Structure

The main structure is the `Cli` struct, which is derived from `clap::Parser`. It defines the top-level commands
available in the application.

```rust
#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
```

The `Commands` enum defines the available subcommands, currently only `Run`.

```rust
#[derive(Subcommand)]
pub enum Commands {
    Run(args::RunArgs),
}
```

## Configuration

The `#[command(...)]` attribute on the `Cli` struct configures various aspects of the CLI:

- **Name, Version, Author:** Automatically populated from `Cargo.toml`.
- **About/Long About:** Provides short and detailed descriptions shown in help.
- **After Help:** Displays extended examples and usage tips after the main help text.
- **Styles:** Defines custom colors and formatting for the help output.
- **Options:** Like `arg_required_else_help` (shows help if no subcommand is provided) and `disable_help_subcommand` (
  disables the explicit `help` subcommand).

## Argument Definitions

The actual arguments for the `run` command are defined in the `src/commands/args.rs` file, which is referenced here via
`args::RunArgs`. The `Cli` module primarily serves as the top-level orchestrator for argument parsing.
