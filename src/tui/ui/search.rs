//! Search bar rendering.
//!
//! Draws the search input field with cursor support.

use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::Span;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::tui::app::App;
use crate::tui::state::InputMode;

use super::get_border_style;

pub fn draw_search_bar(frame: &mut Frame, app: &App, area: Rect) {
   let theme = app.theme();
   let (title, style) = match app.input_mode() {
      InputMode::Normal => (" Search (press 'i' or '/' to search)", Style::default()),
      InputMode::Search => (" Search", Style::default().add_modifier(Modifier::BOLD)),
   };

   let block = Block::default()
      .borders(Borders::ALL)
      .title(title)
      .border_style(if app.input_mode() == InputMode::Search {
         get_border_style(&app.filter(), theme)
      } else {
         Style::default()
      })
      .padding(ratatui::widgets::Padding::horizontal(1));

   let search_text = if app.search_query().is_empty() && app.input_mode() == InputMode::Normal {
      Paragraph::new(Span::styled("Enter your search term...", Style::default().add_modifier(Modifier::DIM)))
   } else {
      Paragraph::new(Span::raw(app.search_query()))
   };

   let search_widget = search_text.style(style).block(block);
   frame.render_widget(search_widget, area);
}
