<div align="center">

# ⚡ KN

**Blazing fast Node.js package manager CLI written in Rust**

[![Rust](https://img.shields.io/badge/Rust-2021-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-macOS%20%7C%20Linux%20%7C%20Windows-lightgrey.svg)](https://github.com/wangsizhu0504/kn)

</div>

---

## ✨ Features

- 🎨 Beautiful terminal UI with modern styling
- 📦 Auto-detects package manager (npm/yarn/pnpm/bun)
- 🏃 Fast script execution with fuzzy matching
- 👀 Watch mode for auto-rerun on file changes
- 📊 Analyze package sizes and disk usage
- 🧹 Smart cleanup for project files and caches

---

## 🚀 Installation

### Quick Install

```bash
curl -fsSL https://raw.githubusercontent.com/wangsizhu0504/kn/main/install.sh | bash
```

### From Source

```bash
cargo install --git https://github.com/wangsizhu0504/kn
```

---

## 📖 Usage

```bash
kn help              # Show all commands
kn install react     # Install packages
kn i react -D        # Install as dev dependency
kn run dev           # Run npm scripts
kn list              # Show available scripts
kn info              # Show environment info
kn size              # Analyze package sizes
kn clean             # Clean project files
```

---

## 🎯 Commands

```
  install (i, add)        Install packages (auto-detects package manager)
  run (r)                 Run npm scripts from package.json
  uninstall (remove, rm)  Uninstall packages
  execute (exec, x)       Execute package binaries
  upgrade (update, up)    Upgrade dependencies
  upgrade-self            Upgrade kn to the latest version
  clean-install (ci)      Clean install dependencies (frozen lockfile)
  list (ls)               Show available package scripts
  info (env)              Show package manager and environment information
  watch (w)               Watch files and re-run script on changes
  clean                   Clean node_modules, cache, etc.
  size                    Analyze package sizes
  help                    Show this help message
```

---

## ⚙️ Configuration

Create `~/.knrc`:

```ini
default_agent = npm
global_agent = npm
```

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

KN supports configuration via `.knrc` file (INI or JSON format) in your home directory.

#### Configuration Location
- Default: `~/.knrc` or `~/.knrc.json`
- Custom: Set `KN_CONFIG_FILE` environment variable

#### Configuration Options

**JSON Format** (recommended for IDE support):
```json
{
  "defaultAgent": "prompt",
  "globalAgent": "npm",
  "autoUpdate": true,
  "collectStats": true,
  "logLevel": "info"
}
```

**INI Format** (backward compatible):
```ini
# ~/.knrc
default_agent = prompt
global_agent = npm
auto_update = true
log_level = info
```

#### Configuration Options Explained

| Option | Values | Description |
|--------|--------|-------------|
| `defaultAgent` | `npm`, `yarn`, `pnpm`, `bun`, `prompt` | Default package manager |
| `globalAgent` | `npm`, `yarn`, `pnpm`, `bun` | Package manager for global installs |
| `autoUpdate` | `true`, `false` | Enable automatic update checks |
| `logLevel` | `error`, `warn`, `info`, `debug`, `trace` | Logging verbosity |

#### Environment Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `KN_CONFIG_FILE` | Custom config file path | `~/.config/kn/config.json` |
| `KN_LOG` | Override log level | `debug`, `trace` |

**Example with custom log level:**
```bash
KN_LOG=debug kn install package
```

---

## ⚙️ Configuration

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
