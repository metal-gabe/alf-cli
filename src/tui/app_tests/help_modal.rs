use super::*;

#[test]
fn test_toggle_help_twice_hides_modal() {
   let mut app = make_app();
   assert!(!app.show_help());
   app.toggle_help();
   assert!(app.show_help());
   app.toggle_help();
   assert!(!app.show_help());
}

#[test]
fn test_help_scroll_operations_with_max_set() {
   let mut app = make_app();
   app.update_help_max_scroll(30, 10); // max = 20
   app.help_jump_bottom();
   assert_eq!(app.help_scroll_offset(), 20);
   app.help_jump_top();
   assert_eq!(app.help_scroll_offset(), 0);
}

#[test]
fn test_help_scroll_down_increments() {
   let mut app = make_app();
   app.update_help_max_scroll(20, 5); // max = 15
   app.help_scroll_down();
   assert_eq!(app.help_scroll_offset(), 1);
}

#[test]
fn test_help_scroll_up_decrements() {
   let mut app = make_app();
   app.update_help_max_scroll(20, 5);
   app.help_scroll_down();
   app.help_scroll_down();
   app.help_scroll_up();
   assert_eq!(app.help_scroll_offset(), 1);
}
