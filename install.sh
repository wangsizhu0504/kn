#!/bin/bash

# KN Installation Script
# Installs kn - A minimal, blazing fast Node.js package manager and scripts runner

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Emojis
ROCKET="🚀"
PACKAGE="📦"
CHECK="✅"
CROSS="❌"
WRENCH="🔧"
SPARKLE="✨"

echo -e "${CYAN}${ROCKET} KN Installation Script${NC}"
echo ""

# Detect OS and Architecture
OS_TYPE="$(uname)"
ARCH="$(uname -m)"

echo -e "${BLUE}Operating System: ${YELLOW}$OS_TYPE${NC}"
echo -e "${BLUE}Architecture: ${YELLOW}$ARCH${NC}"
echo ""

# Determine filename based on OS and architecture
if [[ "$OS_TYPE" == "Linux" ]]; then
    if [[ "$ARCH" == "x86_64" ]]; then
        FILENAME="kn-linux-x86_64.tar.gz"
    elif [[ "$ARCH" == "aarch64" ]] || [[ "$ARCH" == "arm64" ]]; then
        FILENAME="kn-linux-aarch64.tar.gz"
    else
        echo -e "${CROSS} ${RED}Unsupported Linux architecture: $ARCH${NC}"
        exit 1
    fi
elif [[ "$OS_TYPE" == "Darwin" ]]; then
    if [[ "$ARCH" == "x86_64" ]]; then
        FILENAME="kn-macos-x86_64.tar.gz"
    elif [[ "$ARCH" == "arm64" ]]; then
        FILENAME="kn-macos-aarch64.tar.gz"
    else
        FILENAME="kn-macos-x86_64.tar.gz"
        echo -e "${YELLOW}Unknown macOS Architecture: $ARCH, using x86_64${NC}"
    fi
elif [[ "$OS_TYPE" == "Windows_NT" ]] || [[ "$OS_TYPE" == MINGW* ]] || [[ "$OS_TYPE" == MSYS* ]]; then
    FILENAME="kn-windows-x86_64.zip"
else
    echo -e "${CROSS} ${RED}Unknown Operating System: $OS_TYPE${NC}"
    exit 1
fi

# GitHub release URL
REPO_OWNER="wangsizhu0504"
REPO_NAME="kn"
ARCHIVE_URL="https://github.com/${REPO_OWNER}/${REPO_NAME}/releases/latest/download/$FILENAME"

echo -e "${PACKAGE} ${BLUE}Downloading from: ${YELLOW}$ARCHIVE_URL${NC}"
echo ""

# Create temporary download directory
DOWNLOAD_DIR=$(mktemp -d)
TEMP_FILE="$DOWNLOAD_DIR/kn-archive"

# Determine installation directory
if [ -d "$HOME/.kn" ]; then
    INSTALL_DIR="$HOME/.kn"
elif [ -n "$XDG_DATA_HOME" ]; then
    INSTALL_DIR="$XDG_DATA_HOME/.kn"
elif [ "$OS_TYPE" = "Darwin" ]; then
    INSTALL_DIR="$HOME/Library/Application Support/.kn"
else
    INSTALL_DIR="$HOME/.local/share/.kn"
fi

# Create bin directory
BIN_DIR="$INSTALL_DIR/bin"

echo -e "${BLUE}Installation directory: ${YELLOW}$INSTALL_DIR${NC}"
echo ""

# Download the archive
echo -e "${PACKAGE} ${CYAN}Downloading kn...${NC}"
if command -v curl &> /dev/null; then
    curl -fsSL "$ARCHIVE_URL" -o "$TEMP_FILE"
elif command -v wget &> /dev/null; then
    wget -q -O "$TEMP_FILE" "$ARCHIVE_URL"
else
    echo -e "${CROSS} ${RED}Neither curl nor wget is available. Please install one of them.${NC}"
    exit 1
fi

if [ $? -ne 0 ]; then
    echo -e "${CROSS} ${RED}Failed to download kn.${NC}"
    echo -e "${YELLOW}Note: Pre-built binaries might not be available yet.${NC}"
    echo -e "${YELLOW}Please build from source using: cargo install --git https://github.com/${REPO_OWNER}/${REPO_NAME}${NC}"
    rm -rf "$DOWNLOAD_DIR"
    exit 1
fi

echo -e "${CHECK} ${GREEN}Download completed${NC}"
echo ""

# Extract archive
echo -e "${WRENCH} ${CYAN}Extracting...${NC}"
if [[ "$FILENAME" == *.tar.gz ]]; then
    tar -xzf "$TEMP_FILE" -C "$DOWNLOAD_DIR"
elif [[ "$FILENAME" == *.zip ]]; then
    unzip -q "$TEMP_FILE" -d "$DOWNLOAD_DIR"
fi

# Create installation directory
if [ ! -d "$BIN_DIR" ]; then
    mkdir -p "$BIN_DIR"
fi

