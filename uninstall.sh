#!/bin/bash

# KN Uninstallation Script

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

echo -e "${CYAN}🗑️  KN Uninstallation Script${NC}"
echo ""

# Detect possible installation locations
CARGO_BIN="$HOME/.cargo/bin/kn"
KN_DIR="$HOME/.kn"
KN_XDG="$XDG_DATA_HOME/.kn"
KN_MACOS="$HOME/Library/Application Support/.kn"
KN_LOCAL="$HOME/.local/share/.kn"

FOUND=false

# Remove from cargo bin
if [ -f "$CARGO_BIN" ]; then
    echo -e "${BLUE}Removing from Cargo bin...${NC}"
    rm -f "$CARGO_BIN"
    echo -e "${GREEN}✓ Removed $CARGO_BIN${NC}"
    FOUND=true
fi

# Remove installation directories
for DIR in "$KN_DIR" "$KN_XDG" "$KN_MACOS" "$KN_LOCAL"; do
    if [ -d "$DIR" ]; then
        echo -e "${BLUE}Removing directory: $DIR${NC}"
        rm -rf "$DIR"
        echo -e "${GREEN}✓ Removed $DIR${NC}"
        FOUND=true
    fi
done

# Clean up shell configurations
CURRENT_SHELL="$(basename "$SHELL")"

if [ "$CURRENT_SHELL" = "zsh" ]; then
    CONF_FILE=${ZDOTDIR:-$HOME}/.zshrc
    if grep -q "# kn" "$CONF_FILE" 2>/dev/null; then
        echo -e "${BLUE}Cleaning up $CONF_FILE...${NC}"
        sed -i.bak '/# kn/,/# kn end/d' "$CONF_FILE"
        rm -f "${CONF_FILE}.bak"
        echo -e "${GREEN}✓ Cleaned $CONF_FILE${NC}"
    fi

elif [ "$CURRENT_SHELL" = "fish" ]; then
    CONF_FILE=$HOME/.config/fish/conf.d/kn.fish
    if [ -f "$CONF_FILE" ]; then
        echo -e "${BLUE}Removing $CONF_FILE...${NC}"
        rm -f "$CONF_FILE"
        echo -e "${GREEN}✓ Removed $CONF_FILE${NC}"
    fi

elif [ "$CURRENT_SHELL" = "bash" ]; then
    for CONF_FILE in "$HOME/.bashrc" "$HOME/.bash_profile" "$HOME/.profile"; do
        if grep -q "# kn" "$CONF_FILE" 2>/dev/null; then
            echo -e "${BLUE}Cleaning up $CONF_FILE...${NC}"
            sed -i.bak '/# kn/,/# kn end/d' "$CONF_FILE"
            rm -f "${CONF_FILE}.bak"
            echo -e "${GREEN}✓ Cleaned $CONF_FILE${NC}"
        fi
    done
fi

# Remove config and data files
if [ -d "$HOME/.config/kn" ]; then
    echo -e "${BLUE}Removing config directory...${NC}"
    rm -rf "$HOME/.config/kn"
    echo -e "${GREEN}✓ Removed $HOME/.config/kn${NC}"
fi

if [ -d "$HOME/.tmp/kn" ]; then
    echo -e "${BLUE}Removing data directory...${NC}"
    rm -rf "$HOME/.tmp/kn"
    echo -e "${GREEN}✓ Removed $HOME/.tmp/kn${NC}"
fi

echo ""

if [ "$FOUND" = true ]; then
    echo -e "${GREEN}✨ KN has been successfully uninstalled${NC}"
    echo ""
    echo -e "${YELLOW}Note: You may need to restart your terminal or run:${NC}"
    echo -e "  ${BLUE}source ~/.zshrc${NC}    # for zsh"
    echo -e "  ${BLUE}source ~/.bashrc${NC}   # for bash"
else
    echo -e "${YELLOW}⚠️  KN installation not found${NC}"
    echo -e "${BLUE}Nothing to uninstall.${NC}"
fi

echo ""
