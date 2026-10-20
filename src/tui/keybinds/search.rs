//! Search mode keybinds (text input and search navigation).

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::tui::app::{App, ExitAction};

/// Handle key events in Search (text input) mode
pub fn handle_search_mode(
   app: &mut App,
   key: KeyEvent,
) {
   match key.code {
      // Exit search mode, keep query and filtered results
      KeyCode::Esc => app.exit_search_keep_query(),

      // Panel management (Shift+n/p/f sends uppercase N/P/F)
      KeyCode::Char('N') => app.cycle_panel(),
      KeyCode::Char('P') => app.cycle_panel_backward(),
      KeyCode::Char('F') => app.toggle_fullscreen(),

      // Filter cycling (Shift+h/l sends uppercase H/L)
      KeyCode::Char('L') => app.cycle_filter(),
      KeyCode::Char('H') => app.cycle_filter_backward(),

      // List navigation while in search mode
      KeyCode::Char('j') if key.modifiers.contains(KeyModifiers::CONTROL) => app.scroll_down(1),
      KeyCode::Char('k') if key.modifiers.contains(KeyModifiers::CONTROL) => app.scroll_up(1),

      // Word and line editing (vim/readline style)
      KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => app.search_delete_char_forward(),
      KeyCode::Char('w') if key.modifiers.contains(KeyModifiers::CONTROL) => app.search_delete_word_backward(),
      KeyCode::Char('h') if key.modifiers.contains(KeyModifiers::CONTROL) => app.search_delete_char(),
      KeyCode::Char('a') if key.modifiers.contains(KeyModifiers::CONTROL) => app.search_cursor_start(),
      KeyCode::Char('e') if key.modifiers.contains(KeyModifiers::CONTROL) => app.search_cursor_end(),
      KeyCode::Char('b') if key.modifiers.contains(KeyModifiers::ALT) => app.search_cursor_word_left(),
      KeyCode::Char('f') if key.modifiers.contains(KeyModifiers::ALT) => app.search_cursor_word_right(),
      KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::ALT) => app.search_delete_word_forward(),
      KeyCode::Backspace if key.modifiers.intersects(KeyModifiers::CONTROL | KeyModifiers::ALT) => {
         app.search_delete_word_backward()
      },
      KeyCode::Delete if key.modifiers.intersects(KeyModifiers::CONTROL | KeyModifiers::ALT) => {
         app.search_delete_word_forward()
      },
      KeyCode::Left if key.modifiers.intersects(KeyModifiers::CONTROL | KeyModifiers::ALT) => {
         app.search_cursor_word_left()
      },
      KeyCode::Right if key.modifiers.intersects(KeyModifiers::CONTROL | KeyModifiers::ALT) => {
         app.search_cursor_word_right()
      },

      // Text editing (captures all other characters including lowercase n,p,h,l)
      KeyCode::Char(c)
         if key.modifiers.contains(KeyModifiers::CONTROL | KeyModifiers::ALT)
            || !key.modifiers.intersects(KeyModifiers::CONTROL | KeyModifiers::ALT) =>
      {
         app.search_insert_char(c)
      },
      KeyCode::Backspace => app.search_delete_char(),
      KeyCode::Delete => app.search_delete_char_forward(),

      // Cursor movement within search
      KeyCode::Left => app.search_cursor_left(),
      KeyCode::Right => app.search_cursor_right(),
      KeyCode::Home => app.search_cursor_start(),
      KeyCode::End => app.search_cursor_end(),

      // Enter - select entry and execute
      KeyCode::Enter => app.select_entry(ExitAction::Execute),
      // Tab - select entry and populate (without executing)
      KeyCode::Tab => app.select_entry(ExitAction::Populate),

      _ => {},
   }
}
