# TreeClip 🌳✨

*A delightfully fast CLI tool that bundles your code into a single, AI-friendly format*

<p align="center">
<sub><strong>Author’s Note:</strong>  
This README was drafted with AI assistance. <br/>
I’m usually too lazy to write proper docs, but I actually reviewed this one, so it shouldn’t be too cursed...<br/>
Besides, it writes better than me ( ¬ ࡇ,¬ )<br/>Though the code is written by me! no AI in that!</sub>
</p>

---

## What's This All About? (◕‿◕✿)

Ever tried explaining your entire codebase to an AI assistant, only to spend 20 minutes copy-pasting files? Yeah, me
too. That's why TreeClip exists!

TreeClip traverses your project directory, gathers all your code files, and bundles them into one neat package with
proper headers showing where each piece came from. It's like creating a "highlight reel" of your project that AI models
can actually digest in one go.

**Think of it as:** *Your project, but as a single, well-organized document that preserves all the context.*

---

## Why I Built This 🛠️

Honestly? I was learning Rust and needed a practical project. But also, I was tired of the tedious workflow of sharing
code with AI assistants:

1. Copy file contents
2. Paste into chat
3. Explain the file structure
4. Repeat 47 times
5. Lose track of which files you've shared
6. Question your life choices

TreeClip turns that into: `treeclip run --clipboard` → done. One command, everything's ready to paste.

Plus, I wanted to practice Rust's error handling, understand the module system, work with the filesystem APIs, and build
something actually useful. Learning by building real tools just hits different, you know?

---

## Installation 🚀

### From Crates.io (Recommended)

You can install `treeclip` directly from crates.io using Cargo:

```bash
cargo install treeclip
```

This will install the binary on your system, making it available from anywhere!

### From Source

If you'd rather build it yourself from the source code:

```bash
git clone https://github.com/seyallius/treeclip.v2.git
cd treeclip.v2
cargo build --release
```

The binary will be located at `target/release/treeclip`. You can also run `cargo install --path .` to install it locally
from the repository folder.

---

## How to Use It ✨

The most common use case is bundling the current directory and copying it to your clipboard. It's as simple as this:

```bash
# Bundle the current directory and copy it to the clipboard
treeclip run --clipboard
```

Now you can paste the entire project structure into your favorite AI chat! Easy peasy. (づ｡◕‿‿◕｡)づ

### Common Usage Patterns

Here’s a quick guide to some of the most useful commands. The table below covers most scenarios you'll encounter!

| #      | Scenario                                        | Command                                                                                                        | What It Does                                                                                                  | When To Use                                        |
|--------|-------------------------------------------------|----------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------|----------------------------------------------------|
| **1**  | **Quick Clipboard Copy**<br>*(My Daily Driver)* | `treeclip run --clipboard`                                                                                     | • Scans current directory<br>• Creates `treeclip_temp.txt`<br>• Copies to clipboard<br>• Shows tree emojis 🌳 | Pasting code into ChatGPT/Claude/etc               |
| **2**  | **Specific Directory + Custom Output**          | `treeclip run ./src -o ./docs/dump.txt`                                                                        | • Scans `./src` only<br>• Saves to custom location<br>• Doesn't touch clipboard                               | Documenting parts of project, creating archives    |
| **3**  | **Exclude Build Artifacts**                     | `treeclip run -e node_modules -e target -e .git`                                                               | • Scans current directory<br>• Ignores specified patterns<br>• Can stack multiple `-e` flags                  | Projects with dependencies/build outputs           |
| **4**  | **Review Before Sharing**                       | `treeclip run --editor --delete`                                                                               | • Creates temp file<br>• Opens in `$EDITOR`<br>• Deletes after closing                                        | When you want to edit before sharing               |
| **5**  | **The Full Experience™**                        | `treeclip run ./my-project -o ./export/snapshot.txt -e node_modules -e "*.lock" --clipboard --stats --verbose` | • Everything at once<br>• Full control<br>• Maximum verbosity<br>• Statistics shown                           | When you want ALL the features                     |
| **6**  | **Fast Mode (No Animations)**                   | `treeclip run --fast-mode --clipboard`                                                                         | • Instant execution<br>• No progress bars<br>• No cute emojis 😢                                              | CI/CD, scripts, large projects, or when in a hurry |
| **7**  | **Include Hidden Files**                        | `treeclip run --no-skip-hidden`                                                                                | • Includes `.env.example`, `.editorconfig`, etc<br>• Normally skipped by default                              | When you need config files included                |
| **8**  | **Stats Without Clipboard**                     | `treeclip run --stats`                                                                                         | • Creates output file<br>• Shows lines/words/bytes<br>• Size emoji feedback 🐣🐘🐋                            | Analyzing codebase size                            |
| **9**  | **Just Save to File**                           | `treeclip run ./src -o output.txt --fast-mode`                                                                 | • No clipboard<br>• No stats<br>• Just saves file quickly                                                     | Archiving, documentation generation                |
| **10** | **Verbose Progress Tracking**                   | `treeclip run --verbose --clipboard`                                                                           | • Shows every step<br>• File count updates<br>• Detailed logging                                              | Debugging, understanding what's included           |

