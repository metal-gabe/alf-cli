//! Tests for SearchState (query manipulation and cursor tracking)

use super::SearchState;

// ===== is_empty / new =====

#[test]
fn new_state_is_empty() {
   let state = SearchState::new();
   assert!(state.is_empty());
   assert_eq!(state.query(), "");
   assert_eq!(state.cursor_position(), 0);
}

// ===== insert_char =====

#[test]
fn insert_char_appends_at_end() {
   let mut state = SearchState::new();
   state.insert_char('a');
   state.insert_char('b');
   state.insert_char('c');
   assert_eq!(state.query(), "abc");
   assert_eq!(state.cursor_position(), 3);
}

#[test]
fn insert_char_converts_uppercase_to_lowercase() {
   let mut state = SearchState::new();
   state.insert_char('A');
   state.insert_char('B');
   assert_eq!(state.query(), "ab");
}

#[test]
fn insert_char_at_mid_position() {
   let mut state = SearchState::new();
   state.insert_char('a');
   state.insert_char('c');
   state.move_cursor_left();
   state.insert_char('b');
   assert_eq!(state.query(), "abc");
   assert_eq!(state.cursor_position(), 2);
}

#[test]
fn insert_char_special_chars_unchanged() {
   let mut state = SearchState::new();
   state.insert_char('!');
   state.insert_char('-');
   state.insert_char('_');
   assert_eq!(state.query(), "!-_");
}

#[test]
fn insert_char_utf8_multibyte() {
   let mut state = SearchState::new();
   state.insert_char('é');
   state.insert_char('à');
   assert_eq!(state.query(), "éà");
   assert_eq!(state.cursor_position(), 2);
}

// ===== delete_char =====

#[test]
fn delete_char_removes_last_character() {
   let mut state = SearchState::new();
   state.insert_char('a');
   state.insert_char('b');
   state.insert_char('c');
   state.delete_char();
   assert_eq!(state.query(), "ab");
   assert_eq!(state.cursor_position(), 2);
}

#[test]
fn delete_char_at_start_is_noop() {
   let mut state = SearchState::new();
   state.delete_char();
   assert_eq!(state.query(), "");
   assert_eq!(state.cursor_position(), 0);
}

