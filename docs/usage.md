# Usage

This page details all the commands, options, and usage patterns available in TreeClip.

## Main Command: `run`

The primary command for TreeClip is `run`. It orchestrates the directory traversal, file extraction, and output
handling.

```bash
treeclip run [OPTIONS] [INPUT_PATHS]...
```

### Positional Arguments

| Argument      | Description                          | Default                 |
|---------------|--------------------------------------|-------------------------|
| `INPUT_PATHS` | One or more directories to traverse. | `.` (current directory) |

> **Note:** You can specify multiple input paths to combine files from different directories into a single output file.

#### Examples of Multiple Input Paths

```bash
# Bundle current directory and src folder
treeclip run . src -o output.txt

# Bundle multiple specific directories
treeclip run src/ tests/ examples/ -o combined.txt
```

### Optional Arguments

| Flag                   | Short | Description                                                                              | Default               |
|------------------------|-------|------------------------------------------------------------------------------------------|-----------------------|
| `--output-path <PATH>` | `-o`  | Path to the output file.                                                                 | `./treeclip_temp.txt` |
| `--root <PATH>`        |       | Root directory for `.treeclipignore` lookup (usually not needed to change).              | `.`                   |
| `--exclude <PATTERN>`  | `-e`  | Glob patterns to exclude. Can be specified multiple times.                               | None                  |
| `--clipboard`          | `-c`  | Copy the final output to the system clipboard.                                           | Off                   |
| `--stats`              |       | Display statistics (lines, words, characters, size) about the extracted content.         | Off                   |
| `--tree`               | `-t`  | Append the directory structure tree to the end of the output file.                       | Off                   |
| `--editor`             |       | Open the output file in your system's default editor after creation.                     | Off                   |
| `--delete`             |       | Delete the output file after closing the editor (requires `--editor`).                   | Off                   |
| `--verbose`            | `-v`  | Enable verbose output showing detailed progress information.                             | Off                   |
| `--skip-hidden`        | `-H`  | Skip hidden files and directories (those starting with `.`). This is enabled by default. | **On**                |
| `--no-skip-hidden`     |       | Include hidden files and directories (disable `--skip-hidden`).                          | Off                   |
| `--fast-mode`          | `-f`  | Execute quickly without animations or welcome banners. Useful for scripts or CI/CD.      | Off                   |
| `--help`               | `-h`  | Show help information.                                                                   | -                     |
| `--version`            | `-V`  | Show the version number.                                                                 | -                     |

## Common Usage Patterns

Here are some common and useful commands:

| Scenario                               | Command                                                                                                               | Description                                                                                                 |
|----------------------------------------|-----------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------|
| **Quick Clipboard Copy**               | `treeclip run --clipboard`                                                                                            | Scans current directory, creates temp file, copies content to clipboard.                                    |
| **Specific Directory + Custom Output** | `treeclip run ./src -o ./docs/dump.txt`                                                                               | Scans only `./src`, saves output to `./docs/dump.txt`.                                                      |
| **Exclude Build Artifacts**            | `treeclip run -e node_modules -e target -e .git`                                                                      | Scans current directory but ignores common build artifacts and version control directories.                 |
| **Review Before Sharing**              | `treeclip run --editor --delete`                                                                                      | Creates temp file, opens it in editor, deletes it after the editor closes.                                  |
| **The Full Experience™**               | `treeclip run ./my-project -o ./export/snapshot.txt -e node_modules -e "*.lock" --clipboard --stats --verbose --tree` | Combines many options: custom paths, exclusions, clipboard copy, stats, verbose output, and tree structure. |
| **Fast Mode (No Animations)**          | `treeclip run --fast-mode --clipboard`                                                                                | Executes quickly without UI elements, ideal for scripts.                                                    |
| **Include Hidden Files**               | `treeclip run --no-skip-hidden`                                                                                       | Includes files like `.env.example`, `.editorconfig` which are skipped by default.                           |
| **Stats Without Clipboard**            | `treeclip run --stats`                                                                                                | Creates output file and shows content statistics.                                                           |
| **Just Save to File**                  | `treeclip run ./src -o output.txt --fast-mode`                                                                        | Saves content to a file quickly without extra features.                                                     |
| **Verbose Progress Tracking**          | `treeclip run --verbose --clipboard`                                                                                  | Shows detailed progress information during execution.                                                       |
| **Multiple Directories**               | `treeclip run ./src ./tests ./examples -o combined.txt`                                                               | Combines files from multiple directories into a single output file.                                         |

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

## Shell Glob Patterns

> **Warning:** Be careful with shell glob patterns (like `*` or `?`). Your shell might expand them before passing them
> to TreeClip.

For example, running `treeclip run -e *.txt` in a directory containing `file1.txt` and `file2.txt` will actually execute
as `treeclip run -e file1.txt -e file2.txt`, which is likely not the intended exclusion pattern.

**Solution:** Always quote glob patterns intended for TreeClip:

```bash
treeclip run -e '*.txt' # Correct
treeclip run -e "*.log" # Also correct
```
