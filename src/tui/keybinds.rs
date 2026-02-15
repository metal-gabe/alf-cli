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
   // Check for truly global keybinds first - these work ALWAYS or are reserved
   match key.code {
      // Quit app
      KeyCode::Char('d') | KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
         app.should_quit = true;
         return;
      }
      // Reserved for future use - ignore these globally
      KeyCode::Char('n') | KeyCode::Char('p') if key.modifiers.contains(KeyModifiers::CONTROL) => {
         return;
      }
      _ => {}
   }

   // Check if pending key has expired - clear it if so
   if app.is_pending_key_expired() {
      app.clear_pending_key();
   }

   // If help modal is open, handle help-specific keybinds
   if app.show_help {
      handle_help_mode(app, key);
      return;
   }

   // Handle other global keybinds (work in any mode except help)
   if handle_global_keybinds(app, &key) {
      return;
   }

   // Then handle mode-specific keybinds
   match app.input_mode {
      InputMode::Normal => handle_normal_mode(app, key),
      InputMode::Search => handle_search_mode(app, key),
   }
}

/// Handle global keybinds that work in any mode (except help modal).
/// Returns true if a global keybind was handled.
fn handle_global_keybinds(app: &mut App, key: &KeyEvent) -> bool {
   match key.code {
      // Clear search (Ctrl-u)
      KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
         app.clear_search();
         true
      }
      _ => false,
   }
}

/// Handle key events in Normal (vim navigation) mode
fn handle_normal_mode(app: &mut App, key: KeyEvent) {
   // Check for pending multi-key sequences first
   if let Some(pending) = app.pending_key {
      handle_pending_key(app, pending, key);
      return;
   }

   match key.code {
      // Quit
      KeyCode::Char('q') => app.should_quit = true,

      // Clear pending key state
      KeyCode::Esc => app.clear_pending_key(),

      // Toggle help modal
      KeyCode::Char('?') => {
         app.toggle_help();
         app.clear_pending_key();
      }

      // Scrolling with modifiers (check these first before plain keys)
      KeyCode::Char('f') if key.modifiers.contains(KeyModifiers::CONTROL) => {
         app.scroll_down(20);
      }
      KeyCode::Char('b') if key.modifiers.contains(KeyModifiers::CONTROL) => {
         app.scroll_up(20);
      }
      KeyCode::Char('j') if key.modifiers.contains(KeyModifiers::CONTROL) => {
         app.scroll_down(10);
      }
      KeyCode::Char('k') if key.modifiers.contains(KeyModifiers::CONTROL) => {
         app.scroll_up(10);
      }
      KeyCode::Char('g') if key.modifiers.contains(KeyModifiers::CONTROL) => {
         app.cycle_group_mode();
      }
      KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
         app.toggle_sort_order();
      }

      // Navigation (single line - plain keys, respects active panel)
      KeyCode::Char('j') | KeyCode::Down => app.scroll_down(1),
      KeyCode::Char('k') | KeyCode::Up => app.scroll_up(1),
      KeyCode::Char('G') => app.move_bottom(),
      KeyCode::Char('g') => {
         // Start of 'gg' sequence
         app.set_pending_key('g');
      }

      // Start of 'og', 'oG', 'os' sequences
      KeyCode::Char('o') => {
         app.set_pending_key('o');
      }

      // Enter search mode
      KeyCode::Char('/') => app.enter_search_mode(),

      // Panel cycling
      KeyCode::Char('n') => app.cycle_panel(),
      KeyCode::Char('p') => app.cycle_panel_backward(),

      // Filter cycling
      KeyCode::Char('l') => app.cycle_filter(),
      KeyCode::Char('h') => app.cycle_filter_backward(),

      // Filter direct selection
      KeyCode::Char('1') => app.set_filter(EntryFilter::Aliases),
      KeyCode::Char('2') => app.set_filter(EntryFilter::Functions),
      KeyCode::Char('3') => app.set_filter(EntryFilter::All),

      _ => {}
   }
}

/// Handle the second key in a multi-key sequence
fn handle_pending_key(app: &mut App, pending: char, key: KeyEvent) {
   // Determine if the key sequence is valid
   let _handled = match (pending, key.code) {
      // Clear pending on Esc (always)
      (_, KeyCode::Esc) => {
         app.clear_pending_key();
         true
      }

      // 'gg' sequence - go to top
      ('g', KeyCode::Char('g')) => {
         app.move_top();
         app.clear_pending_key();
         true
      }

      // 'og' sequence - cycle group mode forward
      ('o', KeyCode::Char('g')) => {
         app.cycle_group_mode();
         app.clear_pending_key();
         true
      }

      // 'oG' sequence - cycle group mode backward
      ('o', KeyCode::Char('G')) => {
         app.cycle_group_mode_backward();
         app.clear_pending_key();
         true
      }

      // 'os' sequence - toggle sort order
      ('o', KeyCode::Char('s')) => {
         app.toggle_sort_order();
         app.clear_pending_key();
         true
      }

      // Any other key - invalid sequence, clear pending
      _ => {
         app.clear_pending_key();
         false
      }
   };
}

/// Handle key events in Search (text input) mode
fn handle_search_mode(app: &mut App, key: KeyEvent) {
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

/// Handle key events in Help modal mode
fn handle_help_mode(app: &mut App, key: KeyEvent) {
   // Check for pending multi-key sequences first
   if let Some(pending) = app.pending_key {
      match (pending, key.code) {
         // 'gg' sequence - go to top
         ('g', KeyCode::Char('g')) => {
            app.help_jump_top();
            app.clear_pending_key();
            return;
         }
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

      // Start of 'gg' sequence
      KeyCode::Char('g') => {
         app.set_pending_key('g');
      }

      // Ignore all other keys (prevent propagation to underlying panels)
      _ => {}
   }
}
