# Core Module (`core/`)

The `core` module contains the essential business logic for TreeClip, including file traversal, exclusion logic,
clipboard operations, editor interactions, error handling, and UI utilities.

## Sub-modules

- **`clipboard` (`core/clipboard/mod.rs`):** Handles interaction with the system clipboard using the `arboard` crate. It
  reads the final output file and places its content onto the clipboard.
- **`editor` (`core/editor/mod.rs`):** Provides functions to open the output file in the default system editor (using
  `start` on Windows, `open` on macOS, `xdg-open` on Linux) and to delete the file.
- **`errors` (`core/errors.rs`):** Defines custom error types using `thiserror` and provides guidance on error handling
  patterns within the application. It uses `anyhow` for context propagation.
- **`exclude` (`core/exclude/mod.rs`):** Manages file and directory exclusion patterns. It uses the `ignore` crate to
  parse `.treeclipignore`, `.gitignore`, and other standard ignore files, as well as CLI-provided patterns (`-e` flags).
- **`traversal` (`core/traversal/`):** Contains the logic for walking the directory tree.
    - `walker.rs`: The main `Walker` struct and its `traverse` method. It uses `walkdir` for traversal, applies
      exclusion rules, reads file contents (optionally in parallel using `rayon`), and writes them to the output file
      with the correct format.
    - `filter.rs`: Contains helper functions for filtering entries during traversal (e.g., checking for hidden files).
- **`tree` (`core/tree/mod.rs`):** Handles the generation and writing of the directory tree structure. It uses `walkdir`
  and `rayon` to build a tree representation and formats it using box-drawing characters.
- **`ui` (`core/ui/`):** Contains modules for user interface elements like banners, animations, formatting, and tables.
    - `animations.rs`: Provides spinner and animated dot functions.
    - `banner.rs`: Generates and displays welcome and goodbye banners.
    - `formatter.rs`: Formats configuration displays and statistics.
    - `messages.rs`: Centralizes user-facing message strings.
    - `table.rs`: A utility for creating aligned, Unicode-aware tables and boxes for displaying information.
- **`utils` (`core/utils.rs`):** Contains general utility functions like path validation, number formatting (with
  thousand separators), and byte size formatting.

## Key Concepts

- **Parallel Processing:** The `traversal/walker.rs` module leverages `rayon` (`par_iter`) to read file contents
  concurrently, which significantly speeds up processing for large projects.
- **Exclusion Matching:** The `exclude` module builds a matcher that efficiently determines if a given file or directory
  path should be skipped during traversal based on multiple sources of patterns.
- **Error Handling:** The `errors` module defines specific error types (`ClipboardError`, `FileSystemError`, etc.) and
  demonstrates how to chain them with context using `anyhow` for better debugging and user feedback.
- **UI Formatting:** The `ui` modules provide reusable components for creating a consistent and visually appealing
  command-line interface.
