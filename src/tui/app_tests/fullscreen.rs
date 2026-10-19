use super::*;

#[test]
fn fullscreen_is_off_by_default() {
   let app = make_app();
   assert!(!app.is_fullscreen());
}

#[test]
fn toggle_fullscreen_twice_returns_to_off() {
   let mut app = make_app();
   app.toggle_fullscreen();
   assert!(app.is_fullscreen());
   app.toggle_fullscreen();
   assert!(!app.is_fullscreen());
}

#[test]
fn fullscreen_persists_across_panel_cycling() {
   let cases = [
      (App::cycle_panel as fn(&mut App), [Panel::Description, Panel::Script, Panel::List]),
      (App::cycle_panel_backward, [Panel::Script, Panel::Description, Panel::List]),
   ];
   for (cycle, expected_panels) in cases {
      let mut app = make_app();
      app.toggle_fullscreen();
      for expected_panel in expected_panels {
         cycle(&mut app);
         assert!(app.is_fullscreen());
         assert_eq!(app.active_panel(), expected_panel);
      }
   }
}
