# TreeClip Documentation

Welcome to the TreeClip documentation! TreeClip is a delightfully fast CLI tool that bundles your code into a single,
AI-friendly format.

## Overview

Ever tried explaining your entire codebase to an AI assistant, only to spend 20 minutes copy-pasting files? TreeClip
exists to solve that problem. It traverses your project directory, gathers all your code files, and bundles them into
one neat package with proper headers showing where each piece came from. It's like creating a "highlight reel" of your
project that AI models can actually digest in one go.

**Think of it as:** *Your project, but as a single, well-organized document that preserves all the context.*

## Quick Start

The most common use case is bundling the current directory and copying it to your clipboard:

```bash
treeclip run --clipboard
```

This command scans the current directory, creates a temporary file, copies its content to the clipboard, and shows a
friendly tree emoji animation.

## What's Inside This Documentation

- **[Getting Started](./getting-started.md):** A gentle introduction to TreeClip and its core concepts.
- **[Installation](./installation.md):** Detailed instructions on how to install TreeClip on your system.
- **[Usage](./usage.md):** Comprehensive guide to all commands, options, and common usage patterns.
- **[Implementation](./implementation/):** Deep dive into the internal architecture and modules of TreeClip.
- **[Examples](./examples/):** Practical examples for different project types and scenarios.
- **[Troubleshooting](./troubleshooting.md):** Solutions to common problems and error messages.
- **[FAQ](./faq.md):** Frequently asked questions and answers.

```
