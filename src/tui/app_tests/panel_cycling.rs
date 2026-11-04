use super::*;

#[test]
fn app_forwards_panel_cycling_to_ui_state() {
   let mut app = make_app();
   app.cycle_panel();
   assert_eq!(app.active_panel(), Panel::Description);
   app.cycle_panel_backward();
   assert_eq!(app.active_panel(), Panel::List);
}
