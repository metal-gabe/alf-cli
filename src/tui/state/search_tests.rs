//! Tests for SearchState (query manipulation and cursor tracking)

use super::SearchState;

// ===== is_empty / new =====

#[test]
fn test_new_state_is_empty() {
   let state = SearchState::new();
   assert!(state.is_empty());
   assert_eq!(state.query(), "");
   assert_eq!(state.cursor_position(), 0);
}

// ===== insert_char =====

#[test]
fn test_insert_char_appends_at_end() {
   let mut state = SearchState::new();
   state.insert_char('a');
   state.insert_char('b');
   state.insert_char('c');
   assert_eq!(state.query(), "abc");
   assert_eq!(state.cursor_position(), 3);
}

#[test]
fn test_insert_char_converts_uppercase_to_lowercase() {
   let mut state = SearchState::new();
   state.insert_char('A');
   state.insert_char('B');
   assert_eq!(state.query(), "ab");
}

#[test]
fn test_insert_char_at_mid_position() {
   let mut state = SearchState::new();
   state.insert_char('a');
   state.insert_char('c');
   state.move_cursor_left();
   state.insert_char('b');
   assert_eq!(state.query(), "abc");
   assert_eq!(state.cursor_position(), 2);
}

#[test]
fn test_insert_char_special_chars_unchanged() {
   let mut state = SearchState::new();
   state.insert_char('!');
   state.insert_char('-');
   state.insert_char('_');
   assert_eq!(state.query(), "!-_");
}

#[test]
fn test_insert_char_utf8_multibyte() {
   let mut state = SearchState::new();
   state.insert_char('é');
   state.insert_char('à');
   assert_eq!(state.query(), "éà");
   assert_eq!(state.cursor_position(), 2);
}

// ===== delete_char =====

#[test]
fn test_delete_char_removes_last_character() {
   let mut state = SearchState::new();
   state.insert_char('a');
   state.insert_char('b');
   state.insert_char('c');
   state.delete_char();
   assert_eq!(state.query(), "ab");
   assert_eq!(state.cursor_position(), 2);
}

#[test]
fn test_delete_char_at_start_is_noop() {
   let mut state = SearchState::new();
   state.delete_char();
   assert_eq!(state.query(), "");
   assert_eq!(state.cursor_position(), 0);
}

#[test]
fn test_delete_char_removes_mid_character() {
   let mut state = SearchState::new();
   state.insert_char('a');
   state.insert_char('b');
   state.insert_char('c');
   state.move_cursor_left();
   state.delete_char();
   assert_eq!(state.query(), "ac");
   assert_eq!(state.cursor_position(), 1);
}

#[test]
fn test_delete_char_utf8_multibyte() {
   let mut state = SearchState::new();
   state.insert_char('é');
   state.insert_char('à');
   state.delete_char();
   assert_eq!(state.query(), "é");
   assert_eq!(state.cursor_position(), 1);
}

// ===== move_cursor_left =====

#[test]
fn test_move_cursor_left_decrements() {
   let mut state = SearchState::new();
   state.insert_char('a');
   state.insert_char('b');
   state.move_cursor_left();
   assert_eq!(state.cursor_position(), 1);
}

#[test]
fn test_move_cursor_left_stops_at_zero() {
   let mut state = SearchState::new();
   state.move_cursor_left();
   assert_eq!(state.cursor_position(), 0);
}

// ===== move_cursor_right =====

#[test]
fn test_move_cursor_right_increments() {
   let mut state = SearchState::new();
   state.insert_char('a');
   state.insert_char('b');
   state.move_cursor_left();
   state.move_cursor_left();
   state.move_cursor_right();
   assert_eq!(state.cursor_position(), 1);
}

#[test]
fn test_move_cursor_right_stops_at_end() {
   let mut state = SearchState::new();
   state.insert_char('a');
   state.move_cursor_right();
   assert_eq!(state.cursor_position(), 1, "Should not exceed char count");
}

// ===== set_query =====

#[test]
fn test_set_query_stores_lowercased() {
   let mut state = SearchState::new();
   state.set_query("Hello World".to_string());
   assert_eq!(state.query(), "hello world");
}

#[test]
fn test_set_query_positions_cursor_at_end() {
   let mut state = SearchState::new();
   state.set_query("abc".to_string());
   assert_eq!(state.cursor_position(), 3);
}

#[test]
fn test_set_query_replaces_existing_query() {
   let mut state = SearchState::new();
   state.insert_char('x');
   state.set_query("new".to_string());
   assert_eq!(state.query(), "new");
   assert_eq!(state.cursor_position(), 3);
}

// ===== clear =====

#[test]
fn test_clear_empties_query() {
   let mut state = SearchState::new();
   state.set_query("hello".to_string());
   state.clear();
   assert_eq!(state.query(), "");
   assert!(state.is_empty());
}

#[test]
fn test_clear_resets_cursor_to_zero() {
   let mut state = SearchState::new();
   state.set_query("hello".to_string());
   state.clear();
   assert_eq!(state.cursor_position(), 0);
}
