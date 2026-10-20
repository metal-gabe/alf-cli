use super::*;

#[test]
fn enter_search_mode_sets_search_input_mode() {
   let mut app = make_app();
   app.enter_search_mode();
   assert_eq!(app.input_mode(), InputMode::Search);
}

#[test]
fn exit_search_keep_query_returns_to_normal() {
   let mut app = make_app();
   app.enter_search_mode();
   app.search_insert_char('a');
   app.exit_search_keep_query();
   assert_eq!(app.input_mode(), InputMode::Normal);
   assert_eq!(app.search_query(), "a", "Query should be preserved");
}

#[test]
fn exit_search_clear_query_returns_to_normal_and_clears() {
   let mut app = make_app();
   app.enter_search_mode();
   app.search_insert_char('a');
   app.exit_search_clear_query();
   assert_eq!(app.input_mode(), InputMode::Normal);
   assert_eq!(app.search_query(), "", "Query should be cleared");
}

#[test]
fn search_insert_char_filters_visible_entries() {
   let mut app = make_app();
   app.set_search_query("alpha".to_string());
   assert_eq!(app.visible_indices().len(), 1);
}

#[test]
fn search_delete_char_updates_visible_entries() {
   let mut app = make_app();
   app.set_search_query("alpha".to_string());
   for _ in 0.."alpha".len() {
      app.search_delete_char();
   }
   assert_eq!(app.search_query(), "");
   assert_eq!(app.visible_indices().len(), 4);
}

#[test]
fn set_search_query_filters_by_name() {
   let mut app = make_app();
   app.set_search_query("gamma".to_string());
   assert_eq!(app.visible_indices().len(), 1);
   assert_eq!(app.selected_entry().unwrap().name, "gamma");
}

#[test]
fn set_search_query_no_match_empties_visible() {
   let mut app = make_app();
   app.set_search_query("zzznomatch".to_string());
   assert_eq!(app.visible_indices().len(), 0);
}

#[test]
fn clear_search_restores_all_entries() {
   let mut app = make_app();
   app.set_search_query("alpha".to_string());
   app.clear_search();
   assert_eq!(app.search_query(), "");
   assert_eq!(app.visible_indices().len(), 4);
}

#[test]
fn search_cursor_left_and_right() {
   let mut app = make_app();
   app.set_search_query("abc".to_string());
   assert_eq!(app.cursor_position(), 3);
   app.search_cursor_left();
   assert_eq!(app.cursor_position(), 2);
   app.search_cursor_right();
   assert_eq!(app.cursor_position(), 3);
}

type AppOp = fn(&mut App);

fn app_with_cursor(
   query: &str,
   cursor: usize,
) -> App {
   let mut app = make_app();
   app.set_search_query(query.to_string());
   for _ in cursor..query.chars().count() {
      app.search_cursor_left();
   }
   app
}

#[test]
fn search_delete_wrappers_update_visible_entries() {
   let cases: [(&str, AppOp, &str, usize, &str, usize); 3] = [
      ("search_delete_word_backward", App::search_delete_word_backward, "zzz", 3, "", 4),
      ("search_delete_word_forward", App::search_delete_word_forward, "zzz", 0, "", 4),
      ("search_delete_char_forward", App::search_delete_char_forward, "alphaz", 5, "alpha", 1),
   ];
   for (name, op, query, cursor, expected_query, expected_visible) in cases {
      let mut app = app_with_cursor(query, cursor);
      assert_eq!(app.visible_indices().len(), 0, "{name}: {query:?} should start with no matches");
      op(&mut app);
      assert_eq!(app.search_query(), expected_query, "{name}");
      assert_eq!(app.visible_indices().len(), expected_visible, "{name}");
   }
}

#[test]
fn search_cursor_wrappers_move_cursor_without_changing_query() {
   let cases: [(&str, AppOp, usize, usize); 4] = [
      ("search_cursor_start", App::search_cursor_start, 4, 0),
      ("search_cursor_end", App::search_cursor_end, 0, 6),
      ("search_cursor_word_left", App::search_cursor_word_left, 6, 4),
      ("search_cursor_word_right", App::search_cursor_word_right, 0, 3),
   ];
   for (name, op, cursor, expected_cursor) in cases {
      let mut app = app_with_cursor("git-co", cursor);
      op(&mut app);
      assert_eq!(app.search_query(), "git-co", "{name}");
      assert_eq!(app.cursor_position(), expected_cursor, "{name}");
   }
}
