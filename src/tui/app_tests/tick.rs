use super::*;

#[test]
fn test_tick_with_no_pending_key_no_panic() {
   let mut app = make_app();
   app.tick();
}

#[test]
fn test_tick_does_not_alter_state_when_key_fresh() {
   let mut app = make_app();
   app.set_pending_key('g');
   app.tick();
   assert!(app.pending_key().is_some());
}
