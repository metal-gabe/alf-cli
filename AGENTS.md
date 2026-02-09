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
- `src/cli/` — clap-based CLI argument parsing (search, init, config subcommands).
- `src/models/` — Core data structures: `AliasEntry`, `EntryType`, `SearchResult`.
- `src/config/` — Configuration structs and load/save stubs (TOML, platform-aware paths).
- `src/parser/` — Shell file parsing stubs.
- `src/search/` — Fuzzy search stubs using nucleo-matcher.
- `src/tui/` — Terminal UI implementation:
  - `mod.rs` — TUI run loop (terminal setup/teardown, event loop).
  - `app.rs` — Application state (entries, selection, filters, input mode, scroll offsets).
  - `ui.rs` — UI rendering: header, search bar, entry list, description panel, script panel, footer.
  - `events.rs` — Event polling with crossterm.
  - `keybinds.rs` — Vim-style keybinding handler (Normal + Search modes).
  - `themes.rs` — 8 predefined color themes (not yet wired into rendering).
  - `mock.rs` — Mock alias/function data for development.

## Commands
- `cargo build` — Build the project.
- `cargo run` or `cargo run -- search` — Launch the TUI with mock data.
- `cargo run -- --help` — Show CLI help.
- `cargo test` — Run tests.
- `cargo clippy` — Lint.
- `cargo fmt --check` — Check formatting.
