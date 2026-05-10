use super::*;

#[test]
fn test_cycle_panel_forward_three_times_returns_to_list() {
   let mut app = make_app();
   app.cycle_panel();
   app.cycle_panel();
   app.cycle_panel();
   assert_eq!(app.active_panel(), Panel::List);
}

#[test]
fn test_cycle_panel_backward_three_times_returns_to_list() {
   let mut app = make_app();
   app.cycle_panel_backward();
   app.cycle_panel_backward();
   app.cycle_panel_backward();
   assert_eq!(app.active_panel(), Panel::List);
}

#[test]
fn test_cycle_panel_forward_sequence() {
   let mut app = make_app();
   assert_eq!(app.active_panel(), Panel::List);
   app.cycle_panel();
   assert_eq!(app.active_panel(), Panel::Description);
   app.cycle_panel();
   assert_eq!(app.active_panel(), Panel::Script);
}
