use super::*;

fn make_app_with_comment_on_first_entry() -> App {
   let mut commented = make_entry("alpha", EntryType::Alias, "cmd_alpha");
   commented.comments = Some(vec!["Says alpha".to_string()]);
   App::new(vec![commented, make_entry("beta", EntryType::Alias, "cmd_beta")], Theme::default())
}

#[test]
fn parse_comments_defaults_to_always() {
   let app = make_app();
   assert_eq!(app.parse_comments(), ParseComments::Always);
}

#[test]
fn cycle_parse_comments_follows_dynamic_true_false_order() {
   let mut app = make_app();
   app.set_parse_comments(ParseComments::Dynamic);
   for expected in [ParseComments::Always, ParseComments::Never, ParseComments::Dynamic] {
      app.cycle_parse_comments();
      assert_eq!(app.parse_comments(), expected);
   }
}

#[test]
fn description_visibility_follows_setting_and_selected_entry_comments() {
   let cases = [
      (ParseComments::Always, [true, true]),
      (ParseComments::Dynamic, [true, false]),
      (ParseComments::Never, [false, false]),
   ];
   for (setting, expected) in cases {
      let mut app = make_app_with_comment_on_first_entry();
      app.set_parse_comments(setting);
      let is_visible_with_comment = app.is_description_visible();
      app.move_down();
      let is_visible_without_comment = app.is_description_visible();
      assert_eq!([is_visible_with_comment, is_visible_without_comment], expected, "{setting:?}");
   }
}

#[test]
fn panel_cycling_skips_hidden_description() {
   let mut app = make_app();
   app.set_parse_comments(ParseComments::Never);
   app.cycle_panel();
   assert_eq!(app.active_panel(), Panel::Script);
   app.cycle_panel_backward();
   assert_eq!(app.active_panel(), Panel::List);
   app.cycle_panel_backward();
   assert_eq!(app.active_panel(), Panel::Script);
}

#[test]
fn sync_active_panel_moves_focus_from_hidden_description_to_script() {
   let mut app = make_app();
   app.cycle_panel();
   assert_eq!(app.active_panel(), Panel::Description);
   app.cycle_parse_comments();
   app.sync_active_panel();
   assert_eq!(app.active_panel(), Panel::Script);
}
