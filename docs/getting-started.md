# Getting Started

This guide will help you understand TreeClip's core concepts and run your first command.

## What TreeClip Does

TreeClip simplifies the process of sharing your codebase with AI assistants. Instead of manually copying and pasting
individual files, TreeClip:

1. **Traverses** your specified directory (or the current directory by default).
2. **Gathers** all text-based code files.
3. **Bundles** them into a single output file.
4. **Formats** the output with clear headers (`==> path/to/file`) for each file's content.

This single file preserves the project's structure and context, making it easy for AI models to understand and analyze
your code.

## Your First Command

The most common and useful command is to bundle the current directory and copy it to your clipboard:

```bash
treeclip run --clipboard
```

This command does the following:

- Scans the current directory (`.`).
- Creates a temporary output file (usually `treeclip_temp.txt`).
- Reads all files and writes them to the output file with headers.
- Copies the entire content of the output file to your system clipboard.
- Shows a welcome banner and progress animation (unless `--fast-mode` is used).

After running this command, you can paste the entire codebase directly into your AI chat interface.

## Understanding the Output Format

TreeClip generates a simple, structured format:

```text
==> src/main.rs
fn main() {
    println!("Hello, world!");
}

==> src/lib.rs
pub fn add(left: usize, right: usize) -> usize {
    left + right
}
```

Each file is clearly separated by a header line starting with `==>`, followed by its relative path. This makes it easy
for both humans and AI to identify where each piece of code comes from.

## Next Steps

- Learn how to [install](./installation.md) TreeClip.
- Explore the full range of [commands and options](./usage.md).
- Check out [examples](./examples/) for specific use cases.

```
