//! UI rendering logic for the TUI.
//!
//! Implements the full layout matching the design:
//! - Header bar with filter badges and shell indicator
//! - Search bar with cursor support
//! - Main body: left list panel + right detail panels (description + script)
//! - Footer bar with help text

pub mod body;
mod components;
mod footer;
mod header;
mod help;
mod search;

use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Modifier, Style, Stylize};
use ratatui::widgets::Block;
use ratatui::Frame;

use crate::tui::app::App;
use crate::tui::state::{EntryFilter, InputMode};
use crate::tui::themes::Theme;

/// Get the border style for the current filter
pub(super) fn get_border_style(
   filter: &EntryFilter,
   theme: &Theme,
) -> Style {
   match filter {
      EntryFilter::Aliases => Style::default().fg(theme.alias_color).add_modifier(Modifier::BOLD),
      EntryFilter::Functions => Style::default().fg(theme.function_color).add_modifier(Modifier::BOLD),
      _ => Style::default().white().add_modifier(Modifier::BOLD),
   }
}

pub use body::draw_main_body;
pub use footer::draw_footer;
pub use header::draw_header;
pub use help::draw_help_modal;
pub use search::draw_search_bar;

/// Columns between the search bar\'s left edge and its text: one for the border and one for padding
const SEARCH_CURSOR_OFFSET: u16 = 2;

/// Draw the complete TUI interface
pub fn draw(
   frame: &mut Frame,
   app: &mut App,
) {
   let theme = app.theme().clone();

   let background = Block::default().style(Style::default().bg(theme.background));
   frame.render_widget(background, frame.area());

   let outer_chunks = Layout::default()
      .direction(Direction::Vertical)
      .constraints([Constraint::Length(1), Constraint::Length(3), Constraint::Min(0), Constraint::Length(1)])
      .split(frame.area());

   draw_header(frame, app, &theme, outer_chunks[0]);
   draw_search_bar(frame, app, outer_chunks[1]);
   draw_main_body(frame, app, &theme, outer_chunks[2]);
   draw_footer(frame, app, &theme, outer_chunks[3]);

   if app.is_help_visible() {
      draw_help_modal(frame, app, &theme);
   }

   if app.input_mode() == InputMode::Search && !app.is_help_visible() {
      frame.set_cursor_position((
         outer_chunks[1].x + SEARCH_CURSOR_OFFSET + app.cursor_position() as u16,
         outer_chunks[1].y + 1,
      ));
   }
}
