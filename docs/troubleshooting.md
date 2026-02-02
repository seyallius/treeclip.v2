# Troubleshooting

This section addresses common issues and errors you might encounter while using TreeClip.

## Common Issues

### 1. Command Not Found

**Problem:** Running `treeclip` results in `command not found`.

**Solution:**

- Ensure TreeClip was installed correctly using `cargo install treeclip`.
- Verify that the Cargo binary directory (usually `~/.cargo/bin`) is added to your system's `$PATH`.
- Try opening a new terminal session or sourcing your shell profile (e.g., `source ~/.bashrc` or `source ~/.zshrc`).

### 2. Clipboard Operation Fails

**Problem:** The `--clipboard` flag doesn't seem to copy the content, or an error occurs.

**Solution:**

- **Linux:** Clipboard functionality requires a running clipboard manager (like `xclip`, `xsel`, `wl-clipboard`). Ensure
  one is installed and running in your desktop environment.
- **CI/CD Environments:** Clipboard operations are generally not available in headless environments. Avoid using
  `--clipboard` in scripts running on CI/CD servers.
- **Large Files:** The clipboard might have size limitations. Check if the output file size is extremely large.

### 3. "Path does not exist" Error

**Problem:** TreeClip reports that an input path does not exist.

**Solution:**

- Verify that the path specified as an argument to `treeclip run` is correct and exists.
- Ensure you are running the command from the correct directory if using relative paths.

### 4. "No files found" Error

**Problem:** TreeClip reports that no files were found in the specified directory.

**Solution:**

- Check if the directory actually contains readable files.
- Review your `.treeclipignore` file or `-e` exclusion patterns. They might be too broad and excluding all intended
  files.
- If using `--skip-hidden` (the default), ensure the files you want are not hidden (starting with `.`). Use
  `--no-skip-hidden` if needed.

### 5. Shell Glob Pattern Issues

**Problem:** Using wildcard patterns like `*.txt` with `-e` doesn't work as expected.

**Solution:**

- Always quote glob patterns to prevent shell expansion: `treeclip run -e '*.txt'` or `treeclip run -e "*.log"`. See
  the [Usage](./usage.md#shell-glob-patterns) page for details.

### 6. Editor Does Not Open or Fails

**Problem:** The `--editor` flag doesn't open the output file.

**Solution:**

- Ensure a default text editor is available on your system (`start` on Windows, `open` on macOS, `xdg-open` on Linux).
- If the default fails, TreeClip falls back to the `EDITOR` environment variable. Set it (e.g., `export EDITOR=nano` or
  `export EDITOR=vim`).
- Check the output of the command for any specific error messages related to opening the editor.

### 7. Init Command Fails

**Problem:** The `treeclip init` command fails.

**Solution:**

- **Directory Does Not Exist:** Ensure the directory specified with `-d` exists and is accessible.
- **Not a Directory:** The path specified with `-d` must point to a directory, not a file.
- **Permission Denied:** Make sure you have write permissions in the target directory.
- **Overwrite Confirmation:** If `.treeclipignore` already exists, `treeclip init` will ask for confirmation before
  overwriting it, unless the `--force` flag is used.

## Getting More Help

- Run `treeclip --help` or `treeclip run --help` or `treeclip init --help` for detailed command-line option information.
- Check the [Usage](./usage.md) page for comprehensive command details.
- If you suspect a bug, please report it on the [GitHub repository](https://github.com/seyallius/treeclip.v2/issues).
