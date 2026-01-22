#!/bin/bash

# # KN Installation Script

set -e

echo "🚀 Installing KN..."

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo is not installed. Please install Rust first:"
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# Install using cargo
echo "📦 Installing via cargo..."
cargo install kn

# Create convenience symlinks
echo "🔗 Creating convenience symlinks..."

# Determine bin directory
BIN_DIR="$HOME/.cargo/bin"
if [ ! -d "$BIN_DIR" ]; then
    BIN_DIR="/usr/local/bin"
    if [ ! -w "$BIN_DIR" ]; then
        BIN_DIR="$HOME/.local/bin"
        mkdir -p "$BIN_DIR"
    fi
fi

# Create symlinks for k commands
cd "$BIN_DIR"
if [ -f "kn" ]; then
    echo "✅ Created symlinks:"
    echo "   kn  - Main command (already exists)"
else
    echo "✅ Already available as 'kn'"
fi

# Add to PATH if needed
if [[ ":$PATH:" != *":$BIN_DIR:"* ]]; then
    echo "⚠️  Add $BIN_DIR to your PATH:"
    echo "   export PATH=\"\$PATH:$BIN_DIR\""
    echo "   Add this to your ~/.bashrc or ~/.zshrc"
fi

echo ""
echo "🎉 KN installed successfully!"
echo ""
echo "📖 Quick Start:"
echo "   kn                   # Show help"
echo "   kn i react           # Install react"
echo "   kn r dev             # Run dev script"  
echo "   kn r                 # List all scripts"
echo "   kn uninstall webpack  # Uninstall webpack"
echo ""
echo "📚 Documentation: https://github.com/wangsizhu0504/kn"