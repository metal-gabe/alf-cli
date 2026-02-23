//! Normal mode keybinds (vim-style navigation and control).

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::tui::app::App;
use crate::tui::state::EntryFilter;

/// Handle key events in Normal (vim navigation) mode
pub fn handle_normal_mode(app: &mut App, key: KeyEvent) {
   // Check for pending multi-key sequences first
   if let Some(pending) = app.pending_key() {
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

      // Start of 'tj', 'tk' sequences (theme cycling)
      KeyCode::Char('t') => {
         app.set_pending_key('t');
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

      // 'tj' sequence - cycle to next theme
      ('t', KeyCode::Char('j')) => {
         app.cycle_theme_next();
         app.clear_pending_key();
         true
      }

      // 'tk' sequence - cycle to previous theme
      ('t', KeyCode::Char('k')) => {
         app.cycle_theme_prev();
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
