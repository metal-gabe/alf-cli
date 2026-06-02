//! Keybinding definitions and handling.
//!
//! Implements vim-style keybindings with mode awareness (Normal vs Search).
//! Organized into focused submodules by concern:
//! - `global`: keybinds that work in any mode (quit, clear search)
//! - `normal`: vim-style navigation and panel/filter control
//! - `search`: text input and search mode navigation
//! - `help`: help modal scrolling and navigation

mod global;
mod help;
mod normal;
mod search;

#[cfg(test)]
mod keybind_tests;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::tui::app::App;

/// Handle a key event based on the current input mode.
///
/// Dispatches to appropriate handler based on mode:
/// 1. Check for truly global keybinds (Ctrl-C, Ctrl-D to quit)
/// 2. If help modal is open, use help mode handlers
/// 3. Otherwise, use mode-specific handlers (Normal or Search)
pub(super) fn handle_key_event(
   app: &mut App,
   key: KeyEvent,
) {
   // Check for truly global keybinds first - these work ALWAYS or are reserved
   match key.code {
      // Quit app (Ctrl-C or Ctrl-D)
      KeyCode::Char('c') | KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
         app.should_quit = true;
         return;
      },
      // Reserved for future use - ignore these globally
      KeyCode::Char('n') | KeyCode::Char('p') if key.modifiers.contains(KeyModifiers::CONTROL) => {
         return;
      },
      _ => {},
   }

   // Check if pending key has expired - clear it if so
   if app.is_pending_key_expired() {
      app.clear_pending_key();
   }

   // If help modal is open, handle help-specific keybinds
   if app.show_help() {
      help::handle_help_mode(app, key);
      return;
   }

   // Handle other global keybinds (work in any mode except help)
   if global::handle_global_keybinds(app, &key) {
      return;
   }

   // Then handle mode-specific keybinds
   match app.input_mode() {
      crate::tui::state::InputMode::Normal => normal::handle_normal_mode(app, key),
      crate::tui::state::InputMode::Search => search::handle_search_mode(app, key),
   }
}
