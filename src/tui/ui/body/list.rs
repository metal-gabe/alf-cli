//! Entry list panel rendering.
//!
//! Renders the left panel showing the list of aliases/functions with headers and badges.

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{List, ListItem, ListState};
use ratatui::Frame;

use crate::models::EntryType;
use crate::tui::app::App;
use crate::tui::state::Panel;
use crate::tui::themes::Theme;

use super::super::components::{active_style, panel_block};

/// Draw the left panel: list of aliases/functions
pub fn draw_entry_list(frame: &mut Frame, app: &App, theme: &Theme, area: Rect) {
   let is_active = app.active_panel() == Panel::List;
   let block = panel_block("[ Entries ]", is_active, &app.filter(), theme);

   // Check if we have no results after a search
   if app.visible_indices().is_empty() && !app.search_query().is_empty() {
      let no_results = ratatui::widgets::Paragraph::new(Span::styled(
         "(No results found)",
         Style::default().add_modifier(Modifier::DIM),
      ))
      .block(block);
      frame.render_widget(no_results, area);
      return;
   }

   // Split area into header (1 line), divider (1 line), and list (remaining)
   let inner_area = block.inner(area);
   let chunks = Layout::default()
      .direction(Direction::Vertical)
      .constraints([
         Constraint::Length(1), // Header row
         Constraint::Length(1), // Divider line
         Constraint::Min(0),    // List entries
      ])
      .split(inner_area);

   // Render the block border
   frame.render_widget(block, area);

   // Calculate available width for content (accounting for highlight symbol and padding)
   let content_width = inner_area.width.saturating_sub(2); // 2 for highlight symbol "▸ ", 2 for padding
   let badge_width = 4; // "[&] " or "[f] "
   let source_file_max_width = 15; // Reserve space for source file display

   // Draw header row: "Name" on left (aligned with entry names after badge), "File Source" on right
   let header_left_prefix = "  "; // 4 spaces to align with entry names after "[&] "
   let header_left = "Name";
   let header_right = "Source";

   // Calculate padding: total width - prefix - "Name" - "File Source"
   let header_content_width = header_left_prefix.len() + header_left.len() + header_right.len();
   let header_padding =
      if content_width as usize > header_content_width { content_width as usize - header_content_width } else { 1 };

   let header_line = Line::from(vec![
      Span::raw(header_left_prefix),
      Span::raw(header_left),
      Span::raw(" ".repeat(header_padding)),
      Span::raw(header_right),
   ]);

   let header = ratatui::widgets::Paragraph::new(header_line).style(active_style(is_active, true));

   frame.render_widget(header, chunks[0]);

   // Draw divider line (horizontal line of dashes)
   let divider_line = Line::from(Span::raw("─".repeat(content_width as usize + 2)));
   let divider = ratatui::widgets::Paragraph::new(divider_line).style(active_style(is_active, false));
   frame.render_widget(divider, chunks[1]);

   let items: Vec<ListItem> = app
      .visible_indices()
      .iter()
      .map(|&idx| {
         let entry = &app.entries()[idx];
         let badge = match entry.entry_type {
            EntryType::Alias => Span::styled(
               "[@] ",
               if is_active {
                  Style::default().fg(theme.alias_color)
               } else {
                  Style::default().fg(theme.alias_color).add_modifier(Modifier::DIM)
               },
            ),
            EntryType::Function => Span::styled(
               "[ƒ] ",
               if is_active {
                  Style::default().fg(theme.function_color)
               } else {
                  Style::default().fg(theme.function_color).add_modifier(Modifier::DIM)
               },
            ),
         };
         let name = Span::styled(&entry.name, active_style(is_active, false));

         // Format source file: extract filename and truncate/pad to fit
         let source_filename = entry.source_file.file_name().and_then(|n| n.to_str()).unwrap_or("unknown");

         // Truncate or pad the source filename to fit within source_file_max_width
         // Right-align by padding on the left
         let formatted_source = if source_filename.len() > source_file_max_width {
            // Truncate from the left with ellipsis: "…long_filename"
            format!("…{}", &source_filename[source_filename.len() - source_file_max_width + 1..])
         } else {
            format!("{:>width$}  ", source_filename, width = source_file_max_width)
         };

         // Calculate padding between name and source file
         let text_width = badge_width + entry.name.len() + formatted_source.len();
         let padding_width = if content_width as usize > text_width { content_width as usize - text_width } else { 1 };

         let source = Span::styled(formatted_source, active_style(is_active, false));

         ListItem::new(Line::from(vec![badge, name, Span::raw(" ".repeat(padding_width)), source]))
      })
      .collect();

   let list = List::new(items)
      .highlight_style(Style::default().fg(theme.foreground).bg(theme.selection).bold())
      .highlight_symbol("▸ ");

   let mut list_state = ListState::default();

   if !app.visible_indices().is_empty() {
      list_state.select(Some(app.selected_index()));
   }

   frame.render_stateful_widget(list, chunks[2], &mut list_state);
}
