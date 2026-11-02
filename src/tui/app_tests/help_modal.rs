use super::*;

#[test]
fn app_forwards_help_toggling_to_ui_state() {
   let mut app = make_app();
   assert!(!app.is_help_visible());
   app.toggle_help();
   assert!(app.is_help_visible());
   app.toggle_help();
   assert!(!app.is_help_visible());
}

#[test]
fn app_forwards_help_scrolling_to_ui_state() {
   let mut app = make_app();
   app.update_help_max_scroll(30, 10);
   app.help_jump_bottom();
   assert_eq!(app.help_scroll_offset(), 20);
   app.help_jump_top();
   assert_eq!(app.help_scroll_offset(), 0);
   app.help_scroll_down();
   app.help_scroll_down();
   assert_eq!(app.help_scroll_offset(), 2);
   app.help_scroll_up();
   assert_eq!(app.help_scroll_offset(), 1);
}
