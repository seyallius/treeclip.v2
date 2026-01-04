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

Now you can paste the entire project structure into your favorite AI chat! Easy peasy. (づ｡◕‿‿◕｡)づ

For comprehensive documentation, including detailed usage patterns, troubleshooting, and advanced features, please visit our [GitBook documentation](https://me0-42.gitbook.io/treeclip/).

---

<p align="center">
<sub> Made with ♡ by someone tired of copy-pasting code files! </sub>
</p>