### Pro-Tip: Use a `.treeclipignore` File!

For files and directories you *always* want to ignore (like `node_modules` or `target`), create a `.treeclipignore` file
in your project's root directory. It works just like a `.gitignore` file!

Here's a great starting point:

```
# .treeclipignore

# Dependencies
node_modules/
target/
.git/

# Build artifacts & logs
dist/
build/
*.log
*.lock
```

With this file in place, you can just run `treeclip run --clipboard` without needing to add `--exclude` flags every
time. So much easier!

---

## All Command Options

### Positional Arguments

| Argument     | Description           | Default                 |
|--------------|-----------------------|-------------------------|
| `INPUT_PATH` | Directory to traverse | `.` (current directory) |

### Optional Arguments

| Flag                   | Short | Description                          | Default               |
|------------------------|-------|--------------------------------------|-----------------------|
| `--output-path <PATH>` | `-o`  | Where to save the output file        | `./treeclip_temp.txt` |
| `--root <PATH>`        |       | Root directory for `.treeclipignore` | `.`                   |
| `--exclude <PATTERN>`  | `-e`  | Patterns to exclude (can repeat)     | None                  |
| `--clipboard`          | `-c`  | Copy output to clipboard             | Off                   |
| `--stats`              |       | Show content statistics              | Off                   |
| `--editor`             |       | Open output in default editor        | Off                   |
| `--delete`             |       | Delete output after closing editor   | Off                   |
| `--verbose`            | `-v`  | Show detailed progress               | Off                   |
| `--skip-hidden`        | `-H`  | Skip hidden files/folders            | **On**                |
| `--no-skip-hidden`     |       | Include hidden files                 | Off                   |
| `--raw`                | `-r`  | Extract raw content                  | **On**                |
| `--fast-mode`          | `-f`  | Skip animations, instant execution   | Off                   |
| `--help`               | `-h`  | Show help message                    | -                     |
| `--version`            | `-V`  | Show version                         | -                     |

---

## Output Format

TreeClip creates a clean, AI-friendly format:

```
==> src/main.rs
fn main() {
    println!("Hello, world!");
}

==> src/lib.rs
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

==> tests/integration_test.rs
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}
```

Each file is clearly separated with its path, making it easy for AI to understand your project structure. No confusion,
no missing context!

---

## Tips & Tricks 💡

### Shell Glob Pattern Expansion

⚠️ **Important Note on Shell Glob Patterns:**

When using glob patterns (wildcards like `*` or `?`) with TreeClip, be aware that your shell may expand them before
passing them to the program. This can cause unexpected behavior if matching files exist in your current directory.

For example for a directory structure like this:

```text
.
├── Cargo.toml
├── cliff.toml
├── something.toml
├── event
│   ├── main.go
│   ├── main_test.go        # will be ignored (*_test.go)
│   └── some_other_test.go  # will be ignored (*_test.go)
└── src                     # entire folder will be ignored (*.rs)
    ├── main.rs
    ├── lib.rs
    └── utils.rs
```

