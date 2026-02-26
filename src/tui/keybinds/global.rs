//! Global keybinds that work in any mode (except help modal).

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::tui::app::App;

/// Handle global keybinds that work in any mode (except help modal).
/// Returns true if a global keybind was handled.
pub fn handle_global_keybinds(app: &mut App, key: &KeyEvent) -> bool {
   match key.code {
      // Scroll down (Ctrl-d, same as Ctrl-j)
      KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
         app.scroll_down(10);
         true
      }
      // Scroll up (Ctrl-u, same as Ctrl-k)
      KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
         app.scroll_up(10);
         true
      }
      _ => false,
   }
}
