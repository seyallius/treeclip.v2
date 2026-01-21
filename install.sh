#!/bin/bash
# install.sh - Simple installer for TreeClip

set -e

# -------------------------------------------- Public Functions --------------------------------------------

# Display help message
show_help() {
    cat << EOF
TreeClip Installer 🌳✨

Usage: $0 [OPTIONS]

Options:
  --help          Show this help message
  --version       Show version
  --prefix DIR    Install to custom directory (default: /usr/local/bin for Linux/macOS)
  --force         Force overwrite existing installation

Examples:
  $0                     # Install to default location
  $0 --prefix ~/.local   # Install to user directory
  $0 --force            # Force reinstall
EOF
}

# Main installation function
install_treeclip() {
    local prefix="${1:-/usr/local/bin}"
    local force="${2:-false}"
    local temp_dir=$(mktemp -d)
    
    echo "🌳 Installing TreeClip..."
    
    # Detect OS and architecture
    detect_platform
    
    # Download appropriate binary
    download_binary "$temp_dir"
    
    # Install to prefix
    install_binary "$temp_dir" "$prefix" "$force"
    
    # Cleanup
    rm -rf "$temp_dir"
    
    echo "✨ Installation complete! Try: treeclip --help"
}

# -------------------------------------------- Private Helper Functions --------------------------------------------

# Detect platform (OS + architecture)
detect_platform() {
    case "$(uname -s)" in
        Linux*)
            OS="linux"
            ;;
        Darwin*)
            OS="macos"
            ;;
        CYGWIN*|MINGW32*|MSYS*|MINGW*)
            OS="windows"
            ;;
        *)
            echo "❌ Unsupported OS: $(uname -s)"
            exit 1
            ;;
    esac
    
    case "$(uname -m)" in
        x86_64|amd64)
            ARCH="x86_64"
            ;;
        aarch64|arm64)
            ARCH="aarch64"
            ;;
        *)
            echo "❌ Unsupported architecture: $(uname -m)"
            exit 1
            ;;
    esac
}

# Download the appropriate binary
download_binary() {
    local temp_dir="$1"
    local version="latest"  # Could make this configurable
    
    echo "📦 Downloading TreeClip for $OS-$ARCH..."
    
    if [ "$OS" = "windows" ]; then
        local binary_name="treeclip-windows-x86_64.exe"
        local archive_name="${binary_name}.zip"
        local extract_cmd="unzip -q"
    else
        local binary_name="treeclip-$OS-$ARCH"
        local archive_name="${binary_name}.tar.gz"
        local extract_cmd="tar xzf"
    fi
    
    # Download from GitHub releases
    local download_url="https://github.com/seyallius/treeclip.v2/releases/$version/download/$archive_name"
    
    if ! curl -fsSL -o "$temp_dir/$archive_name" "$download_url"; then
        echo "❌ Failed to download TreeClip. Please check:"
        echo "   - Internet connection"
        echo "   - GitHub Releases page: https://github.com/seyallius/treeclip.v2/releases"
        exit 1
    fi
    
    # Extract
    cd "$temp_dir"
    $extract_cmd "$archive_name"
    
    if [ ! -f "$binary_name" ]; then
        echo "❌ Binary not found after extraction"
        exit 1
    fi
    
    # Make executable (Unix-like systems)
    if [ "$OS" != "windows" ]; then
        chmod +x "$binary_name"
    fi
}

# Install binary to target location
install_binary() {
    local temp_dir="$1"
    local prefix="$2"
    local force="$3"
    local binary_name
    
    if [ "$OS" = "windows" ]; then
        binary_name="treeclip-windows-x86_64.exe"
        local target_path="$prefix/treeclip.exe"
    else
        binary_name="treeclip-$OS-$ARCH"
        local target_path="$prefix/treeclip"
    fi
    
    # Create directory if it doesn't exist
    mkdir -p "$prefix"
    
    # Check if already exists
    if [ -f "$target_path" ] && [ "$force" = "false" ]; then
        echo "⚠️  TreeClip already exists at $target_path"
        echo "   Use --force to overwrite"
        return 1
    fi
    
    # Install
    cp "$temp_dir/$binary_name" "$target_path"
    echo "✅ Installed to: $target_path"
}

# Parse command line arguments
parse_args() {
    local prefix=""
    local force=false
    
    while [[ $# -gt 0 ]]; do
        case $1 in
            --help)
                show_help
                exit 0
                ;;
            --prefix)
                prefix="$2"
                shift 2
                ;;
            --force)
                force=true
                shift
                ;;
            *)
                echo "❌ Unknown option: $1"
                show_help
                exit 1
                ;;
        esac
    done
    
    install_treeclip "$prefix" "$force"
}

# Main execution
if [[ "${BASH_SOURCE[0]}" = "${0}" ]]; then
    parse_args "$@"
fi