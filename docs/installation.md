# Installation

Learn how to install TreeClip on your system.

## Quick Install (All Platforms)

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

## Manual Installation from GitHub Releases

Visit the [GitHub Releases](https://github.com/seyallius/treeclip.v2/releases) page and download the appropriate binary
for your system:

| Platform              | File                              | Instructions                                                           |
|-----------------------|-----------------------------------|------------------------------------------------------------------------|
| Linux (x86_64)        | `treeclip-linux-x86_64.tar.gz`    | `tar xzf treeclip-linux-x86_64.tar.gz && mv treeclip /usr/local/bin/`  |
| Linux (ARM64)         | `treeclip-linux-aarch64.tar.gz`   | `tar xzf treeclip-linux-aarch64.tar.gz && mv treeclip /usr/local/bin/` |
| macOS (Intel)         | `treeclip-macos-x86_64.tar.gz`    | `tar xzf treeclip-macos-x86_64.tar.gz && mv treeclip /usr/local/bin/`  |
| macOS (Apple Silicon) | `treeclip-macos-aarch64.tar.gz`   | `tar xzf treeclip-macos-aarch64.tar.gz && mv treeclip /usr/local/bin/` |
| Windows               | `treeclip-windows-x86_64.exe.zip` | Unzip and move `treeclip.exe` to your PATH                             |

## Package Managers (Future)

We're working on adding TreeClip to popular package managers:

```bash
# Coming soon!
# brew install treeclip           # macOS
# apt install treeclip            # Ubuntu/Debian
# yum install treeclip            # RHEL/Fedora
# choco install treeclip          # Windows (Chocolatey)
```

## From Crates.io (Rust Developers)

If you have Rust installed, you can still use:

```bash
cargo install treeclip
```

## Installing from Source

If you prefer to build TreeClip from the source code, follow these steps:

1. **Clone the repository:**

   ```bash
   git clone https://github.com/seyallius/treeclip.v2.git
   cd treeclip.v2
   ```

2. **Build and install:**

   You can build and install the release version directly:

   ```bash
   cargo install --path .
   ```

   This will compile the project in release mode and install the `treeclip` binary to your `$PATH`, similar to the
   crates.io method.

   Alternatively, you can build the binary manually:
