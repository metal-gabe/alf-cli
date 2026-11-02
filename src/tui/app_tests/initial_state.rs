use super::*;

#[test]
fn new_app_starts_in_its_initial_state() {
   let app = make_app();
   assert_eq!(app.selected_index(), 0);
   assert!(app.is_running());
   assert!(app.exit_action().is_none());
   assert_eq!(app.input_mode(), InputMode::Normal);
   assert_eq!(app.visible_indices().len(), 4);
   assert_eq!(app.search_query(), "");
}
