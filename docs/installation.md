# Installation

Learn how to install TreeClip on your system.

## Prerequisites

TreeClip is a Rust application. To install it, you need to have [Rust](https://www.rust-lang.org/tools/install)
installed on your system, which includes the `cargo` package manager.

## Installing from Crates.io (Recommended)

The easiest way to install TreeClip is directly from [crates.io](https://crates.io/crates/treeclip):

```bash
cargo install treeclip
```

This command will download the latest version of TreeClip, compile it, and install the `treeclip` binary to your
system's `$PATH` (typically `~/.cargo/bin` on Unix-like systems). After installation, you can run `treeclip` from any
terminal.

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

   ```bash
   cargo build --release
   ```

   The compiled binary will be located at `target/release/treeclip`. You can run it directly from there (
   `./target/release/treeclip --help`) or copy it to a directory in your `$PATH` for global access.

## Verification

To verify the installation, open a new terminal (or source your shell profile if needed) and run:

```bash
treeclip --version
```

This should print the version of TreeClip installed on your system.
