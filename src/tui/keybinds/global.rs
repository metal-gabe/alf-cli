//! Global keybinds that work in any mode (except help modal).

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::tui::app::App;

/// Handle global keybinds that work in any mode (except help modal).
/// Returns true if a global keybind was handled.
pub fn handle_global_keybinds(app: &mut App, key: &KeyEvent) -> bool {
   match key.code {
      // Clear search (Ctrl-u)
      KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
         app.clear_search();
         true
      }
      _ => false,
   }
}
