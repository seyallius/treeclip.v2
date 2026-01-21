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

## Documentation 📚

For comprehensive documentation, please visit our [GitBook documentation](https://me0-42.gitbook.io/treeclip/).

## Installation 🚀

### From Crates.io (Recommended)

You can install `treeclip` directly from crates.io using Cargo:

[![Crates.io](https://img.shields.io/crates/v/treeclip.svg)](https://crates.io/crates/treeclip)

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

```bash
# Bundle the current directory and copy it to the clipboard
treeclip run --clipboard
```

### Curl

You can install TreeClip with a single command:

Unix-like systems (Linux/macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/seyallius/treeclip.v2/main/install.sh | bash

# Or with custom options
curl -fsSL https://raw.githubusercontent.com/seyallius/treeclip.v2/main/install.sh | bash -s -- --prefix ~/.local/bin
```

Windows (PowerShell)

```bash
iwr -useb https://raw.githubusercontent.com/seyallius/treeclip.v2/main/install.ps1 | iex
```

### Manual Installation from GitHub Releases

Visit the [GitHub Releases](https://github.com/seyallius/treeclip.v2/releases) page and download the appropriate binary
for your system:

| Platform              | File                              | Instructions                                                           |
|-----------------------|-----------------------------------|------------------------------------------------------------------------|
| Linux (x86_64)        | `treeclip-linux-x86_64.tar.gz`    | `tar xzf treeclip-linux-x86_64.tar.gz && mv treeclip /usr/local/bin/`  |
| Linux (ARM64)         | `treeclip-linux-aarch64.tar.gz`   | `tar xzf treeclip-linux-aarch64.tar.gz && mv treeclip /usr/local/bin/` |
| macOS (Intel)         | `treeclip-macos-x86_64.tar.gz`    | `tar xzf treeclip-macos-x86_64.tar.gz && mv treeclip /usr/local/bin/`  |
| macOS (Apple Silicon) | `treeclip-macos-aarch64.tar.gz`   | `tar xzf treeclip-macos-aarch64.tar.gz && mv treeclip /usr/local/bin/` |
| Windows               | `treeclip-windows-x86_64.exe.zip` | Unzip and move `treeclip.exe` to your PATH                             |

### Package Managers (Future)

We're working on adding TreeClip to popular package managers:

```bash
# Coming soon!
# brew install treeclip           # macOS
# apt install treeclip            # Ubuntu/Debian
# yum install treeclip            # RHEL/Fedora
# choco install treeclip          # Windows (Chocolatey)
```

Now you can paste the entire project structure into your favorite AI chat! Easy peasy. (づ｡◕‿‿◕｡)づ

For comprehensive documentation, including detailed usage patterns, troubleshooting, and advanced features, please visit
our [GitBook documentation](https://me0-42.gitbook.io/treeclip/).

## TODO (Future Plans) 🚧

- [ ] Configuration file support (`.treecliprc`)
- [ ] Interactive mode for selecting files
- [ ] Multiple output format support (JSON, Markdown, HTML)
- [ ] Token counting for AI models
- [x] Smart exclusion patterns (auto-detect `.gitignore`)
- [ ] Streaming for huge projects
- [ ] Plugin system for custom processors
- [x] Multiple inputs
- [ ] Commands and Options completion
- [ ] Add don't overwrite output file option
- [x] Add tree option showing and writing a tree structure of traversed file(s)
- [x] Optimize performance (use concurrency and parallelism)
- [ ] Add init option for basic init (.treeclipignore with basic init like .gitignore)
- [x] Add link to existing ignore file ↑ (done in different way - reads already present ignore files)
- [ ] Make it not depndant to cargo and on new release, also release pre-built binaries for multiple OSes

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

<p align="center">
<sub> Made with ♡ by someone tired of copy-pasting code files! </sub>
</p>
