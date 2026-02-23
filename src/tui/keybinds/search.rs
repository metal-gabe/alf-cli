//! Search mode keybinds (text input and search navigation).

use crossterm::event::{KeyCode, KeyEvent};

use crate::tui::app::App;

/// Handle key events in Search (text input) mode
pub fn handle_search_mode(app: &mut App, key: KeyEvent) {
   match key.code {
      // Exit search mode, keep query and filtered results
      KeyCode::Esc => app.exit_search_keep_query(),

      // Panel cycling (Shift+n/p sends uppercase N/P)
      KeyCode::Char('N') => app.cycle_panel(),
      KeyCode::Char('P') => app.cycle_panel_backward(),

      // Filter cycling (Shift+h/l sends uppercase H/L)
      KeyCode::Char('L') => app.cycle_filter(),
      KeyCode::Char('H') => app.cycle_filter_backward(),

      // Text editing (captures all other characters including lowercase n,p,h,l)
      KeyCode::Char(c) => app.search_insert_char(c),
      KeyCode::Backspace => app.search_delete_char(),

      // Cursor movement within search
      KeyCode::Left => app.search_cursor_left(),
      KeyCode::Right => app.search_cursor_right(),

      // Enter is reserved for future use
      KeyCode::Enter => {}

      _ => {}
   }
}
