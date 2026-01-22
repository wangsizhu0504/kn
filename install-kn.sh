#!/bin/bash

# Installation script for kn (Node.js package manager)

set -e

echo "🚀 Installing kn - Minimal, blazing fast Node.js package manager"
echo "================================================================="

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BINARY_PATH="$SCRIPT_DIR/target/release/kn"

# Check if binary exists
if [ ! -f "$BINARY_PATH" ]; then
    echo "🔨 Building kn..."
    cd "$SCRIPT_DIR"
    if command -v cargo &> /dev/null; then
        cargo build --release
    else
        echo "❌ Rust/Cargo not found! Please install Rust first:"
        echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        exit 1
    fi
fi

# Check if build was successful
if [ ! -f "$BINARY_PATH" ]; then
    echo "❌ Build failed! Please check your Rust installation and try again."
    exit 1
fi

# Determine installation directory
if [ -w "/usr/local/bin" ]; then
    INSTALL_DIR="/usr/local/bin"
    USE_SUDO=false
else
    INSTALL_DIR="/usr/local/bin"
    USE_SUDO=true
fi

# Install kn
echo "📦 Installing kn to $INSTALL_DIR..."
if [ "$USE_SUDO" = true ]; then
    echo "🔐 Using sudo for installation..."
    sudo cp "$BINARY_PATH" "$INSTALL_DIR/kn"
    sudo chmod +x "$INSTALL_DIR/kn"
else
    cp "$BINARY_PATH" "$INSTALL_DIR/kn"
    chmod +x "$INSTALL_DIR/kn"
fi

echo "✅ kn installed successfully!"

# Verify installation
echo ""
echo "🧪 Verifying installation..."

# Refresh PATH in case installation dir wasn't in PATH
export PATH="$PATH:$INSTALL_DIR"

if command -v kn &> /dev/null; then
    echo "✅ kn is now available in your PATH!"
    echo ""
    echo "📋 Version check:"
    kn --version
    echo ""
    echo "🎉 Installation completed successfully!"
    echo ""
    echo "📚 Quick Start:"
    echo "  kn --help           # Show all commands"
    echo "  kn i <package>     # Install packages"
    echo "  kn run <script>    # Run npm scripts"
    echo "  kn list            # List available scripts"
    echo "  kn x <command>    # Execute package binaries"
    echo "  kn info            # Show package manager info"
    echo ""
    echo "🧪 Test installation:"
    echo "  kn list            # Test in a Node.js project"
    echo ""
    echo "📖 For full documentation: https://github.com/your-username/kn"
else
    echo "❌ Installation verification failed!"
    echo "Please make sure $INSTALL_DIR is in your PATH."
    echo ""
    echo "Add this to your shell profile (~/.zshrc, ~/.bashrc):"
    echo "  export PATH=\"\$PATH:$INSTALL_DIR\""
    echo ""
    echo "Then restart your terminal or run: source ~/.zshrc"
    exit 1
fi