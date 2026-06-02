use super::*;

#[test]
fn test_scroll_down_on_list_panel_jumps_selection() {
   let mut app = make_app();
   assert_eq!(app.selected_index(), 0);
   assert_eq!(app.active_panel(), Panel::List);
   app.scroll_down(2);
   assert_eq!(app.selected_index(), 2);
}

#[test]
fn test_scroll_up_on_list_panel_saturates_at_zero() {
   let mut app = make_app();
   app.scroll_up(100);
   assert_eq!(app.selected_index(), 0);
}
