# Frequently Asked Questions (FAQ)

Find answers to common questions about TreeClip.

## General

### Q: What is TreeClip primarily used for?

**A:** TreeClip is designed to simplify the process of sharing entire codebases with AI assistants. Instead of manually
copying and pasting individual files, TreeClip bundles all relevant code files into a single, structured text file that
can be easily pasted into an AI chat interface.

### Q: How does TreeClip differ from `tar` or `zip`?

**A:** While `tar` and `zip` create compressed archives, TreeClip creates a single, human-readable text file with clear
headers for each included file's content. This format is specifically optimized for consumption by AI models and
text-based interfaces, preserving context and structure in a flat, easy-to-parse way.

### Q: Can TreeClip handle binary files?

**A:** TreeClip is designed primarily for text-based code files. It attempts to read files using UTF-8 encoding. Binary
files will likely cause errors or produce unreadable output and are generally not included in the output unless
explicitly targeted and readable as text.

## Usage

### Q: What's the difference between `--skip-hidden` and `--no-skip-hidden`?

**A:** By default, TreeClip uses `--skip-hidden`, which means it will skip files and directories whose names start with
a dot (e.g., `.git`, `.env`, `.vscode`). Using `--no-skip-hidden` disables this behavior and includes these hidden items
in the bundle.

### Q: How does the `.treeclipignore` file work?

**A:** The `.treeclipignore` file works similarly to `.gitignore`. It contains patterns for files and directories that
TreeClip should exclude from the bundling process. TreeClip looks for this file in the directory specified by the
`--root` option (defaulting to the current directory) and applies its rules during traversal.

### Q: Can I use TreeClip with any AI assistant?

**A:** Yes, TreeClip outputs a standard text format that can be pasted into any text-based interface, including popular
AI assistants like ChatGPT, Claude, Gemini, and others.

## Technical

### Q: Is TreeClip written in Rust?

**A:** Yes, TreeClip is written in Rust. This provides benefits like memory safety, performance, and easy cross-platform
compilation.

### Q: How does TreeClip handle large projects?

**A:** TreeClip uses parallel processing (via the `rayon` crate) to read file contents concurrently, which helps with
performance on large projects. However, very large projects will still result in large output files, which might have
implications for clipboard size limits or AI model context windows.

### Q: Can I exclude files based on their content?

**A:** Currently, TreeClip only supports exclusion based on file/directory names and paths using glob patterns (like in
`.gitignore`). Exclusion based on file *content* is not implemented.

## Development

### Q: Where can I find the source code?

**A:** The source code for TreeClip is available on
GitHub: [https://github.com/seyallius/treeclip.v2](https://github.com/seyallius/treeclip.v2)

### Q: Can I contribute to TreeClip?

**A:** Absolutely! Contributions are welcome. Check the repository's `README` or `CONTRIBUTING.md` (if available) for
guidelines on how to submit bug reports, feature requests, or pull requests.