#[test]
fn delete_char_removes_mid_character() {
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
fn delete_char_utf8_multibyte() {
   let mut state = SearchState::new();
   state.insert_char('é');
   state.insert_char('à');
   state.delete_char();
   assert_eq!(state.query(), "é");
   assert_eq!(state.cursor_position(), 1);
}

// ===== move_cursor_left =====

#[test]
fn move_cursor_left_decrements() {
   let mut state = SearchState::new();
   state.insert_char('a');
   state.insert_char('b');
   state.move_cursor_left();
   assert_eq!(state.cursor_position(), 1);
}

#[test]
fn move_cursor_left_stops_at_zero() {
   let mut state = SearchState::new();
   state.move_cursor_left();
   assert_eq!(state.cursor_position(), 0);
}

// ===== move_cursor_right =====

#[test]
fn move_cursor_right_increments() {
   let mut state = SearchState::new();
   state.insert_char('a');
   state.insert_char('b');
   state.move_cursor_left();
   state.move_cursor_left();
   state.move_cursor_right();
   assert_eq!(state.cursor_position(), 1);
}

#[test]
fn move_cursor_right_stops_at_end() {
   let mut state = SearchState::new();
   state.insert_char('a');
   state.move_cursor_right();
   assert_eq!(state.cursor_position(), 1, "Should not exceed char count");
}

// ===== set_query =====

#[test]
fn set_query_stores_lowercased() {
   let mut state = SearchState::new();
   state.set_query("Hello World".to_string());
   assert_eq!(state.query(), "hello world");
}

#[test]
fn set_query_positions_cursor_at_end() {
   let mut state = SearchState::new();
   state.set_query("abc".to_string());
   assert_eq!(state.cursor_position(), 3);
}

#[test]
fn set_query_replaces_existing_query() {
   let mut state = SearchState::new();
   state.insert_char('x');
   state.set_query("new".to_string());
   assert_eq!(state.query(), "new");
   assert_eq!(state.cursor_position(), 3);
}

// ===== clear =====

#[test]
fn clear_empties_query() {
   let mut state = SearchState::new();
   state.set_query("hello".to_string());
   state.clear();
   assert_eq!(state.query(), "");
   assert!(state.is_empty());
}

#[test]
fn clear_resets_cursor_to_zero() {
   let mut state = SearchState::new();
   state.set_query("hello".to_string());
   state.clear();
   assert_eq!(state.cursor_position(), 0);
}

// ===== word & line editing =====

fn state_from(marked: &str) -> SearchState {
   let (before, after) = marked.split_once('|').expect("marked query must contain a '|' cursor");
   let mut state = SearchState::new();
   state.set_query(format!("{before}{after}"));
   for _ in 0..after.chars().count() {
      state.move_cursor_left();
   }
   state
}

fn marked(state: &SearchState) -> String {
   let chars: Vec<char> = state.query().chars().collect();
   let before: String = chars[..state.cursor_position()].iter().collect();
   let after: String = chars[state.cursor_position()..].iter().collect();
   format!("{before}|{after}")
}

fn assert_cases(
   name: &str,
   op: fn(&mut SearchState),
   cases: &[(&str, &str)],
) {
   for (start, expected) in cases {
      let mut state = state_from(start);
      op(&mut state);
      assert_eq!(marked(&state), *expected, "{name} on {start:?}");
   }
}

#[test]
fn delete_word_backward_cases() {
   assert_cases(
      "delete_word_backward",
      SearchState::delete_word_backward,
      &[
         ("git-co|", "git-|"),
         ("git-|", "git|"),
         ("git-checkout |", "git-|"),
         ("foo--|", "foo|"),
         ("git-|co", "git|co"),
         ("dc_up|", "|"),
         ("   |", "|"),
         ("|abc", "|abc"),
         ("|", "|"),
      ],
   );
}

#[test]
fn delete_word_forward_cases() {
   assert_cases(
      "delete_word_forward",
      SearchState::delete_word_forward,
      &[
         ("|git-co", "|-co"),
         ("|git checkout", "|checkout"),
         ("git|-co", "git|co"),
         ("|foo--bar", "|--bar"),
         ("|   abc", "|abc"),
         ("abc|", "abc|"),
         ("|", "|"),
      ],
   );
}

#[test]
fn move_word_backward_cases() {
   assert_cases(
      "move_word_backward",
      SearchState::move_word_backward,
      &[
         ("git-co|", "git-|co"),
         ("git-|co", "git|-co"),
         ("git-checkout |", "git-|checkout "),
         ("foo--|", "foo|--"),
         ("dc_up|", "|dc_up"),
         ("   |", "|   "),
         ("|abc", "|abc"),
         ("|", "|"),
      ],
   );
}

#[test]
fn move_word_forward_cases() {
   assert_cases(
      "move_word_forward",
      SearchState::move_word_forward,
      &[
         ("|git-co", "git|-co"),
         ("git|-co", "git-|co"),
         ("|git checkout", "git |checkout"),
         ("|foo--bar", "foo|--bar"),
         ("|   abc", "   |abc"),
         ("abc|", "abc|"),
         ("|", "|"),
      ],
   );
}

#[test]
fn delete_char_forward_cases() {
   assert_cases(
      "delete_char_forward",
      SearchState::delete_char_forward,
      &[("|abc", "|bc"), ("a|bc", "a|c"), ("abc|", "abc|"), ("|", "|")],
   );
}

#[test]
fn move_cursor_start_and_end_cases() {
   assert_cases("move_cursor_start", SearchState::move_cursor_start, &[("ab|c", "|abc"), ("|abc", "|abc"), ("|", "|")]);
   assert_cases("move_cursor_end", SearchState::move_cursor_end, &[("ab|c", "abc|"), ("abc|", "abc|"), ("|", "|")]);
}

#[test]
fn word_editing_is_utf8_safe() {
   assert_cases(
      "delete_word_backward",
      SearchState::delete_word_backward,
      &[("café-bar|", "café-|"), ("café|", "|"), ("foo🎉🎉|", "foo|")],
   );
   assert_cases(
      "delete_word_forward",
      SearchState::delete_word_forward,
      &[("|naïve café", "|café"), ("|🎉bar", "|bar")],
   );
   assert_cases("move_word_backward", SearchState::move_word_backward, &[("naïve café|", "naïve |café")]);
   assert_cases("move_word_forward", SearchState::move_word_forward, &[("|naïve café", "naïve |café")]);
   assert_cases("delete_char_forward", SearchState::delete_char_forward, &[("é|àb", "é|b"), ("|🎉x", "|x")]);
}
