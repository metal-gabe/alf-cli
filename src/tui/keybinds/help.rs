//! Help modal keybinds (help content navigation and scrolling).

use crossterm::event::{KeyCode, KeyEvent};

use crate::tui::app::App;

/// Handle key events in Help modal mode
pub fn handle_help_mode(app: &mut App, key: KeyEvent) {
   // Check for pending multi-key sequences first
   if let Some(pending) = app.pending_key() {
      match (pending, key.code) {
         // Esc clears pending key
         (_, KeyCode::Esc) => {
            app.clear_pending_key();
            return;
         }
         // Any other key - invalid sequence, clear pending
         _ => {
            app.clear_pending_key();
            // Fall through to process the current key normally
         }
      }
   }

   match key.code {
      // Close help modal with '?', 'q', or Esc
      KeyCode::Char('?') | KeyCode::Char('q') | KeyCode::Esc => {
         app.toggle_help();
         app.clear_pending_key();
      }

      // Scroll help content
      KeyCode::Char('j') | KeyCode::Down => app.help_scroll_down(),
      KeyCode::Char('k') | KeyCode::Up => app.help_scroll_up(),

      // Jump to bottom
      KeyCode::Char('G') => app.help_jump_bottom(),

      // Jump to top
      KeyCode::Char('g') => app.help_jump_top(),

      // Ignore all other keys (prevent propagation to underlying panels)
      _ => {}
   }
}
