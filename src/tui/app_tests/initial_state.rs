use super::*;

#[test]
fn test_new_app_selected_index_is_zero() {
   let app = make_app();
   assert_eq!(app.selected_index(), 0);
}

#[test]
fn test_new_app_should_not_quit() {
   let app = make_app();
   assert!(!app.should_quit);
}

#[test]
fn test_new_app_exit_action_is_none() {
   let app = make_app();
   assert!(app.exit_action.is_none());
}

#[test]
fn test_new_app_input_mode_is_normal() {
   let app = make_app();
   assert_eq!(app.input_mode(), InputMode::Normal);
}

#[test]
fn test_new_app_all_entries_visible() {
   let app = make_app();
   assert_eq!(app.visible_indices().len(), 4);
}

#[test]
fn test_new_app_search_query_is_empty() {
   let app = make_app();
   assert_eq!(app.search_query(), "");
}
