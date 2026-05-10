use super::*;

#[test]
fn test_set_filter_aliases_reduces_visible_count() {
   let mut app = make_app();
   app.set_filter(EntryFilter::Aliases);
   assert_eq!(app.filter(), EntryFilter::Aliases);
   assert_eq!(app.visible_indices().len(), 2);
}

#[test]
fn test_set_filter_functions_reduces_visible_count() {
   let mut app = make_app();
   app.set_filter(EntryFilter::Functions);
   assert_eq!(app.filter(), EntryFilter::Functions);
   assert_eq!(app.visible_indices().len(), 2);
}

#[test]
fn test_set_filter_all_restores_full_count() {
   let mut app = make_app();
   app.set_filter(EntryFilter::Aliases);
   app.set_filter(EntryFilter::All);
   assert_eq!(app.visible_indices().len(), 4);
}

#[test]
fn test_cycle_filter_forward_full_cycle() {
   let mut app = make_app();
   assert_eq!(app.filter(), EntryFilter::All);
   app.cycle_filter();
   assert_eq!(app.filter(), EntryFilter::Aliases);
   app.cycle_filter();
   assert_eq!(app.filter(), EntryFilter::Functions);
   app.cycle_filter();
   assert_eq!(app.filter(), EntryFilter::All);
}

#[test]
fn test_cycle_filter_backward_full_cycle() {
   let mut app = make_app();
   app.cycle_filter_backward();
   assert_eq!(app.filter(), EntryFilter::Functions);
   app.cycle_filter_backward();
   assert_eq!(app.filter(), EntryFilter::Aliases);
   app.cycle_filter_backward();
   assert_eq!(app.filter(), EntryFilter::All);
}

#[test]
fn test_filter_clamps_selection_when_count_shrinks() {
   let mut app = make_app();
   app.move_bottom(); // Select last (index 3)
   app.set_filter(EntryFilter::Aliases); // Now only 2 visible
   assert!(app.selected_index() < 2, "Selection should clamp to new visible count");
}
