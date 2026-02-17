//! Footer bar rendering.
//!
//! Draws the bottom bar with help text, mode indicator, and status indicators.

use ratatui::layout::Rect;
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::tui::app::App;
use crate::tui::state::{GroupMode, SortOrder};
use crate::tui::themes::Theme;

/// Draw the footer bar with help text
pub fn draw_footer(frame: &mut Frame, app: &App, theme: &Theme, area: Rect) {
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
         't' => "Waiting: j (next theme), k (prev theme) ",
         _ => "Press \"?\" for Help ", // Fallback for unknown pending keys
      }
   } else {
      "Press \"?\" for Help "
   };

   // Build the middle: theme display
   let theme_label = "THEME: ";
   let theme_display_name = theme.display_name.as_str();
   let theme_center_width = theme_label.len() + theme_display_name.len();

   // Build left side with bold labels
   let left_spans = vec![
      Span::styled("GROUP: ", Style::default().bold()),
      Span::raw(group_text),
      Span::raw(" | "),
      Span::styled("SORT: ", Style::default().bold()),
      Span::raw(sort_text),
   ];

   // Calculate widths for centering theme between left and right
   let left_width: usize = left_spans.iter().map(|s| s.width()).sum();
   let right_width = right_text.len();
   let total_width = area.width as usize;

   let left_padding = if total_width > left_width + theme_center_width + right_width {
      (total_width - left_width - theme_center_width - right_width) / 2
   } else {
      1
   };
   let right_padding = if total_width > left_width + left_padding + theme_center_width + right_width {
      total_width - left_width - left_padding - theme_center_width - right_width
   } else {
      1
   };

   let mut footer_spans = left_spans;
   footer_spans.push(Span::raw(" ".repeat(left_padding)));
   footer_spans.push(Span::styled(theme_label, Style::default().bold()));
   footer_spans.push(Span::raw(theme_display_name));
   footer_spans.push(Span::raw(" ".repeat(right_padding)));
   footer_spans.push(Span::raw(right_text));

   let footer_line = Line::from(footer_spans);

   let footer = Paragraph::new(footer_line);
   frame.render_widget(footer, area);
}
