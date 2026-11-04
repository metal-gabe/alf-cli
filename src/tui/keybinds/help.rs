//! Help modal keybinds (help content navigation and scrolling).

use crossterm::event::{KeyCode, KeyEvent};

use crate::tui::app::App;

/// Handle key events in Help modal mode
pub fn handle_help_mode(
   app: &mut App,
   key: KeyEvent,
) {
   if let Some(pending) = app.pending_key() {
      match (pending, key.code) {
         (_, KeyCode::Esc) => {
            app.clear_pending_key();
            return;
         },
         _ => {
            app.clear_pending_key();
         },
      }
   }

   match key.code {
      KeyCode::Char('?') | KeyCode::Char('q') | KeyCode::Esc => {
         app.toggle_help();
         app.clear_pending_key();
      },

      KeyCode::Char('j') | KeyCode::Down => app.help_scroll_down(),
      KeyCode::Char('k') | KeyCode::Up => app.help_scroll_up(),

      KeyCode::Char('G') => app.help_jump_bottom(),

      KeyCode::Char('g') => app.help_jump_top(),

      _ => {},
   }
}
