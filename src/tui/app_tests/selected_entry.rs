use super::*;

#[test]
fn test_selected_entry_returns_first_visible_by_default() {
   let app = make_app();
   assert_eq!(app.selected_entry().unwrap().name, "alpha");
}

#[test]
fn test_selected_entry_returns_none_when_empty() {
   let app = make_empty_app();
   assert!(app.selected_entry().is_none());
}
