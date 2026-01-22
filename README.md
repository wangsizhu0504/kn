# kn

<p align="center">
  <h1>⚡ KN - Fast & Smart Node.js Package Manager</h1>
  <p>Minimal, blazing fast Node.js package manager and scripts runner with intelligent features</p>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Rust-2021-orange.svg" alt="Rust 2021">
  <img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="MIT License">
  <img src="https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey.svg" alt="Cross Platform">
  <img src="https://img.shields.io/badge/CLI-Custom-blue.svg" alt="Custom CLI">
</p>

## ✨ Features

### 🚀 Core Features
- **⚡ Blazing Fast** - Rust implementation with ~1.6ms startup time
- **🔧 Auto-Detection** - Automatically detects npm, yarn, pnpm, or bun from lock files
- **📝 Smart Scripts** - Fast script running with beautiful formatted output
- **🎯 Smart Command Suggestions** - Levenshtein distance-based command matching
- **🌍 Cross Platform** - Windows, macOS, and Linux support

### 🎨 Intelligent Features
- **📜 Command History** - Track and replay your commands (`kn !!`, `kn !N`)
- **🔗 Script Aliases** - Create shortcuts for frequently used scripts
- **🔍 Fuzzy Search** - Auto-match script names (e.g., `tst` → `test`)
- **📊 Performance Stats** - Track execution time and run counts
- **⚡ Parallel Execution** - Run multiple scripts simultaneously
- **🧹 Smart Cleanup** - Clean node_modules, cache, and build artifacts
- **📊 Dependency Analysis** - Analyze project dependencies and disk usage
- **👀 Watch Mode** - Monitor file changes and auto-rerun scripts

### 🎯 Developer Experience
- **🎨 Beautiful Output** - Colorful ASCII art and well-formatted displays
- **📋 Dashboard UI** - Script list with elegant dashboard layout
- **🚫 Zero Config** - Works out of the box, configure only if needed
- **🔥 Custom CLI** - No external CLI framework dependencies

## 🚀 Installation

### From crates.io (Recommended)
```bash
cargo install kn
```

### Build from Source
```bash
git clone https://github.com/your-username/kn
cd kn
cargo build --release
```

### Quick Installation Script
```bash
# Clone and build
git clone https://github.com/your-username/kn
cd kn
chmod +x install-kn.sh
./install-kn.sh
```

## 📖 Usage

### Core Commands

```bash
# Package Management
kn install react typescript -D    # Install packages
kn i react vite                   # Short alias
kn uninstall webpack              # Remove packages
kn rm webpack                     # Short alias
kn upgrade                        # Update dependencies
kn clean-install                  # Clean install with frozen lockfile

# Script Execution
kn run dev                        # Run a script
kn r build                        # Short alias
kn run                            # List all available scripts

# Direct Execution
kn execute tsc                    # Execute a package binary
kn x create-react-app my-app      # Short alias

# Information
kn list                           # Show all scripts (dashboard style)
kn info                           # Show package manager info
kn help                           # Show help with ASCII art

# Advanced Features
kn history                        # Show command history
kn !!                             # Re-run last command
kn !3                             # Re-run command #3 from history
kn alias set d dev                # Create script alias
kn alias                          # List all aliases
kn stats                          # Show script performance stats
kn parallel dev test build        # Run multiple scripts in parallel
kn p lint test                    # Short alias for parallel
kn clean                          # Clean project files
kn clean --cache                  # Clean package manager cache
kn clean --all                    # Deep clean (project + cache)
kn analyze                        # Analyze project dependencies
kn watch dev                      # Watch files and re-run script
```

### Quick Examples

```bash
# Basic usage
kn i react                 # Install react
kn r dev                   # Run dev script
kn ls                      # List scripts

# Fuzzy search (auto-corrects typos)
kn r tst                   # Automatically runs 'test'
kn r dv                    # Automatically runs 'dev'

# Create and use aliases
kn alias set d dev
kn alias set b build
kn r d                     # Runs dev script

# Work with history
kn run test
kn !!                      # Re-runs test
kn history                 # Show all history

# Parallel execution (save time!)
kn parallel lint test build
# Output shows real-time progress and time saved

# Performance tracking
kn r test                  # Output: ✓ Completed in 0.15s
kn stats                   # Shows all scripts' performance data
```

## 🎯 Smart Features

### 🔍 Fuzzy Script Search

kn automatically matches similar script names using Levenshtein distance:

```bash
$ kn run tst
Did you mean 'test'? Running it...
Running tests...
✓ Completed in 0.15s
```

### 📜 Command History

Track and replay commands easily:
 with Dashboard UI

