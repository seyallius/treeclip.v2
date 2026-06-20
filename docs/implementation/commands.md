# Commands Module (`commands/`)

This module contains the logic for executing specific CLI commands. Currently, it houses the `run` and `init` commands.

## Run Command (`run.rs`)

The `execute` function in `src/commands/run.rs` is the central orchestrator for the `treeclip run` command. It follows
this sequence:

1. **Banner Display:** Shows a welcome message unless `--fast-mode` is enabled.
2. **Glob Expansion & Path Normalization:** Expands any glob patterns in input paths into concrete file/directory
   paths, then converts relative paths (like `.`) to absolute paths for consistent processing.
3. **Configuration Logging:** Prints the effective configuration (expanded input paths, output path, flags, exclusions)
   using UI formatting utilities.
4. **Traversal Execution:** Creates a `Walker` instance and calls `process_dir` to perform the actual file scanning and
   bundling.
5. **Clipboard Handling:** If `--clipboard` is set, initializes a `Clipboard` instance and copies the output file's
   content.
6. **Statistics Display:** If `--stats` is set, reads the output file and calculates/display statistics (lines, words,
   characters, size).
7. **Editor Handling:** If `--editor` is set, opens the output file using the system's default editor. If `--delete` is
   also set, the file is deleted after the editor closes.
8. **Goodbye Banner:** Shows a goodbye message unless `--fast-mode` is enabled.

### Glob Expansion in normalize_paths

The `normalize_paths` function calls `glob::expand_input_paths()` to resolve glob patterns before any traversal. This
happens early in the pipeline so that the rest of the command operates on concrete paths:

```rust
// Expand glob patterns in input paths
let expanded_strings = glob::expand_input_paths(&args.input_paths)
    .with_context(|| format!("Failed to expand input paths: {:?}", args.input_paths))?;
```

If a pattern matches nothing, an error is raised immediately rather than silently producing empty output.

## Arguments (`args.rs`)

The `args.rs` file defines the `RunArgs` struct using `clap::Args`. This struct holds all the parsed command-line
arguments specific to the `run` command, such as `input_paths`, `output_path`, `exclude`, `clipboard`, `stats`, etc.

### input_paths Field

The `input_paths` field is declared as `Vec<String>` (rather than `Vec<PathBuf>`) to accommodate glob pattern
characters. A dedicated `validate_input` function ensures the string is non-empty but does not attempt path validation
(since glob patterns like `src/**/*.rs` are not literal paths). The actual expansion happens in `run.rs::normalize_paths`.
