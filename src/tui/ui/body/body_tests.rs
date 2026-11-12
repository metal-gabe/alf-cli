//! Tests for the main body layout

use ratatui::backend::TestBackend;
use ratatui::Terminal;

use super::draw_main_body;
use crate::config::ParseComments;
use crate::tui::app::app_tests::make_app;
use crate::tui::app::App;
use crate::tui::state::Panel;

const PANEL_TITLES: [(Panel, &str); 3] =
   [(Panel::List, "[ Entries"), (Panel::Description, "[ Description ]"), (Panel::Script, "[ Script ]")];

fn render_body(app: &mut App) -> String {
   let mut terminal = Terminal::new(TestBackend::new(100, 30)).unwrap();
   let theme = app.theme().clone();
   terminal
      .draw(|frame| {
         let area = frame.area();
         draw_main_body(frame, app, &theme, area);
      })
      .unwrap();
   terminal.backend().buffer().content().iter().map(|cell| cell.symbol()).collect()
}

#[test]
fn split_layout_renders_all_panels() {
   let mut app = make_app();
   let rendered = render_body(&mut app);
   for (panel, title) in PANEL_TITLES {
      assert!(rendered.contains(title), "{panel:?} title missing from split layout");
   }
}

#[test]
fn hidden_description_leaves_list_and_script_panels() {
   for setting in [ParseComments::Dynamic, ParseComments::Never] {
      let mut app = make_app();
      app.set_parse_comments(setting);
      let rendered = render_body(&mut app);
      for (panel, title) in PANEL_TITLES {
         let is_title_rendered = rendered.contains(title);
         assert_eq!(is_title_rendered, panel != Panel::Description, "{setting:?}: {panel:?} title");
      }
   }
}

#[test]
fn fullscreen_renders_only_active_panel() {
   for (active_panel, _) in PANEL_TITLES {
      let mut app = make_app();
      while app.active_panel() != active_panel {
         app.cycle_panel();
      }
      app.toggle_fullscreen();
      let rendered = render_body(&mut app);
      for (panel, title) in PANEL_TITLES {
         let is_title_rendered = rendered.contains(title);
         assert_eq!(is_title_rendered, panel == active_panel, "Fullscreen {active_panel:?}: {panel:?} title");
      }
   }
}
