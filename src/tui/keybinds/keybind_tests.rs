//! Tests for the keybind dispatch layer

use super::handle_key_event;
use crate::models::{AliasEntry, EntryType};
use crate::tui::app::{App, ExitAction};
use crate::tui::state::InputMode;
use crate::tui::themes::Theme;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::path::PathBuf;

fn make_entry(name: &str, entry_type: EntryType, value: &str) -> AliasEntry {
   AliasEntry {
      name: name.to_string(),
      entry_type,
      value: value.to_string(),
      comments: None,
      source_file: PathBuf::from("test.sh"),
   }
}

fn make_app() -> App {
   let entries = vec![
      make_entry("beta", EntryType::Alias, "cmd_beta"),
      make_entry("alpha", EntryType::Alias, "cmd_alpha"),
      make_entry("gamma", EntryType::Function, "{ body_gamma }"),
      make_entry("delta", EntryType::Function, "{ body_delta }"),
   ];
   App::new(entries, Theme::default())
}

fn make_empty_app() -> App {
   App::new(vec![], Theme::default())
}

fn key(code: KeyCode) -> KeyEvent {
   KeyEvent::new(code, KeyModifiers::NONE)
}

fn ctrl(c: char) -> KeyEvent {
   KeyEvent::new(KeyCode::Char(c), KeyModifiers::CONTROL)
}

// ===== Normal mode — Enter/Tab (enter-tab logic) =====

#[test]
fn test_enter_in_normal_mode_sets_execute_action() {
   let mut app = make_app();
   handle_key_event(&mut app, key(KeyCode::Enter));
   assert!(app.should_quit);
   assert!(matches!(app.exit_action, Some(ExitAction::Execute)));
}

#[test]
fn test_tab_in_normal_mode_sets_populate_action() {
   let mut app = make_app();
   handle_key_event(&mut app, key(KeyCode::Tab));
   assert!(app.should_quit);
   assert!(matches!(app.exit_action, Some(ExitAction::Populate)));
}

#[test]
fn test_enter_in_normal_mode_on_empty_app_quits_with_no_action() {
   let mut app = make_empty_app();
   handle_key_event(&mut app, key(KeyCode::Enter));
   assert!(app.should_quit);
   assert!(app.exit_action.is_none());
}

#[test]
fn test_tab_in_normal_mode_on_empty_app_quits_with_no_action() {
   let mut app = make_empty_app();
   handle_key_event(&mut app, key(KeyCode::Tab));
   assert!(app.should_quit);
   assert!(app.exit_action.is_none());
}

// ===== Normal mode — navigation & mode switching =====

#[test]
fn test_q_in_normal_mode_quits() {
   let mut app = make_app();
   handle_key_event(&mut app, key(KeyCode::Char('q')));
   assert!(app.should_quit);
}

#[test]
fn test_slash_in_normal_mode_enters_search() {
   let mut app = make_app();
   handle_key_event(&mut app, key(KeyCode::Char('/')));
   assert_eq!(app.input_mode(), InputMode::Search);
}

#[test]
fn test_i_in_normal_mode_enters_search() {
   let mut app = make_app();
   handle_key_event(&mut app, key(KeyCode::Char('i')));
   assert_eq!(app.input_mode(), InputMode::Search);
}

// ===== Search mode — Enter/Tab (enter-tab logic) =====

#[test]
fn test_enter_in_search_mode_sets_execute_action() {
   let mut app = make_app();
   app.enter_search_mode();
   handle_key_event(&mut app, key(KeyCode::Enter));
   assert!(app.should_quit);
   assert!(matches!(app.exit_action, Some(ExitAction::Execute)));
}

#[test]
fn test_tab_in_search_mode_sets_populate_action() {
   let mut app = make_app();
   app.enter_search_mode();
   handle_key_event(&mut app, key(KeyCode::Tab));
   assert!(app.should_quit);
   assert!(matches!(app.exit_action, Some(ExitAction::Populate)));
}