if you run:

```bash
treeclip run \
  -e event/*_test.go \
  -e *.rs \
  -e *.toml
```

And `.toml` files exist in your current directory (like `Cargo.toml`, `cliff.toml`), your shell will expand `*.toml` to
all matching filenames, resulting in:

```bash
treeclip run \
  -e event/*_test.go \
  -e *.rs \
  -e Cargo.toml cliff.toml something.toml
```

This causes TreeClip to treat the expanded filenames as multiple positional arguments, leading to errors.

**Solutions:**

1. **Escape with quotes:**
   ```bash
   treeclip run \
    -e event/*_test.go \
    -e *.rs \
    -e '*.toml'
   ```

2. **Escape with backslashes:**
   ```bash
   treeclip run \
    -e event/*_test.go \
    -e *.rs \
    -e \*.toml
   ```

This only occurs when the wildcard matches files in the current directory. Patterns that don't match any files in the
current directory (like `event/*_test.go` when no such files exist in the current directory) will not be expanded and
will work as expected.

### Combine with Other Tools

```bash
# Count total lines in your project (treeclip already does that!)
treeclip run --fast-mode && wc -l treeclip_temp.txt

# Compare two versions of your code
treeclip run ./v1 -o v1.txt --fast-mode
treeclip run ./v2 -o v2.txt --fast-mode
diff v1.txt v2.txt
```

### Create Aliases

Add to your `.bashrc` or `.zshrc`:

```bash
# Quick clipboard copy
alias clip='treeclip run --clipboard --fast-mode'

# Review before sharing
alias clipr='treeclip run --clipboard --editor --delete'

# Full verbose snapshot
alias clips='treeclip run --clipboard --stats --verbose'
```

### Project-Specific Configurations

Keep a `.treeclipignore` in your project root:

```
# Dependencies
node_modules
target
vendor

# Build outputs
dist
build
out
*.min.js

# Logs and temp files
*.log
*.tmp
.DS_Store

# Don't share secrets!
.env
secrets.json
```

---

## Examples from Real Projects

### React Project

```bash
treeclip run ./src \
  --exclude node_modules \
  --exclude "*.test.js" \
  --clipboard
```

### Rust Project

```bash
treeclip run \
  --exclude target \
  --exclude Cargo.lock \
  --clipboard \
  --stats
```

### Python Project

```bash
treeclip run \
  --exclude __pycache__ \
  --exclude "*.pyc" \
  --exclude venv \
  --clipboard
```

---

## TODO (Future Plans) 🚧

- [ ] Configuration file support (`.treecliprc`)
- [ ] Interactive mode for selecting files
- [ ] Multiple output format support (JSON, Markdown, HTML)
- [ ] Token counting for AI models
- [ ] Smart exclusion patterns (auto-detect `.gitignore`)
- [ ] Streaming for huge projects
- [ ] Plugin system for custom processors
- [ ] Multiple inputs
- [ ] Commands and Options completion
- [ ] Add don't overwrite output file option
- [ ] Add tree option showing and writing a tree structure of traversed file(s)
- [ ] Optimize performance (use concurrency and parallelism)
- [ ] Add init option for basic init (.treeclipignore with basic init like .gitignore)
- [ ] Add link to existing ignore file ↑

But honestly? I built this to learn Rust and solve my immediate problem. If you find it useful, awesome! If you want to
contribute, even better! ♡

---

## Contributing

Found a bug? Have an idea? Want to make it cuter?

1. Fork the repo
2. Make your changes
3. Submit a PR with a description

I'm still learning Rust, so if you spot any anti-patterns or improvements, I'm all ears! (ﾉ◕ヮ◕)ﾉ*:･ﾟ✧

---

## License

[MIT License](./LICENSE) - feel free to use this however you want!

---

## Credits

Built with:

- Rust 🦀
- Intention of becoming a rustacean
- A genuine desire to stop copy-pasting code files

---

**Happy clipping!** ✨

*Made with ♡ (and a lot of Stack Overflow) by someone who just wanted a better way to share code with AI*