```bash
$ kn list
╭─────────────────────────────────────────────────────────────────────╮
│  📦  my-project v1.0.0                                              │
├─────────────────────────────────────────────────────────────────────┤
│  📋  Available Scripts                                              │
├─────────────────────────────────────────────────────────────────────┤
│  ├─ dev           vite --mode development                           │
│  ├─ build         vite build                                        │
│  ├─ test          vitest run                                        │
│  ├─ lint          eslint src/                                       │
│  └─ preview       vite preview                                      │
╰─────────────────────────────────────────────────────────────────────╯

  💡 Tip: Run scripts with: kn run <script-name>
   1  parallel lint test

$ kn !!              # Re-run last command
$ kn !3              # Run command #3
```

### 🔗 Script Aliases

Create shortcuts for frequently used scripts:

```bash
$ kn alias set d dev
✓ Alias created: d → dev

$ kn alias set b build
✓ Alias created: b → build

$ kn alias
🔗 Script Aliases
  d               → dev
  b               → build

$ kn r d            # Runs 'dev' script
```

### 📊 Performance Statistics

Automatic performance tracking:

```bash
$ kn stats
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
──────────────────────   # Main entry point
│   ├── agents.rs           # Package manager definitions
│   ├── cli_parser.rs       # Custom CLI argument parser (no clap!)
│   ├── command/            # Command handlers
│   │   ├── install.rs      # Package installation
│   │   ├── run.rs          # Script execution with fuzzy search
│   │   ├── list.rs         # Script listing with dashboard UI
│   │   ├── uninstall.rs    # Package removal
│   │   ├── execute.rs      # Direct binary execution
│   │   ├── upgrade.rs      # Dependency updates
│   │   ├── clean_install.rs
│   │   ├── agent.rs        # Direct package manager access
│   │   ├── info.rs         # Environment information
│   │   ├── watch.rs        # File watching
│   │   ├── history.rs      # Command history tracking
│   │   ├── alias.rs        # Script alias management
│   │   ├── stats.rs        # Performance statistics
│   │   ├── parallel.rs     # Parallel script execution
│   │   ├── clean.rs        # Project cleanup
│   │   ├── analyze.rs      # Dependency analysis
│   │   ├── cli.rs          # Command dispatcher
│   │   └── mod.rs
│   ├── command_utils.rs    # Package.json utilities
│   ├── config.rs           # Configuration management
│   ├── detect.rs           # Package manager detection
│   ├── runner.rs           # Command execution engine
│   ├── display.rs          # Styled output and ASCII art
│   ├── storage.rs          # Persistent storage for history/aliases
│   ├── utils.rs            # General utilities
│   └── test.rs             # Comprehensive test suite
├── Cargo.toml
├── README.md
├── NEW_FEATURES.md         # Detailed new features documentation
├── install-kn.sh          # Installation script
└── test-project/          # Test project for developmen logic
│   ├── runner.rs    all 19 commands)
- ✅ CLI interface (help, version, history, aliases)
- ✅ Fuzzy search algorithm
- ✅ Performance tracking
- ✅ Edge cases and error handling

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_package_parsing

# Run tests with output
cargo test -- --nocapture

# Run tests in release mode
cargo test --release
```

All tests pass: **16/16 ✅**
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_package_parsing

# Run tests with output
cargo test -- --nocapture
```

### Quick Test Script
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
- Command history (last 100 commands)
- Script aliases
- Performance statistics

This data persists across sessions and is automatically managed.ini
default_agent = npm    # fallback when no lock found
global_agent = npm      # for global installs
### Script Execution Benchmarks
- **kn**: ~1.6ms ⚡ (fastest)
- bun: ~4.2ms
- node --run: ~6.1ms
- npm: ~154.6ms
- yarn: ~306.1ms
- pnpm: ~463.2ms

## 🎨 Command Overview

| Category | Commands | Description |
|----------|----------|-------------|
| **Package Management** | `install`, `uninstall`, `upgrade`, `clean-install` | Manage dependencies |
| **Script Execution** | `run`, `execute`, `watch`, `parallel` | Run and monitor scripts |
| **Productivity** | `alias`, `history`, `!!`, `!N` | Shortcuts and history |
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
- **History lookup**: < 1ms
- **Fuzzy search**: < 1ms (Levenshtein distance)
- **Alias resolution**: < 1

---

## 📊 Project Stats

- **Lines of Code**: ~5,000+ (Rust)
- **Commands**: 19 total (14 new intelligent features)
- **Test Coverage**: 16/16 tests passing
- **Compile Time**: < 30s (debug), < 60s (release)
- **Binary Size**: ~3MB (release)
- **Startup Time**: < 10ms

---

<p align="center">
  Made with ❤️ and ⚡ in Rust
  <br>
  <strong>Fast. Smart. Beautiful.</strong>
git clone https://github.com/your-username/kn
cd kn

# Build
cargo build

# Run tests
cargo test

# Build release version
cargo build --release

# Install locally
cargo install --path .
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Setup

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass (`cargo test`)
6. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

<p align="center">
  Made with ❤️ in Rust
</p>
