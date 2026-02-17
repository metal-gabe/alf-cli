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
pub fn draw_script_panel(frame: &mut Frame, app: &mut App, theme: &Theme, area: Rect) {
   let is_active = app.active_panel() == Panel::Script;
   let block = panel_block("[ Script ]", is_active, &app.filter(), theme);

   let script_text = match app.selected_entry() {
      Some(entry) => entry.value.clone(),
      None => "(No entry selected)".to_string(),
   };

   // Calculate content dimensions for scrollbar
   let total_lines = script_text.lines().count();
   let inner_area = block.inner(area);
   let visible_lines = inner_area.height as usize;

   // Update max scroll in app state
   app.update_script_max_scroll(total_lines, visible_lines);

   // Apply syntax highlighting with optional dimming
   let highlighted_text = syntax::highlight_shell_script_with_style(&script_text, !is_active);

   let paragraph = Paragraph::new(highlighted_text)
      .block(block)
      .wrap(Wrap { trim: false })
      .scroll((app.script_scroll_offset() as u16, 0));

   frame.render_widget(paragraph, area);

   // Render scrollbar
   render_scrollbar(frame, inner_area, total_lines, visible_lines, app.script_scroll_offset(), theme.border);
}
