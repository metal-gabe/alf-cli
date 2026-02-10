//! Keybinding definitions and handling.
//!
//! Implements vim-style keybindings with mode awareness (Normal vs Search).

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::tui::app::{App, EntryFilter, InputMode};

/// Handle a key event based on the current input mode.
///
/// In Normal mode, vim-style navigation keys are active.
/// In Search mode, printable characters are inserted into the search query.
pub fn handle_key_event(app: &mut App, key: KeyEvent) {
   match app.input_mode {
      InputMode::Normal => handle_normal_mode(app, key),
      InputMode::Search => handle_search_mode(app, key),
   }
}

/// Handle key events in Normal (vim navigation) mode
fn handle_normal_mode(app: &mut App, key: KeyEvent) {
   // Check for pending multi-key sequences first
   if let Some(pending) = app.pending_key.take() {
      handle_pending_key(app, pending, key);
      return;
   }

   match key.code {
      // Quit
      KeyCode::Char('q') => app.should_quit = true,

      // Navigation
      KeyCode::Char('j') | KeyCode::Down => app.move_down(),
      KeyCode::Char('k') | KeyCode::Up => app.move_up(),
      KeyCode::Char('G') => app.move_bottom(),
      KeyCode::Char('g') => {
         // Start of 'gg' sequence
         app.pending_key = Some('g');
      }

      // Scrolling (half page)
      KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
         app.scroll_down(10);
      }
      KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
         app.scroll_up(10);
      }

      // Full page scrolling
      KeyCode::Char('f') if key.modifiers.contains(KeyModifiers::CONTROL) => {
         app.scroll_down(20);
      }
      KeyCode::Char('b') if key.modifiers.contains(KeyModifiers::CONTROL) => {
         app.scroll_up(20);
      }

      // Enter search mode
      KeyCode::Char('/') => app.enter_search_mode(),

      // Panel cycling
      KeyCode::Tab => app.cycle_panel(),

      // Filter toggles
      KeyCode::Char('1') => app.set_filter(EntryFilter::Aliases),
      KeyCode::Char('2') => app.set_filter(EntryFilter::Functions),
      KeyCode::Char('3') => app.set_filter(EntryFilter::All),

      _ => {}
   }
}

/// Handle the second key in a multi-key sequence
fn handle_pending_key(app: &mut App, pending: char, key: KeyEvent) {
   if let ('g', KeyCode::Char('g')) = (pending, key.code) {
      app.move_top();
   }
   // Any other key after 'g' is silently ignored
}

/// Handle key events in Search (text input) mode
fn handle_search_mode(app: &mut App, key: KeyEvent) {
   match key.code {
      // Exit search mode, keep query and filtered results
      KeyCode::Esc => app.exit_search_keep_query(),

      // Text editing
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
