//! Description panel rendering.
//!
//! Renders the right-top panel showing entry comments/description with scrollbar support.

use ratatui::layout::Rect;
use ratatui::text::Text;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Wrap;
use ratatui::Frame;

use crate::tui::app::App;
use crate::tui::state::Panel;
use crate::tui::themes::Theme;

use super::super::components::{active_style, panel_block, render_scrollbar};

/// Draw the right-top panel: comments/description
pub fn draw_description_panel(
   frame: &mut Frame,
   app: &mut App,
   theme: &Theme,
   area: Rect,
) {
   let is_active = app.active_panel() == Panel::Description;
   let block = panel_block("[ Description ]", is_active, &app.filter(), theme);

   let description_text = match app.selected_entry() {
      Some(entry) => match &entry.comments {
         Some(comments) => comments.join("\n"),
         None => "(No description available)".to_string(),
      },
      None => "(No entry selected)".to_string(),
   };

   // Calculate content dimensions for scrollbar
   let total_lines = description_text.lines().count();
   let inner_area = block.inner(area);
   let visible_lines = inner_area.height as usize;

   // Update max scroll in app state
   app.update_description_max_scroll(total_lines, visible_lines);

   let paragraph = Paragraph::new(Text::styled(description_text, active_style(is_active, false)))
      .block(block)
      .wrap(Wrap {
         trim: false,
      })
      .scroll((app.description_scroll_offset() as u16, 0));

   frame.render_widget(paragraph, area);

   // Render scrollbar
   render_scrollbar(frame, inner_area, total_lines, visible_lines, app.description_scroll_offset(), theme.border);
}
