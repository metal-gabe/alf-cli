use super::*;

#[test]
fn test_set_and_clear_pending_key() {
   let mut app = make_app();
   app.set_pending_key('g');
   assert_eq!(app.pending_key(), Some('g'));
   app.clear_pending_key();
   assert!(app.pending_key().is_none());
}