# Move binary to installation directory
if [ -f "$DOWNLOAD_DIR/kn" ]; then
    mv "$DOWNLOAD_DIR/kn" "$BIN_DIR/kn"
    chmod +x "$BIN_DIR/kn"
elif [ -f "$DOWNLOAD_DIR/kn.exe" ]; then
    mv "$DOWNLOAD_DIR/kn.exe" "$BIN_DIR/kn.exe"
else
    echo -e "${CROSS} ${RED}Binary not found in archive${NC}"
    rm -rf "$DOWNLOAD_DIR"
    exit 1
fi

# Clean up
rm -rf "$DOWNLOAD_DIR"

echo -e "${CHECK} ${GREEN}Installation completed${NC}"
echo ""

# Helper function to ensure directory exists
ensure_containing_dir_exists() {
    local CONTAINING_DIR
    CONTAINING_DIR="$(dirname "$1")"
    if [ ! -d "$CONTAINING_DIR" ]; then
        echo -e "  ${BLUE}Creating directory $CONTAINING_DIR${NC}"
        mkdir -p "$CONTAINING_DIR"
    fi
}

# Setup shell configuration
setup_shell() {
    CURRENT_SHELL="$(basename "$SHELL")"

    echo -e "${WRENCH} ${CYAN}Setting up shell environment for: ${YELLOW}$CURRENT_SHELL${NC}"
    echo ""

    if [ "$CURRENT_SHELL" = "zsh" ]; then
        CONF_FILE=${ZDOTDIR:-$HOME}/.zshrc
        ensure_containing_dir_exists "$CONF_FILE"

        # Check if already configured
        if grep -q "# kn" "$CONF_FILE" 2>/dev/null; then
            echo -e "${YELLOW}kn is already configured in $CONF_FILE${NC}"
        else
            echo -e "${BLUE}Appending configuration to ${YELLOW}$CONF_FILE${NC}"
            echo "" >> "$CONF_FILE"
            echo "# kn" >> "$CONF_FILE"
            echo "export PATH=\"$BIN_DIR:\$PATH\"" >> "$CONF_FILE"
            echo "# kn end" >> "$CONF_FILE"
        fi

    elif [ "$CURRENT_SHELL" = "fish" ]; then
        CONF_FILE=$HOME/.config/fish/conf.d/kn.fish
        ensure_containing_dir_exists "$CONF_FILE"

        if [ -f "$CONF_FILE" ]; then
            echo -e "${YELLOW}kn is already configured in $CONF_FILE${NC}"
        else
            echo -e "${BLUE}Creating Fish configuration: ${YELLOW}$CONF_FILE${NC}"
            echo "# kn" > "$CONF_FILE"
            echo "set -gx PATH \"$BIN_DIR\" \$PATH" >> "$CONF_FILE"
            echo "# kn end" >> "$CONF_FILE"
        fi

    elif [ "$CURRENT_SHELL" = "bash" ]; then
        if [ "$OS_TYPE" = "Darwin" ]; then
            CONF_FILE=$HOME/.bash_profile
        else
            CONF_FILE=$HOME/.bashrc
        fi
        ensure_containing_dir_exists "$CONF_FILE"

        if grep -q "# kn" "$CONF_FILE" 2>/dev/null; then
            echo -e "${YELLOW}kn is already configured in $CONF_FILE${NC}"
        else
            echo -e "${BLUE}Appending configuration to ${YELLOW}$CONF_FILE${NC}"
            echo "" >> "$CONF_FILE"
            echo "# kn" >> "$CONF_FILE"
            echo "export PATH=\"$BIN_DIR:\$PATH\"" >> "$CONF_FILE"
            echo "# kn end" >> "$CONF_FILE"
        fi

    else
        echo -e "${YELLOW}Could not detect shell type. Please manually add the following to your shell configuration:${NC}"
        echo ""
        echo -e "  export PATH=\"$BIN_DIR:\$PATH\""
        echo ""
        return
    fi

    echo ""
    echo -e "${SPARKLE} ${GREEN}Shell configuration completed!${NC}"
    echo ""
    echo -e "${CYAN}To apply the changes, run:${NC}"
    echo ""
    echo -e "  ${YELLOW}source $CONF_FILE${NC}"
    echo ""
    echo -e "${CYAN}Or simply open a new terminal window.${NC}"
}

setup_shell

echo ""
echo -e "${SPARKLE} ${GREEN}KN has been successfully installed!${NC}"
echo ""
echo -e "${CYAN}Quick Start:${NC}"
echo -e "  ${BLUE}kn install${NC}         # Install dependencies"
echo -e "  ${BLUE}kn run dev${NC}         # Run dev script"
echo -e "  ${BLUE}kn --help${NC}          # Show all commands"
echo ""
echo -e "${CYAN}For more information, visit:${NC} ${YELLOW}https://github.com/${REPO_OWNER}/${REPO_NAME}${NC}"
echo ""

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
