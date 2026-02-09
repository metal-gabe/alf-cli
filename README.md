# `alf` — Alias & Function CLI Searching Tool

A Rust-built CLI tool to search through & rediscover your custom-made shell aliases, functions, and their descriptions (*i.e. comments*).

## Installation

### From Source

```bash
cargo install --git https://github.com/metal-gabe/alf-cli
```

### From Crates.io (after publishing)

```bash
cargo install alf
```

## Quick Start

1. **First-time setup**: Run the initialization wizard
   ```bash
   alf init
   ```

2. **Launch the search interface**:
   ```bash
   alf search
   # or simply
   alf
   ```

## Configuration

Configuration file location (created after `alf init`):

- **Linux**: `~/.config/alf/config.toml`
- **macOS**: `~/Library/Application Support/alf/config.toml`
- **Windows**: `%APPDATA%\alf\config.toml`

### Available Commands

- `alf search` - Launch interactive TUI (default)
- `alf init` - First-run configuration wizard
- `alf config show` - Display current configuration
- `alf config edit` - Open config in editor
- `alf config reset` - Reset to defaults

### Available Themes

- `default` - Classic terminal colors
- `gruvbox` - Retro groove
- `nord` - Arctic blue
- `dracula` - Purple/pink dark theme
- `solarized` - Precision colors
- `catppuccin` - Soothing pastels (Mocha)
- `tokyonight` - Tokyo-inspired dark theme (Storm)
- `shades_of_purple` - Purple-heavy theme

## Keybindings (Vim-style)

- `j/k` - Move down/up
- `gg/G` - Jump to top/bottom
- `Ctrl-u/d` - Scroll up/down
- `/` - Focus search
- `Esc` - Clear search
- `q` - Quit

## Development

### Requirements

- Rust 1.74.0 or later

### Build

```bash
cargo build --release
```

### Run tests

```bash
cargo test
```

### Run locally

```bash
cargo run -- search
```

## Supported Platforms

- Linux (x86_64, aarch64)
- macOS (Intel, Apple Silicon)
- Windows (x86_64)

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.