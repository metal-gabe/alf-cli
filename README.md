# `alf` — Alias & Function CLI Searching Tool

A Rust TUI to rediscover your custom shell aliases & functions.

## Installation

### From Source

```bash
cargo install --git https://github.com/metal-gabe/alf-cli
```

### From Crates.io

```bash
cargo install alf_tui
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
- `alf init --print-shell-hook <SHELL>` - Print the shell wrapper and exit (non-interactive equivalent of `alf activate <SHELL>`)
- `alf activate <SHELL>` - Print shell integration wrapper (`zsh` or `bash`)
- `alf config show` - Display current configuration
- `alf config edit` - Open config in editor
- `alf config reset` - Reset to defaults

### Configuration Options

Running `alf init` writes a `config.toml` like the one below. 

The `shell_files` list is filled in with the shell files detected in your home directory (plus any you add during the wizard), and `theme` reflects your menu choice; every other value is the built-in default. 

The comments are annotations for this README and are not written to the generated file.

```toml
[display]
show_type_badges = true      # show Alias/Function badges
syntax_highlighting = true   # syntax highlight in detail view
parse_comments = true        # parse and display comments from shell files

[general]
shell_files = ["~/.bashrc", "~/.zshrc"]  # detected at init; glob patterns supported
alias_expansion = "name"     # "name" | "script" — what Tab/Enter feeds back to the shell

[search]
case_matching = "smart"      # "ignore" | "smart" | "respect"
normalize = true             # unicode normalization
enable_regex = true
substring_matching = true

[ui]
theme = "default"            # see Available Themes below
keybind_mode = "vim"         # currently only "vim" is supported
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

The full reference is always available in-app — press `?` to toggle the help modal.

<details>
<summary>View all keybindings</summary>
<br>
<strong>Navigation</strong>

<ul>
<li><code>j</code> / <code>↓</code> - Move down 1 line in the active panel</li>
<li><code>k</code> / <code>↑</code> - Move up 1 line in the active panel</li>
<li><code>g</code> / <code>gg</code> - Jump to top</li>
<li><code>G</code> - Jump to bottom</li>
<li><code>Ctrl-f</code> / <code>Ctrl-b</code> - Scroll a full page (20 lines) down/up</li>
<li><code>Ctrl-j</code> / <code>Ctrl-k</code> - Scroll a half page (10 lines) down/up</li>
</ul>

<strong>Panels &amp; filters</strong>
<ul>
<li><code>n</code> / <code>p</code> - Cycle panel focus forward/backward (List → Description → Script)</li>
<li><code>h</code> / <code>l</code> - Cycle filter backward/forward (Aliases ↔ Functions ↔ All)</li>
<li><code>1</code> / <code>2</code> / <code>3</code> - Select the Aliases / Functions / All filter directly</li>
</ul>

<strong>Grouping &amp; sorting</strong>
<ul>
<li><code>og</code> / <code>Ctrl-g</code> - Cycle group mode forward (None → Aliases → Functions)</li>
<li><code>oG</code> - Cycle group mode backward</li>
<li><code>os</code> / <code>Ctrl-s</code> - Toggle sort order (Ascending ↔ Descending)</li>
</ul>

<strong>Search</strong>
<ul>
<li><code>/</code> or <code>i</code> - Enter search mode</li>
<li><code>Esc</code> - Exit search mode (keep query)</li>
<li><code>Ctrl-u</code> - Clear search query (any mode)</li>
<li><code>Shift-N</code> / <code>Shift-P</code> - Cycle panels while in search mode</li>
<li><code>Shift-H</code> / <code>Shift-L</code> - Cycle filters while in search mode</li>
<li><code>Ctrl-j</code> / <code>Ctrl-k</code> - Scroll the list while in search mode</li>
</ul>

<strong>Themes</strong>
<ul>
<li><code>tj</code> - Cycle to the next theme</li>
<li><code>tk</code> - Cycle to the previous theme</li>
</ul>

<strong>Actions &amp; quit</strong>
<ul>
<li><code>Tab</code> - Populate the parent shell prompt with the selected entry</li>
<li><code>Enter</code> - Execute the selected entry in the parent shell</li>
<li><code>?</code> - Toggle the help modal</li>
<li><code>q</code> - Quit (normal mode only)</li>
<li><code>Ctrl-c</code> / <code>Ctrl-d</code> - Force quit (any mode)</li>
</ul>
</details>

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
