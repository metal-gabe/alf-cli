use super::*;

#[test]
fn test_enter_search_mode_sets_search_input_mode() {
   let mut app = make_app();
   app.enter_search_mode();
   assert_eq!(app.input_mode(), InputMode::Search);
}

#[test]
fn test_exit_search_keep_query_returns_to_normal() {
   let mut app = make_app();
   app.enter_search_mode();
   app.search_insert_char('a');
   app.exit_search_keep_query();
   assert_eq!(app.input_mode(), InputMode::Normal);
   assert_eq!(app.search_query(), "a", "Query should be preserved");
}

#[test]
fn test_exit_search_clear_query_returns_to_normal_and_clears() {
   let mut app = make_app();
   app.enter_search_mode();
   app.search_insert_char('a');
   app.exit_search_clear_query();
   assert_eq!(app.input_mode(), InputMode::Normal);
   assert_eq!(app.search_query(), "", "Query should be cleared");
}

#[test]
fn test_search_insert_char_filters_visible_entries() {
   let mut app = make_app();
   app.set_search_query("alpha".to_string());
   assert_eq!(app.visible_indices().len(), 1);
}

#[test]
fn test_search_delete_char_updates_visible_entries() {
   let mut app = make_app();
   app.set_search_query("alpha".to_string());
   // Delete until query is empty
   for _ in 0.."alpha".len() {
      app.search_delete_char();
   }
   assert_eq!(app.search_query(), "");
   assert_eq!(app.visible_indices().len(), 4);
}

#[test]
fn test_set_search_query_filters_by_name() {
   let mut app = make_app();
   app.set_search_query("gamma".to_string());
   assert_eq!(app.visible_indices().len(), 1);
   assert_eq!(app.selected_entry().unwrap().name, "gamma");
}

#[test]
fn test_set_search_query_no_match_empties_visible() {
   let mut app = make_app();
   app.set_search_query("zzznomatch".to_string());
   assert_eq!(app.visible_indices().len(), 0);
}

#[test]
fn test_clear_search_restores_all_entries() {
   let mut app = make_app();
   app.set_search_query("alpha".to_string());
   app.clear_search();
   assert_eq!(app.search_query(), "");
   assert_eq!(app.visible_indices().len(), 4);
}

#[test]
fn test_search_cursor_left_and_right() {
   let mut app = make_app();
   app.set_search_query("abc".to_string());
   assert_eq!(app.cursor_position(), 3);
   app.search_cursor_left();
   assert_eq!(app.cursor_position(), 2);
   app.search_cursor_right();
   assert_eq!(app.cursor_position(), 3);
}
