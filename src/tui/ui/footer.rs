//! Footer bar rendering.
//!
//! Draws the bottom bar with help text and status indicators.

use ratatui::layout::Rect;
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::tui::app::App;
use crate::tui::state::{GroupMode, SortOrder};

/// Draw the footer bar with help text
pub fn draw_footer(frame: &mut Frame, app: &App, area: Rect) {
   // Format group mode
   let group_text = match app.group_mode() {
      GroupMode::None => "none",
      GroupMode::Aliases => "aliases",
      GroupMode::Functions => "functions",
   };

   // Format sort order
   let sort_text = match app.sort_order() {
      SortOrder::Ascending => "asc",
      SortOrder::Descending => "desc",
   };

   // Show pending key indicator or default help text
   let right_text = if let Some(pending) = app.pending_key() {
      match pending {
         'g' => "Waiting: g (go to top) ",
         'o' => "Waiting: g (group), G (back), s (sort) ",
         _ => "Press \"?\" for Help ", // Fallback for unknown pending keys
      }
   } else {
      "Press \"?\" for Help "
   };

   // Build left side with bold labels
   let left_spans = vec![
      Span::styled("GROUP: ", Style::default().bold()),
      Span::raw(group_text),
      Span::raw(" | "),
      Span::styled("SORT: ", Style::default().bold()),
      Span::raw(sort_text),
   ];

   // Calculate padding between left and right text
   let left_width: usize = left_spans.iter().map(|s| s.width()).sum();
   let total_text_width = left_width + right_text.len();
   let padding = if area.width as usize > total_text_width { area.width as usize - total_text_width } else { 1 };

   let mut footer_spans = left_spans;
   footer_spans.push(Span::raw(" ".repeat(padding)));
   footer_spans.push(Span::raw(right_text));

   let footer_line = Line::from(footer_spans);

   let footer = Paragraph::new(footer_line);
   frame.render_widget(footer, area);
}
