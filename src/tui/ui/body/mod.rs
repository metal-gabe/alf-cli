//! Main body layout orchestration.
//!
//! Orchestrates the layout of the main content area:
//! left list panel (40%) and right detail panels (60%),
//! or the active panel alone when fullscreen is toggled on.
//! The script panel fills the detail area whenever the description panel is hidden.

#[cfg(test)]
mod body_tests;
mod description;
mod list;
mod script;

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::Frame;

use crate::tui::app::App;
use crate::tui::state::Panel;
use crate::tui::themes::Theme;

/// Draw the main body: left list panel + right detail panels
pub fn draw_main_body(
   frame: &mut Frame,
   app: &mut App,
   theme: &Theme,
   area: Rect,
) {
   app.sync_active_panel();

   if app.is_fullscreen() {
      match app.active_panel() {
         Panel::List => list::draw_entry_list(frame, app, theme, area),
         Panel::Description => description::draw_description_panel(frame, app, theme, area),
         Panel::Script => script::draw_script_panel(frame, app, theme, area),
      }
      return;
   }

   let main_chunks = Layout::default()
      .direction(Direction::Horizontal)
      .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
      .split(area);

   list::draw_entry_list(frame, app, theme, main_chunks[0]);
   draw_detail_panels(frame, app, theme, main_chunks[1]);
}

/// Draw the right detail panels: description (top) + script (bottom), or the script alone when the description is hidden
fn draw_detail_panels(
   frame: &mut Frame,
   app: &mut App,
   theme: &Theme,
   area: Rect,
) {
   if !app.is_description_visible() {
      script::draw_script_panel(frame, app, theme, area);
      return;
   }

   let detail_chunks = Layout::default()
      .direction(Direction::Vertical)
      .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
      .split(area);

   description::draw_description_panel(frame, app, theme, detail_chunks[0]);
   script::draw_script_panel(frame, app, theme, detail_chunks[1]);
}
