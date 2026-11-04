//! Script panel rendering.
//!
//! Renders the right-bottom panel showing entry script/function body with syntax highlighting and scrollbar.

use ratatui::layout::Rect;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Wrap;
use ratatui::Frame;

use crate::tui::app::App;
use crate::tui::state::Panel;
use crate::tui::syntax;
use crate::tui::themes::Theme;

use super::super::components::{panel_block, render_scrollbar};

/// Draw the right-bottom panel: script/function body
pub fn draw_script_panel(
   frame: &mut Frame,
   app: &mut App,
   theme: &Theme,
   area: Rect,
) {
   let is_active = app.active_panel() == Panel::Script;
   let block = panel_block("[ Script ]", is_active, &app.filter(), theme);

   let script_text = match app.selected_entry() {
      Some(entry) => entry.value.clone(),
      None => "(No entry selected)".to_string(),
   };

   let highlighted_text = syntax::highlight_shell_script_with_style(&script_text, !is_active);

   let paragraph = Paragraph::new(highlighted_text).wrap(Wrap {
      trim: false,
   });
   let inner_area = block.inner(area);
   let total_lines = paragraph.line_count(inner_area.width);
   let visible_lines = inner_area.height as usize;

   app.update_script_max_scroll(total_lines, visible_lines);

   let paragraph = paragraph.block(block).scroll((app.script_scroll_offset() as u16, 0));

   frame.render_widget(paragraph, area);

   render_scrollbar(frame, inner_area, total_lines, visible_lines, app.script_scroll_offset(), theme.border);
}
