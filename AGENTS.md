# AGENTS.md

This file provides guidance to AI coding assistants when working with code in this repository.

## Overview
- `alf` is a Rust CLI TUI for searching shell aliases/functions and their descriptions; binary name is `alf`.
- Built with ratatui (TUI), crossterm (terminal), clap (CLI), nucleo-matcher (fuzzy search), syntect (syntax highlighting).
- MSRV: 1.74.0, dual licensed MIT/Apache-2.0.

## Architecture / structure
- `assets/` holds UI/layout design artifacts (drawio + png).
- `src/main.rs` — CLI entry point, routes to TUI or subcommands.
- `src/lib.rs` — Library root, exports all modules.
- `src/cli/` — clap-based CLI argument parsing (search, init, activate, config, which subcommands).
  - `which.rs` — `alf which <NAME>`: prints alf's parsed info for a name plus the `which`/`type` lookup result.
  - The shell hook in `init.rs` runs `which`/`type` in the user's interactive shell and passes the results via `ALF_LOOKUP_NAME`, `ALF_WHICH_RESULT` and `ALF_TYPE_RESULT`; `which.rs` trusts them only when `ALF_LOOKUP_NAME` matches the parsed name, and otherwise falls back to subprocess lookups.
- `src/models/` — Core data structures: `AliasEntry`, `EntryType`, `SearchResult`.
- `src/config/` — Configuration structs, TOML load and atomic save, advisory file lock, `~`/`$HOME` expansion. The `[display]` and `[search]` settings and `ui.keybind_mode` are reserved and have no effect yet.
- `src/parser/` — Regex-based shell file parser for aliases, functions and their `# alf … # fla` and `#@: … :f#` description comments.
- `src/search/` — Fuzzy search using nucleo-matcher. The TUI does not call it yet; it filters by substring in `src/tui/state/filter.rs`.
- `src/tui/` — Terminal UI implementation:
  - `mod.rs` — TUI run loop (config and entry loading, terminal setup/teardown, event loop, `ALF_OUTPUT`).
  - `app.rs` — Application state facade over `state/`.
  - `state/` — State modules: `data`, `filter`, `input`, `navigation`, `scroll`, `search`, `ui`.
  - `ui/` — UI rendering: `header`, `search`, `footer`, `help`, `components`, and `body/` (`list`, `description`, `script`).
  - `events.rs` — Event polling with crossterm.
  - `keybinds/` — Vim-style keybinding handlers (`normal`, `search`, `help`, `global`).
  - `syntax.rs` — Shell syntax highlighting with syntect.
  - `themes.rs` — 8 predefined color themes.
  - `mock.rs` — Mock alias/function data; nothing at runtime uses it.
- `src/test_support.rs` — `TempHome`, an isolated `$HOME` for tests.

## Commands
- `cargo build` — Build the project.
- `cargo run` or `cargo run -- search` — Launch the TUI with the configured shell files, or the standard ones when no config exists; exits with an error if none can be read.
- `cargo run -- --help` — Show CLI help.
- `cargo test` — Run tests.
- `cargo clippy` — Lint.
- `cargo fmt --check` — Check formatting.
