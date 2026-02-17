//! Centralized color palette for the TUI.
//!
//! All hardcoded RGB color values are defined here as constants,
//! providing a single source of truth for the color scheme.

use ratatui::style::Color;

// Entry type colors
pub const COLOR_ALIAS: Color = Color::Rgb(253, 90, 30);
pub const COLOR_FUNCTION: Color = Color::Rgb(0, 199, 255);

// UI chrome colors
pub const COLOR_BACKGROUND: Color = Color::Rgb(17, 17, 17);
pub const COLOR_TEXT_ACTIVE: Color = Color::Rgb(220, 220, 220);
pub const COLOR_SELECTION_BG: Color = Color::Rgb(23, 148, 129);
pub const COLOR_SCROLLBAR: Color = Color::Rgb(100, 100, 100);
pub const COLOR_SCROLLBAR_HELP: Color = Color::Rgb(255, 200, 100);

// Mode indicator colors
pub const COLOR_MODE_NORMAL: Color = Color::Rgb(144, 238, 144);
pub const COLOR_MODE_SEARCH: Color = Color::Rgb(255, 200, 100);
