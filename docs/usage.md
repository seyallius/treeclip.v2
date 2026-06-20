# Usage

This page details all the commands, options, and usage patterns available in TreeClip.

## Main Command: `run`

The primary command for TreeClip is `run`. It orchestrates the directory traversal, file extraction, and output
handling.

```bash
treeclip run [OPTIONS] [INPUT_PATHS]...
```

### Positional Arguments

| Argument      | Description                                                                                                    | Default                 |
|---------------|----------------------------------------------------------------------------------------------------------------|-------------------------|
| `INPUT_PATHS` | One or more directories or glob patterns to traverse. Supports both literal paths and git-style glob patterns. | `.` (current directory) |

> **Note:** You can specify multiple input paths to combine files from different directories into a single output file.
> Input paths support glob patterns — see [Glob Patterns](#glob-patterns) below for details.

#### Examples of Multiple Input Paths

```bash
# Bundle current directory and src folder
treeclip run . src -o output.txt

# Bundle multiple specific directories
treeclip run src/ tests/ examples/ -o combined.txt

# Use glob patterns to select specific files
treeclip run 'src/**/*.rs' 'tests/**/*.rs' -o rust-code.txt

# Mix literal paths and glob patterns
treeclip run ./src '*.md' 'tests/**/*.rs' -o mixed.txt
```

### Optional Arguments

| Flag                   | Short | Description                                                                                                                                                                               | Default               |
|------------------------|-------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------------------|
| `--output-path <PATH>` | `-o`  | Path to the output file.                                                                                                                                                                  | `./treeclip_temp.txt` |
| `--root <PATH>`        |       | Root directory for `.treeclipignore` lookup (usually not needed to change).                                                                                                               | `.`                   |
| `--exclude <PATTERN>`  | `-e`  | Glob patterns to exclude. Can be specified multiple times.                                                                                                                                | None                  |
| `--clipboard`          | `-c`  | Copy the final output to the system clipboard.                                                                                                                                            | Off                   |
| `--stats`              |       | Display statistics (lines, words, characters, size) about the extracted content.                                                                                                          | Off                   |
| `--tree`               | `-t`  | Append the directory structure tree to the end of the output file.                                                                                                                        | Off                   |
| `--editor`             |       | Open the output file in your system's default editor after creation.                                                                                                                      | Off                   |
| `--delete`             |       | Delete the output file after closing the editor (requires `--editor`).                                                                                                                    | Off                   |
| `--verbose`            | `-v`  | Enable verbose output showing detailed progress information.                                                                                                                              | Off                   |
| `--skip-hidden`        | `-H`  | Skip hidden files and directories (those starting with "."). <br/>This is disabled by default. Using --skip-hidden enables this behavior and excludes these hidden items from the bundle. | **On**                |
| `--fast-mode`          | `-f`  | Execute quickly without animations or welcome banners. Useful for scripts or CI/CD.                                                                                                       | Off                   |
| `--help`               | `-h`  | Show help information.                                                                                                                                                                    | -                     |
| `--version`            | `-V`  | Show the version number.                                                                                                                                                                  | -                     |

## Glob Patterns

TreeClip supports git-style glob patterns for input paths, allowing you to selectively include files and directories
without manually listing each one. Glob patterns are expanded before traversal begins, and unmatched patterns produce an
error to prevent silently generating empty output.

### Supported Patterns

| Pattern  | Description                                       | Example                         |
|----------|---------------------------------------------------|---------------------------------|
| `*`      | Matches anything except path separators           | `*.rs` matches `main.rs`        |
| `**`     | Matches zero or more directories (recursive)      | `src/**/*.rs` matches all `.rs` |
| `?`      | Matches any single character except separators    | `file?.txt` matches `file1.txt` |
| `[abc]`  | Matches one character from the set                | `[abc].rs` matches `a.rs`       |
| `{a,b}`  | Matches one of the alternatives (brace expansion) | `{src,lib}/*.rs`                |
| `.[!.]*` | Matches dot-files (excluding `.` and `..`)        | `.[!.]*` matches `.env`         |

### Usage Examples

```bash
# All files in the object directory
treeclip run 'object/*'

# All Go files directly inside object/
treeclip run 'object/*.go'

# All Rust files recursively under src/
treeclip run 'src/**/*.rs'

# All test files in the entire project
treeclip run '**/*.test.js'

# Files in either src/ or lib/ directories
treeclip run '{src,lib}/*.rs'

# All markdown files in the root
treeclip run '*.md'

# Mix glob patterns with literal paths
treeclip run ./src 'tests/**/*.rs' README.md -o bundle.txt
```

### Important: Quote Your Patterns

Always quote glob patterns to prevent your shell from expanding them before TreeClip sees them:

```bash
# ✅ Correct — TreeClip handles the glob expansion
treeclip run 'src/**/*.rs'

# ❌ Wrong — Shell expands the glob, may miss recursive matches
treeclip run src/**/*.rs
```

On Windows (PowerShell), use double quotes:

```powershell
treeclip run "src/**/*.rs"
```

### Error Handling

If a glob pattern matches no paths, TreeClip reports an error instead of silently producing empty output:

```
Error: Glob pattern matched no paths: 'nonexistent/**/*.rs'
```

This helps catch typos and overly specific patterns early.

## `init`

The `init` command creates a default `.treeclipignore` file in the specified directory.

```bash
treeclip init [OPTIONS]
```

### Optional Arguments

| Flag                 | Short | Description                                                              | Default           |
|----------------------|-------|--------------------------------------------------------------------------|-------------------|
| `--directory <PATH>` | `-d`  | Target directory where `.treeclipignore` will be created.                | `.` (current dir) |
| `--force`            | `-f`  | Overwrite existing `.treeclipignore` without prompting for confirmation. | Off               |

#### Examples of Init Command

```bash
# Create .treeclipignore in the current directory
treeclip init

# Create .treeclipignore in a specific directory
treeclip init -d /path/to/project

# Force overwrite without confirmation
treeclip init --force

# Create .treeclipignore in a specific directory and force overwrite
treeclip init -d /path/to/project --force
```

### Init Command Behavior

1. **Location:** Creates a `.treeclipignore` file in the specified directory (or current directory if none specified).
2. **Content:** The generated file includes default patterns (like `target/`, `node_modules/`, etc.) and imports
   patterns from existing standard ignore files (`.gitignore`, `.dockerignore`, etc.) found in the target directory.
3. **Duplicates:** Automatically avoids adding duplicate patterns from different sources.
4. **Confirmation:** Asks for confirmation before overwriting an existing `.treeclipignore` file unless `--force` is
   used.

## Common Usage Patterns

Here are some common and useful commands:

| Scenario                               | Command                                                                                                               | Description                                                                                                 |
|----------------------------------------|-----------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------|
| **Quick Clipboard Copy**               | `treeclip run --clipboard`                                                                                            | Scans current directory, creates temp file, copies content to clipboard.                                    |
| **Specific Directory + Custom Output** | `treeclip run ./src -o ./docs/dump.txt`                                                                               | Scans only `./src`, saves output to `./docs/dump.txt`.                                                      |
| **Exclude Build Artifacts**            | `treeclip run -e node_modules -e target -e .git`                                                                      | Scans current directory but ignores common build artifacts and version control directories.                 |
| **Glob Pattern Input**                 | `treeclip run 'src/**/*.rs' 'tests/**/*.rs' --clipboard`                                                              | Bundles only Rust source and test files using glob patterns.                                                |
| **Review Before Sharing**              | `treeclip run --editor --delete`                                                                                      | Creates temp file, opens it in editor, deletes it after the editor closes.                                  |
| **The Full Experience™**               | `treeclip run ./my-project -o ./export/snapshot.txt -e node_modules -e "*.lock" --clipboard --stats --verbose --tree` | Combines many options: custom paths, exclusions, clipboard copy, stats, verbose output, and tree structure. |
| **Fast Mode (No Animations)**          | `treeclip run --fast-mode --clipboard`                                                                                | Executes quickly without UI elements, ideal for scripts.                                                    |
| **Include Hidden Files**               | `treeclip run --no-skip-hidden`                                                                                       | Includes files like `.env.example`, `.editorconfig` which are skipped by default.                           |
| **Stats Without Clipboard**            | `treeclip run --stats`                                                                                                | Creates output file and shows content statistics.                                                           |
| **Just Save to File**                  | `treeclip run ./src -o output.txt --fast-mode`                                                                        | Saves content to a file quickly without extra features.                                                     |
| **Verbose Progress Tracking**          | `treeclip run --verbose --clipboard`                                                                                  | Shows detailed progress information during execution.                                                       |
| **Multiple Directories**               | `treeclip run ./src ./tests ./examples -o combined.txt`                                                               | Combines files from multiple directories into a single output file.                                         |
| **Initialize Ignore File**             | `treeclip init`                                                                                                       | Creates a default `.treeclipignore` file in the current directory.                                          |

## Using `.treeclipignore`

For patterns you want to exclude *every time* (like `node_modules`, `target`, etc.), create a `.treeclipignore` file in
your project's root directory. It uses the same syntax as `.gitignore`.

Example `.treeclipignore`:

```text
# Dependencies
node_modules/
target/
vendor/

# Build artifacts & logs
dist/
build/
*.log
*.lock

# OS-specific files
.DS_Store
Thumbs.db
```

With this file present, TreeClip will automatically respect these rules without needing `-e` flags.

## Shell Glob Patterns (Exclusion)

> **Warning:** Be careful with shell glob patterns (like `*` or `?`) when used with the `-e` exclude flag. Your shell might
> expand them before passing them to TreeClip.

For example, running `treeclip run -e *.txt` in a directory containing `file1.txt` and `file2.txt` will actually execute
as `treeclip run -e file1.txt -e file2.txt`, which is likely not the intended exclusion pattern.

**Solution:** Always quote glob patterns intended for TreeClip:

```bash
treeclip run -e '*.txt' # Correct
treeclip run -e "*.log" # Also correct
```

> **Note:** This warning applies to the `-e` (exclude) flag. For input path glob patterns, quoting is always recommended
> to let TreeClip handle the expansion (especially for `**` recursive patterns that shells may not support).
