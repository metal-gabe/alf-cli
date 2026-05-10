use super::*;

#[test]
fn test_move_down_increments_selection() {
   let mut app = make_app();
   assert_eq!(app.selected_index(), 0);
   app.move_down();
   assert_eq!(app.selected_index(), 1);
}

#[test]
fn test_move_up_decrements_selection() {
   let mut app = make_app();
   app.move_down();
   app.move_down();
   assert_eq!(app.selected_index(), 2);
   app.move_up();
   assert_eq!(app.selected_index(), 1);
}

#[test]
fn test_move_up_at_top_stays_at_zero() {
   let mut app = make_app();
   assert_eq!(app.selected_index(), 0);
   app.move_up();
   assert_eq!(app.selected_index(), 0);
}

#[test]
fn test_move_down_at_last_stays_at_last() {
   let mut app = make_app();
   assert_eq!(app.selected_index(), 0);
   for i in 0..10 {
      app.move_down();
      if i == 0 {
         assert_eq!(app.selected_index(), 1);
      }
      if i == 1 {
         assert_eq!(app.selected_index(), 2);
      }
   }
   assert_eq!(app.selected_index(), 3, "Should not exceed visible_count - 1");
}

#[test]
fn test_move_top_jumps_to_zero() {
   let mut app = make_app();
   app.move_down();
   app.move_down();
   assert_eq!(app.selected_index(), 2);
   app.move_top();
   assert_eq!(app.selected_index(), 0);
}

#[test]
fn test_move_bottom_jumps_to_last() {
   let mut app = make_app();
   assert_eq!(app.selected_index(), 0);
   app.move_bottom();
   assert_eq!(app.selected_index(), 3);
}

#[test]
fn test_move_down_on_empty_app_no_panic() {
   let mut app = make_empty_app();
   app.move_down();
   assert_eq!(app.selected_index(), 0);
}

#[test]
fn test_move_bottom_on_empty_app_no_panic() {
   let mut app = make_empty_app();
   app.move_bottom();
   assert_eq!(app.selected_index(), 0);
}