// ===== Search mode — text input & navigation =====

#[test]
fn test_char_in_search_mode_appends_to_query() {
   let mut app = make_app();
   app.enter_search_mode();
   handle_key_event(&mut app, key(KeyCode::Char('a')));
   assert_eq!(app.search_query(), "a");
}

#[test]
fn test_backspace_in_search_mode_removes_last_char() {
   let mut app = make_app();
   app.enter_search_mode();
   handle_key_event(&mut app, key(KeyCode::Char('a')));
   handle_key_event(&mut app, key(KeyCode::Backspace));
   assert!(app.search_query().is_empty());
}

#[test]
fn test_esc_in_search_mode_returns_to_normal() {
   let mut app = make_app();
   app.enter_search_mode();
   handle_key_event(&mut app, key(KeyCode::Esc));
   assert_eq!(app.input_mode(), InputMode::Normal);
}

// ===== Global keybinds =====

#[test]
fn test_ctrl_c_always_quits() {
   let mut app = make_app();
   handle_key_event(&mut app, ctrl('c'));
   assert!(app.should_quit);
}

#[test]
fn test_ctrl_d_always_quits() {
   let mut app = make_app();
   handle_key_event(&mut app, ctrl('d'));
   assert!(app.should_quit);
}

#[test]
fn test_ctrl_u_clears_search_query() {
   let mut app = make_app();
   app.set_search_query("hello".to_string());
   handle_key_event(&mut app, ctrl('u'));
   assert!(app.search_query().is_empty());
}

// ===== Multi-key sequences =====

#[test]
fn test_gg_sequence_moves_to_top() {
   let mut app = make_app();
   app.move_down();
   assert_eq!(app.selected_index(), 1);
   handle_key_event(&mut app, key(KeyCode::Char('g')));
   handle_key_event(&mut app, key(KeyCode::Char('g')));
   assert_eq!(app.selected_index(), 0);
}

#[test]
fn test_og_sequence_cycles_group_mode_forward() {
   let mut app = make_app();
   let before = app.group_mode();
   handle_key_event(&mut app, key(KeyCode::Char('o')));
   handle_key_event(&mut app, key(KeyCode::Char('g')));
   assert_ne!(std::mem::discriminant(&app.group_mode()), std::mem::discriminant(&before));
}

#[test]
fn test_og_upper_sequence_cycles_group_mode_backward() {
   let mut app = make_app();
   let before = app.group_mode();
   handle_key_event(&mut app, key(KeyCode::Char('o')));
   handle_key_event(&mut app, key(KeyCode::Char('G')));
   assert_ne!(std::mem::discriminant(&app.group_mode()), std::mem::discriminant(&before));
}

#[test]
fn test_pending_key_cleared_on_esc() {
   let mut app = make_app();
   handle_key_event(&mut app, key(KeyCode::Char('o')));
   assert!(app.pending_key().is_some());
   handle_key_event(&mut app, key(KeyCode::Esc));
   assert!(app.pending_key().is_none());
}

#[test]
fn test_invalid_sequence_clears_pending_key() {
   let mut app = make_app();
   handle_key_event(&mut app, key(KeyCode::Char('o')));
   assert!(app.pending_key().is_some());
   handle_key_event(&mut app, key(KeyCode::Char('z')));
   assert!(app.pending_key().is_none());
}

// ===== Help modal =====

#[test]
fn test_question_mark_toggles_help_open() {
   let mut app = make_app();
   handle_key_event(&mut app, key(KeyCode::Char('?')));
   assert!(app.show_help());
}

#[test]
fn test_esc_closes_help_modal() {
   let mut app = make_app();
   app.toggle_help();
   handle_key_event(&mut app, key(KeyCode::Esc));
   assert!(!app.show_help());
}

#[test]
fn test_q_closes_help_modal() {
   let mut app = make_app();
   app.toggle_help();
   handle_key_event(&mut app, key(KeyCode::Char('q')));
   assert!(!app.show_help());
}
