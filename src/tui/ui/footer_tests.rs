//! Tests for the footer bar

use ratatui::backend::TestBackend;
use ratatui::Terminal;

use super::draw_footer;
use crate::config::ParseComments;
use crate::tui::app::app_tests::make_app;
use crate::tui::app::App;

fn render_footer(app: &App) -> String {
   let mut terminal = Terminal::new(TestBackend::new(120, 1)).unwrap();
   let theme = app.theme().clone();
   terminal
      .draw(|frame| {
         let area = frame.area();
         draw_footer(frame, app, &theme, area);
      })
      .unwrap();
   terminal.backend().buffer().content().iter().map(|cell| cell.symbol()).collect()
}

#[test]
fn footer_shows_group_sort_and_comments_status() {
   let cases = [(ParseComments::Always, "true"), (ParseComments::Dynamic, "dynamic"), (ParseComments::Never, "false")];
   for (setting, label) in cases {
      let mut app = make_app();
      app.set_parse_comments(setting);
      let rendered = render_footer(&app);
      let expected = format!("G: aliases | S: asc | C: {label}");
      assert!(rendered.starts_with(&expected), "{setting:?}: expected `{expected}` in `{rendered}`");
   }
}
