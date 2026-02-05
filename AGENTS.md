# AGENTS.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Overview
- The README describes `alf`, a Rust CLI for searching shell aliases/functions and their descriptions; binary name is `alf`.

## Architecture / structure
- `assets/` holds UI/layout design artifacts:
  - `alf-cli-layout.drawio` is the editable diagram source.
  - `alf-cli-layout.png` is a rendered preview.
  - The layout depicts a search bar, a list of aliases/functions, a comments/description panel, and a script/function body panel, plus a help/footer area.
- No application source code or build configuration is present in this repo (only assets and README).

## Commands
- No build/lint/test commands found (no `Cargo.toml`, `package.json`, `Makefile`, etc.).
