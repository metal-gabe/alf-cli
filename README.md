# `alf` — Alias & Function CLI Searching Tool

A Rust TUI to rediscover your custom shell aliases & functions.

## Installation

### From Source

```bash
cargo install --git https://github.com/metal-gabe/alf-cli
```

### From Crates.io

```bash
cargo install alf
```

### From Mise 

```bash
mise use -g alf
```

### From Homebrew 

```bash
brew install alf 
```



## Quick Start

1. **First-time setup**: Run the initialization wizard

   ```bash
   alf init
   ```

2. **Launch the search interface**:

   ```bash
   alf
   ```

> [!TIP]
>
> After installing, `alf` can be run right away. Using the `init` command creates a starting config file for you to be able to customize.

## Configuration

Configuration file location (created after `alf init`):

- **Linux**: `~/.config/alf/config.toml`
- **macOS**: `~/.config/alf/config.toml`
- **Windows**: `%USERPROFILE%\.config\alf\config.toml`

### Available Commands

- `alf` - Launch interactive TUI (default, no subcommand)
- `alf search <QUERY>` - Launch TUI with an initial search query pre-filled
- `alf init` - First-run configuration wizard
- `alf activate <SHELL>` - Print shell integration wrapper (`zsh` or `bash`)
- `alf config show` - Display current configuration
- `alf config edit` - Open config in editor
- `alf config reset` - Reset to defaults

### Configuration Options

```toml
[general]
shell_files = ["~/.zshrc", "~/.config/zsh/**/*.zsh"]  # glob patterns supported

[search]
case_matching = "smart"      # "ignore" | "smart" | "respect"
normalize = true             # unicode normalization
enable_regex = true
substring_matching = true

[ui]
theme = "default"            # see Available Themes below
keybind_mode = "vim"         # currently only "vim" is supported

[display]
show_type_badges = true      # show Alias/Function badges
syntax_highlighting = true   # syntax highlight in detail view
parse_comments = true        # parse and display comments from shell files
```

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
- `/` - Focus search & activate search mode
- `Esc` - Exit search mode
- `Tab` - Populate the parent shell prompt with the selected entry
- `Enter` - Execute the selected entry in the parent shell
- `q` - Quit

## Shell Integration

`Tab` and `Enter` only affect the parent shell when the `alf` shell hook is sourced. Add this to your shell config:

```bash
# zsh (~/.zshrc)
eval "$(alf activate zsh)"

# bash (~/.bashrc)
eval "$(alf activate bash)"
```

The hook installs an `alf` shell function that wraps the binary so selections feed back into the prompt.

Tab vs Enter semantics:

- **Tab** — populate the prompt with the selected entry; do not run it.
- **Enter** — run the selected entry immediately (and add it to history).

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
cargo run
```

### Makefile targets

| Target | Description |
|---|---|
| `make build` | Debug build |
| `make build-release` | Optimized release build |
| `make check` | Check without building |
| `make clean` | Remove build artifacts |
| `make clippy` | Lint check |
| `make fmt` / `make fmt-fix` | Check / auto-fix formatting |
| `make install` | Install locally |
| `make lint` | Run fmt + clippy |
| `make run` | Run TUI (debug build) |
| `make snap` | Review insta snapshot diffs |
| `make test` | Run tests via nextest |
| `make test-cov` | Generate HTML coverage report |
| `make test-fresh` | Run tests with no cache |
| `make watch` | Watch & rebuild on changes |

## Supported Platforms

- Linux (x86_64, aarch64)
- macOS (Intel, Apple Silicon)
- Windows (x86_64)

## License

Licensed under either of the following choices at your option.

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))
