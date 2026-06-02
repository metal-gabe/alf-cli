use super::*;

#[test]
fn test_cycle_theme_next_changes_theme_name() {
   let mut app = make_app();
   let initial_name = app.theme().name.clone();
   app.cycle_theme_next();
   assert_ne!(app.theme().name, initial_name, "Theme should change after cycling next");
}

#[test]
fn test_cycle_theme_prev_changes_theme_name() {
   let mut app = make_app();
   let initial_name = app.theme().name.clone();
   app.cycle_theme_prev();
   assert_ne!(app.theme().name, initial_name, "Theme should change after cycling prev");
}

#[test]
fn test_cycle_theme_next_full_cycle_returns_to_start() {
   let mut app = make_app();
   let initial_name = app.theme().name.clone();
   let theme_count = Theme::available_themes().len();
   for _ in 0..theme_count {
      app.cycle_theme_next();
   }
   assert_eq!(app.theme().name, initial_name, "Full cycle should return to starting theme");
}

#[test]
fn test_cycle_theme_prev_full_cycle_returns_to_start() {
   let mut app = make_app();
   let initial_name = app.theme().name.clone();
   let theme_count = Theme::available_themes().len();
   for _ in 0..theme_count {
      app.cycle_theme_prev();
   }
   assert_eq!(app.theme().name, initial_name, "Full backward cycle should return to starting theme");
}

#[test]
fn test_cycle_theme_next_then_prev_returns_to_original() {
   let mut app = make_app();
   let initial_name = app.theme().name.clone();
   app.cycle_theme_next();
   app.cycle_theme_prev();
   assert_eq!(app.theme().name, initial_name);
}
