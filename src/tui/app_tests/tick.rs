use super::*;

#[test]
fn tick_leaves_state_alone_without_an_expired_key() {
   let mut app = make_app();
   app.tick();
   assert!(app.pending_key().is_none());
   assert!(app.is_running());
   app.set_pending_key('o');
   app.tick();
   assert_eq!(app.pending_key(), Some('o'), "A fresh pending key should survive a tick");
}
