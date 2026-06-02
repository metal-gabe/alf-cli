use super::*;

#[test]
fn test_select_entry_execute_sets_quit_and_action() {
   let mut app = make_app();
   app.select_entry(ExitAction::Execute);
   assert!(app.should_quit);
   assert!(matches!(app.exit_action, Some(ExitAction::Execute)));
}

#[test]
fn test_select_entry_populate_sets_quit_and_action() {
   let mut app = make_app();
   app.select_entry(ExitAction::Populate);
   assert!(app.should_quit);
   assert!(matches!(app.exit_action, Some(ExitAction::Populate)));
}

#[test]
fn test_select_entry_on_empty_app_sets_quit_but_no_action() {
   let mut app = make_empty_app();
   app.select_entry(ExitAction::Execute);
   assert!(app.should_quit, "should_quit should still be set");
   assert!(app.exit_action.is_none(), "exit_action should be None when no entry selected");
}
