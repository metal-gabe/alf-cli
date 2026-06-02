use super::*;

#[test]
fn test_cycle_group_mode_changes_group() {
   let mut app = make_app();
   assert_eq!(app.group_mode(), GroupMode::Aliases);
   app.cycle_group_mode();
   assert_eq!(app.group_mode(), GroupMode::Functions);
}

#[test]
fn test_cycle_group_mode_backward_changes_group() {
   let mut app = make_app();
   app.cycle_group_mode_backward();
   assert_eq!(app.group_mode(), GroupMode::None);
}

#[test]
fn test_toggle_sort_order_changes_order() {
   let mut app = make_app();
   assert_eq!(app.sort_order(), SortOrder::Ascending);
   app.toggle_sort_order();
   assert_eq!(app.sort_order(), SortOrder::Descending);
}

#[test]
fn test_toggle_sort_order_reverses_visible_entry_order() {
   let mut app = make_app();
   app.set_filter(EntryFilter::Aliases);
   let first_asc = app.selected_entry().unwrap().name.clone();
   app.toggle_sort_order();
   let first_desc = app.selected_entry().unwrap().name.clone();
   assert_ne!(first_asc, first_desc, "Toggling sort order should change first entry");
}
