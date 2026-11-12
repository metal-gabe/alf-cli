//! Normal mode keybinds (vim-style navigation and control).

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::tui::app::{App, ExitAction};
use crate::tui::state::EntryFilter;

/// Handle key events in Normal (vim navigation) mode
pub fn handle_normal_mode(
   app: &mut App,
   key: KeyEvent,
) {
   if let Some(pending) = app.pending_key() {
      handle_pending_key(app, pending, key);
      return;
   }

   match key.code {
      KeyCode::Char('q') => app.quit(),

      KeyCode::Esc => app.clear_pending_key(),

      KeyCode::Char('?') => {
         app.toggle_help();
         app.clear_pending_key();
      },

      KeyCode::Char('f') if key.modifiers.contains(KeyModifiers::CONTROL) => {
         app.scroll_down(20);
      },
      KeyCode::Char('b') if key.modifiers.contains(KeyModifiers::CONTROL) => {
         app.scroll_up(20);
      },
      KeyCode::Char('j') | KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
         app.scroll_down(10);
      },
      KeyCode::Char('k') | KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
         app.scroll_up(10);
      },
      KeyCode::Char('g') if key.modifiers.contains(KeyModifiers::CONTROL) => {
         app.cycle_group_mode();
      },
      KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
         app.toggle_sort_order();
      },

      KeyCode::Char('j') | KeyCode::Down => app.scroll_down(1),
      KeyCode::Char('k') | KeyCode::Up => app.scroll_up(1),
      KeyCode::Char('G') => app.move_bottom(),
      KeyCode::Char('g') => app.move_top(),

      KeyCode::Char('o') => {
         app.set_pending_key('o');
      },

      KeyCode::Char('t') => {
         app.set_pending_key('t');
      },

      KeyCode::Char('/') | KeyCode::Char('i') => app.enter_search_mode(),

      KeyCode::Char('n') => app.cycle_panel(),
      KeyCode::Char('p') => app.cycle_panel_backward(),
      KeyCode::Char('f') => app.toggle_fullscreen(),

      KeyCode::Char('l') => app.cycle_filter(),
      KeyCode::Char('h') => app.cycle_filter_backward(),

      KeyCode::Char('c') => app.cycle_parse_comments(),

      KeyCode::Char('1') => app.set_filter(EntryFilter::Aliases),
      KeyCode::Char('2') => app.set_filter(EntryFilter::Functions),
      KeyCode::Char('3') => app.set_filter(EntryFilter::All),

      KeyCode::Enter => app.select_entry(ExitAction::Execute),
      KeyCode::Tab => app.select_entry(ExitAction::Populate),

      _ => {},
   }
}

/// Handle the second key in a multi-key sequence
fn handle_pending_key(
   app: &mut App,
   pending: char,
   key: KeyEvent,
) {
   match (pending, key.code) {
      (_, KeyCode::Esc) => {
         app.clear_pending_key();
      },

      ('o', KeyCode::Char('g')) => {
         app.cycle_group_mode();
         app.clear_pending_key();
      },

      ('o', KeyCode::Char('G')) => {
         app.cycle_group_mode_backward();
         app.clear_pending_key();
      },

      ('o', KeyCode::Char('s')) => {
         app.toggle_sort_order();
         app.clear_pending_key();
      },

      ('t', KeyCode::Char('j')) => {
         app.cycle_theme_next();
         app.clear_pending_key();
      },

      ('t', KeyCode::Char('k')) => {
         app.cycle_theme_prev();
         app.clear_pending_key();
      },

      _ => {
         app.clear_pending_key();
      },
   }
}
