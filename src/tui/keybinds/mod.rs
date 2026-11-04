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
/// 1. Check for truly global keybinds (Ctrl-C to quit)
/// 2. If help modal is open, use help mode handlers
/// 3. Otherwise, use mode-specific handlers (Normal or Search)
pub(super) fn handle_key_event(
   app: &mut App,
   key: KeyEvent,
) {
   match key.code {
      KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
         app.quit();
         return;
      },
      KeyCode::Char('n') | KeyCode::Char('p') if key.modifiers.contains(KeyModifiers::CONTROL) => {
         return;
      },
      _ => {},
   }

   if app.is_pending_key_expired() {
      app.clear_pending_key();
   }

   if app.is_help_visible() {
      help::handle_help_mode(app, key);
      return;
   }

   if global::was_global_keybind_handled(app, &key) {
      return;
   }

   match app.input_mode() {
      crate::tui::state::InputMode::Normal => normal::handle_normal_mode(app, key),
      crate::tui::state::InputMode::Search => search::handle_search_mode(app, key),
   }
}
