<div align="center">

# ⚡ KN - Blazing Fast Node.js Package Manager CLI

**A modern, intelligent command-line tool for Node.js package management and script execution**

[![Rust](https://img.shields.io/badge/Rust-2021-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-macOS%20%7C%20Linux%20%7C%20Windows-lightgrey.svg)](https://github.com/wangsizhu0504/kn)

[Features](#-features) • [Installation](#-installation) • [Usage](#-usage) • [Commands](#-commands) • [Contributing](#-contributing)

</div>

---

## 🎯 What is KN?

**KN** is a lightweight, high-performance CLI tool written in Rust that simplifies Node.js package management and script execution. It automatically detects your package manager (npm, yarn, pnpm, bun) and provides an enhanced developer experience with intelligent features like fuzzy search, and performance tracking.

---

## ✨ Features

### Core Capabilities

#### 📦 **Smart Package Management**
- Automatic package manager detection (npm/yarn/pnpm/bun)
- Support for all standard operations: install, uninstall, upgrade
- Enhanced install with progress tracking and time statistics
- Clean install with frozen lockfile support

#### 🏃 **Enhanced Script Execution**
- Fast script running with performance tracking
- Fuzzy matching for script names (typo-tolerant)
- Interactive script selector when no name provided
- Parallel execution of multiple scripts
- Watch mode with file change detection

#### 🛠️ **Developer Tools**
- **`kn doctor`** - Comprehensive project health check
  - Security vulnerability scan
  - Duplicate dependency detection
  - Node.js version compatibility
  - Lock file validation

- **`kn size`** - Disk usage analysis
  - Visual breakdown of package sizes
  - Identifies large dependencies
  - Total size statistics

- **`kn completion`** - Shell auto-completion
  - Support for bash, zsh, and fish
  - Command and script name completion

### Advanced Features

####  **Performance Statistics**
- Automatic tracking of script execution times
- Run count and frequency analysis
- Last run timestamps
- Performance insights with `kn stats`

#### ⚡ **Parallel Execution**
```bash
kn parallel lint test build  # Run multiple scripts concurrently
kn p dev test                # Short alias
```

#### 🧹 **Smart Cleanup**
```bash
kn clean              # Remove node_modules, dist, etc.
kn clean --cache      # Clean package manager cache
kn clean --all        # Deep clean everything
```

#### 👀 **Watch Mode**
```bash
kn watch dev         # Auto-rerun on file changes
```

#### 📊 **Dependency Analysis**
```bash
kn analyze           # Analyze project dependencies
                     # - Dependency tree
                     # - Disk usage
                     # - Outdated packages
                     # - Duplicate detection
```

---

## 🚀 Installation

### From Source (Recommended)

```bash
# Clone the repository
git clone https://github.com/wangsizhu0504/kn.git
cd kn

# Build and install
cargo build --release
cargo install --path .
```

### Using Installation Script

```bash
# Clone and run install script
git clone https://github.com/wangsizhu0504/kn.git
cd kn
chmod +x install.sh
./install.sh
```

### Manual Build

```bash
cargo build --release
# Binary will be in ./target/release/kn
```

---

## 📖 Usage

### Quick Start

```bash
# Install dependencies
kn install

# Run a script
kn run dev

# Show available commands
kn help
```

### Basic Commands

#### Package Management
```bash
kn install <package>           # Install package(s)
kn i <package>                 # Short alias
kn install <pkg> -D            # Install as dev dependency
kn install <pkg> -g            # Install globally
kn install                     # Install all dependencies

kn uninstall <package>         # Remove package(s)
kn rm <package>                # Short alias

kn upgrade                     # Update all dependencies
kn upgrade <package>           # Update specific package
kn update                      # Alias for upgrade

kn clean-install               # Clean install (like npm ci)
kn ci                          # Short alias
```

#### Script Execution
```bash
kn run <script>                # Run npm script
kn r <script>                  # Short alias
kn run                         # Interactive script selector

kn execute <command>           # Execute package binary
kn x <command>                 # Short alias
```

#### Information & Analysis
```bash
kn list                        # List all scripts (dashboard view)
kn ls                          # Short alias

kn info                        # Show package manager info
kn doctor                      # Project health check
kn size                        # Analyze package sizes
kn stats                       # Show performance statistics
kn analyze                     # Dependency analysis
```

#### Productivity Features
```bash
# Parallel execution
kn parallel <script1> <script2>  # Run scripts in parallel
kn p <script1> <script2>         # Short alias

# Watch mode
kn watch <script>              # Watch and auto-rerun

# Cleanup
kn clean                       # Clean project files
kn clean --cache               # Clean PM cache
kn clean --all                 # Deep clean

# Shell completion
kn completion bash             # Generate bash completion
kn completion zsh              # Generate zsh completion
kn completion fish             # Generate fish completion
```

---

## 🎯 Commands

### Core Commands

| Command | Aliases | Description |
|---------|---------|-------------|
| `install` | `i`, `add` | Install dependencies |
| `uninstall` | `rm`, `remove` | Remove dependencies |
| `run` | `r` | Run npm scripts |
| `execute` | `x`, `exec` | Execute package binaries |
| `upgrade` | `update`, `up` | Update dependencies |
| `clean-install` | `ci` | Clean install with lockfile |

### Information Commands

| Command | Aliases | Description |
|---------|---------|-------------|
| `list` | `ls` | List available scripts |
| `info` | `env` | Show package manager info |
| `doctor` | - | Project health check |
| `size` | - | Analyze package sizes |
| `stats` | - | Show performance stats |
| `analyze` | - | Dependency analysis |

### Productivity Commands

| Command | Aliases | Description |
|---------|---------|-------------|
| `parallel` | `p` | Run scripts in parallel |
| `watch` | `w` | Watch mode |
| `clean` | - | Clean project files |
| `completion` | - | Shell auto-completion |

### Utility Commands

| Command | Aliases | Description |
|---------|---------|-------------|
| `agent` | - | Use package manager directly |
| `help` | `-h`, `--help` | Show help information |
| `--version` | `-v` | Show version |

---

## � Command Details

### Package Management

#### `kn install [packages...] [options]`
Install one or more packages with progress tracking.

**Options:**
- `-D, --save-dev` - Install as dev dependency
- `-g, --global` - Install globally
- `-E, --save-exact` - Install exact version

**Examples:**
```bash
kn install react typescript    # Install dependencies
kn i lodash -D                 # Install dev dependency
kn install                     # Install all from package.json
```

#### `kn uninstall <packages...> [options]`
Remove one or more packages.

**Options:**
- `-g, --global` - Uninstall globally

**Examples:**
```bash
kn uninstall webpack           # Remove package
kn rm old-package              # Using alias
```

#### `kn upgrade [packages...] [options]`
Update dependencies to latest versions.

**Options:**
- `-i, --interactive` - Interactive selection
- `-L, --latest` - Update to latest version

**Examples:**
```bash
kn upgrade                     # Update all
kn upgrade react               # Update specific package
```

#### `kn clean-install`
Clean install from lockfile (like `npm ci`).

**Examples:**
```bash
kn clean-install              # Fresh install
kn ci                         # Using alias
```

### Script Execution

#### `kn run [script] [args...]`
Run npm scripts with enhanced features.

**Features:**
- Interactive selection when no script specified
- Fuzzy matching for script names
- Performance tracking
- Shows run statistics

**Examples:**
```bash
kn run dev                    # Run dev script
kn r build -- --watch         # Run with arguments
kn run                        # Interactive selector
```

#### `kn execute <command> [args...]`
Execute package binaries directly.

**Examples:**
```bash
kn execute tsc                # Run TypeScript compiler
kn x eslint src/              # Using alias
```

#### `kn parallel <script1> <script2> ...`
Run multiple scripts in parallel.

**Examples:**
```bash
kn parallel lint test build  # Run 3 scripts
kn p dev test                # Using alias
```

#### `kn watch <script> [patterns...]`
Watch files and auto-rerun script on changes.

**Examples:**
```bash
kn watch dev                 # Watch and rerun
kn w test src/               # Watch specific path
```

### Information & Analysis

#### `kn list`
Display all available scripts in dashboard format.

**Examples:**
```bash
kn list                      # Show all scripts
kn ls                        # Using alias
```

#### `kn info [--verbose]`
Show package manager and environment information.

**Options:**
- `-v, --verbose` - Show detailed information

**Examples:**
```bash
kn info                      # Basic info
kn info -v                   # Detailed info
```

#### `kn doctor`
Comprehensive project health check.

**Checks:**
- package.json validation
- Dependencies installation status
- Security vulnerabilities (npm audit)
- Node.js version compatibility
- Lock file consistency
- Duplicate dependencies

**Examples:**
```bash
kn doctor                    # Run health check
```

#### `kn size`
Analyze disk usage of installed packages.

**Features:**
- Shows top 20 largest packages
- Visual size bars
- Total size statistics
- Large package warnings

**Examples:**
```bash
kn size                      # Analyze package sizes
```

#### `kn stats`
Show script execution performance statistics.

**Examples:**
```bash
kn stats                     # Show all stats
```

#### `kn analyze`
Analyze project dependencies and structure.

**Examples:**
```bash
kn analyze                   # Dependency analysis
```

### Productivity

#### `kn clean [options]`
Clean project files and caches.

**Options:**
- `--cache` - Clean package manager cache
- `--all` - Deep clean (project + cache)
- `--global` - Clean global cache

**Examples:**
```bash
kn clean                     # Clean project
kn clean --cache             # Clean cache only
kn clean --all               # Deep clean
```

#### `kn completion <shell>`
Generate shell completion scripts.

**Supported Shells:**
- `bash`
- `zsh`
- `fish`

**Examples:**
```bash
kn completion bash           # Generate bash completion
kn completion fish > ~/.config/fish/completions/kn.fish
```

**Installation:**
```bash
# Bash (~/.bashrc)
eval "$(kn completion bash)"

# Zsh (~/.zshrc)
eval "$(kn completion zsh)"

# Fish
kn completion fish > ~/.config/fish/completions/kn.fish
```

---

## �🔍 Examples

### Package Management with Progress

```bash
$ kn install react typescript -D

Installing Packages
-----------------------

ℹ️ Installing 2 package(s) as dev dependencies
  1. react
  2. typescript

Using npm
---------

  ⠋ Installing packages...
📊 Script Performance Statistics

  Script                   Runs     Avg Time     Last Run
  ────────────────────────────────────────────────────────────
  dev                         15       125ms     2026-01-22
  test                        42        15ms     2026-01-22
  build                        8        2.5s     2026-01-22
```

### ⚡ Parallel Execution

Run multiple scripts simultaneously:

```bash
$ kn parallel lint test build

⚡ Running 3 scripts in parallel...

[1] Starting lint
[2] Starting test
[3] Starting build
[2] ✓ test (0.15s)
[1] ✓ lint (0.28s)
[3] ✓ build (2.5s)

📊 Summary
  ✓ Successful: 3
  Total time: 2.5s
  Time saved: ~0.43s
```

### 🧹 Smart Cleanup

Clean your project efficiently:

```bash
$ kn clean

🧹 Cleaning local project...

  ✓ Removed node_modules
  ✓ Removed dist
  ✓ Removed .next
  ✓ Removed .turbo

✓ Cleaned 4 directories, freed ~245 MB

$ kn clean --cache     # Clean package manager cache
$ kn clean --all       # Deep clean everything
```

### 📊 Dependency Analysis

Analyze your project dependencies:

```bash
$ kn analyze

📊 Analyzing project dependencies...

  Dependencies Overview
  ├─ Production: 25
  ├─ Development: 18
  └─ Total: 43

  Disk Usage
  └─ node_modules: 245 MB

  Outdated Packages
  └─ 5 packages need updates
     Run 'kn upgrade' to update them

  Duplicate Packages
  └─ ✓ No duplicates detected
```

### 👀 Watch Mode

Monitor files and auto-rerun scripts:

```bash
$ kn watch dev

▶ Running script: dev
─────────────────────────────────────────
Server started on http://localhost:3000

⟳ Change detected, re-running...
─────────────────────────────────────────
Server restarted on http://localhost:3000
```

### 🎯 Smart Command Suggestions

Get helpful suggestions for typos:

```bash
$ kn instal react
❌ Unknown command: instal

💡 Did you mean:
   • install
   • uninstall
```

### Package Manager Auto-Detection

kn automatically detects your package manager based on:

1. **Lock Files** (in order of preference):
   - `pnpm-lock.yaml` → pnpm
   - `yarn.lock` → yarn
   - `package-lock.json` → npm
   - `bun.lockb` / `bun.lock` → bun

2. **package.json field**:
   - `"packageManager": "yarn@4.0.0"` → YarnBerry
   - `"packageManager": "pnpm@6.0.0"` → Pnpm6

### Script Listing

```bash
$ kn list
my-project@1.0.0
start   npm run start
build   npm run build
test    npm run test
```

### Package Manager Information

```bash
$ kn info

Package Manager Information
───────────────────────────────
📦 Package Manager     11.6.2 (npm)
▸ Lock File Analysis
────────────────────
✅ Found matching lock file: package-lock.json
▸ Runtime Information
──────────────────────

### Configuration File

Create a `~/.knrc` file to configure defaults:

```ini
default_agent = npm    # fallback when no lock found
global_agent = npm      # for global installs
```

You can also set a custom config path:
```bash
export KN_CONFIG_FILE="$HOME/.config/kn/knrc"
```

### Persistent Data

kn stores persistent data in `~/.tmp/kn/_storage.json`:
- Performance statistics

This data persists across sessions and is automatically managed.ini
default_agent = npm    # fallback when no lock found
global_agent = npm      # for global installs

## 🎨 Command Overview

| Category | Commands | Description |
|----------|----------|-------------|
| **Package Management** | `install`, `uninstall`, `upgrade`, `clean-install` | Manage dependencies |
| **Script Execution** | `run`, `execute`, `watch`, `parallel` | Run and monitor scripts |
| **Analysis** | `stats`, `analyze`, `list`, `info` | Project insights |
| **Maintenance** | `clean`, `clean --cache`, `clean --all` | Project cleanup |
| **Package Manager** | `agent` | Direct access to underlying PM |

For detailed usage of each command, see [NEW_FEATURES.md](NEW_FEATURES.md).

## �Parallel Execution Benefits

**Serial execution:**
```
test:  0.5s
lint:  1.2s
build: 3.5s
────────────
Total: 5.2s
```

**Parallel execution with kn:**
```
kn parallel test lint build
────────────
Total: 3.5s (time of longest task)
Saved: ~1.7s
```

### Features Performance
- **Startup time**: < 10ms
- **Command parsing**: < 1ms
- **Fuzzy search**: < 1ms (Levenshtein distance)

---

## 📊 Project Stats

- **Lines of Code**: ~5,000+ (Rust)
- **Commands**: 19 total (14 new intelligent features)
- **Test Coverage**: 16/16 tests passing
- **Compile Time**: < 30s (debug), < 60s (release)
- **Binary Size**: ~3MB (release)
- **Startup Time**: < 10ms

---
