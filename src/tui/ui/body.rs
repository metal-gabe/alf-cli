//! Main body rendering.
//!
//! Draws the main content area: left list panel and right detail panels (description + script).

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{
   Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
   Wrap,
};
use ratatui::Frame;

use crate::models::EntryType;
use crate::tui::app::{App, EntryFilter, Panel};
use crate::tui::syntax;

use super::get_border_style;

/// Draw the main body: left list panel + right detail panels
pub fn draw_main_body(frame: &mut Frame, app: &mut App, area: Rect) {
   // Horizontal split: left 40% list, right 60% detail
   let main_chunks = Layout::default()
      .direction(Direction::Horizontal)
      .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
      .split(area);

   draw_entry_list(frame, app, main_chunks[0]);
   draw_detail_panels(frame, app, main_chunks[1]);
}

/// Draw the left panel: list of aliases/functions
fn draw_entry_list(frame: &mut Frame, app: &App, area: Rect) {
   let is_active = app.active_panel == Panel::List;

   let block = Block::default()
      .borders(Borders::ALL)
      .title(Span::styled(
         "[ Entries ]",
         if is_active { Style::default().fg(Color::Rgb(220, 220, 220)) } else { Style::default() },
      ))
      .border_type(if is_active { BorderType::Double } else { BorderType::Plain })
      .border_style(if is_active { get_border_style(&app.filter) } else { Style::default() });

   // Check if we have no results after a search
   if app.visible_indices.is_empty() && !app.search_query.is_empty() {
      let no_results =
         Paragraph::new(Span::styled("(No results found)", Style::default().add_modifier(Modifier::DIM))).block(block);
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
   let header_right = "Source File";

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

   let header = Paragraph::new(header_line).style(if is_active {
      Style::default().add_modifier(Modifier::BOLD)
   } else {
      Style::default().add_modifier(Modifier::BOLD | Modifier::DIM)
   });

   frame.render_widget(header, chunks[0]);

   // Draw divider line (horizontal line of dashes)
   let divider_line = Line::from(Span::raw("─".repeat(content_width as usize + 2)));
   let divider = Paragraph::new(divider_line).style(if is_active {
      Style::default()
   } else {
      Style::default().add_modifier(Modifier::DIM)
   });
   frame.render_widget(divider, chunks[1]);

   let items: Vec<ListItem> = app
      .visible_indices
      .iter()
      .map(|&idx| {
         let entry = &app.entries[idx];
         let badge = match entry.entry_type {
            EntryType::Alias => Span::styled(
               "[&] ",
               if is_active {
                  Style::default().fg(Color::Rgb(253, 90, 30))
               } else {
                  Style::default().fg(Color::Rgb(253, 90, 30)).add_modifier(Modifier::DIM)
               },
            ),
            EntryType::Function => Span::styled(
               "[f] ",
               if is_active {
                  Style::default().fg(Color::Rgb(0, 199, 255))
               } else {
                  Style::default().fg(Color::Rgb(0, 199, 255)).add_modifier(Modifier::DIM)
               },
            ),
         };
         let name = Span::styled(
            &entry.name,
            if is_active { Style::default() } else { Style::default().add_modifier(Modifier::DIM) },
         );

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

         let source = Span::styled(
            formatted_source,
            if is_active { Style::default() } else { Style::default().add_modifier(Modifier::DIM) },
         );

         ListItem::new(Line::from(vec![badge, name, Span::raw(" ".repeat(padding_width)), source]))
      })
      .collect();

   let list = List::new(items)
      .highlight_style(Style::default().fg(Color::Rgb(220, 220, 220)).bg(Color::Rgb(23, 148, 129)).bold())
      .highlight_symbol("▸ ");

   let mut list_state = ListState::default();

   if !app.visible_indices.is_empty() {
      list_state.select(Some(app.selected_index));
   }

   frame.render_stateful_widget(list, chunks[2], &mut list_state);
}

/// Draw the right detail panels: description (top) + script (bottom)
fn draw_detail_panels(frame: &mut Frame, app: &mut App, area: Rect) {
   // Vertical split: top 30% description, bottom 70% script
   let detail_chunks = Layout::default()
      .direction(Direction::Vertical)
      .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
      .split(area);

   draw_description_panel(frame, app, detail_chunks[0]);
   draw_script_panel(frame, app, detail_chunks[1]);
}

/// Draw the right-top panel: comments/description
fn draw_description_panel(frame: &mut Frame, app: &mut App, area: Rect) {
   let is_active = app.active_panel == Panel::Description;

   let block = Block::default()
      .borders(Borders::ALL)
      .title(Span::styled(
         "[ Description ]",
         if is_active { Style::default().fg(Color::Rgb(220, 220, 220)) } else { Style::default() },
      ))
      .border_type(if is_active { BorderType::Double } else { BorderType::Plain })
      .border_style(if is_active { get_border_style(&app.filter) } else { Style::default() });

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

   let paragraph = Paragraph::new(Text::styled(
      description_text,
      if is_active { Style::default() } else { Style::default().add_modifier(Modifier::DIM) },
   ))
   .block(block)
   .wrap(Wrap { trim: false })
   .scroll((app.description_scroll_offset as u16, 0));

   frame.render_widget(paragraph, area);

   // Render scrollbar only if content extends beyond visible area
   if total_lines > visible_lines {
      let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
         .style(Style::default().fg(Color::Rgb(100, 100, 100)))
         .begin_symbol(Some("↑"))
         .end_symbol(Some("↓"));

      let mut scrollbar_state =
         ScrollbarState::new(total_lines.saturating_sub(visible_lines)).position(app.description_scroll_offset);

      // Render scrollbar inside the panel borders
      frame.render_stateful_widget(scrollbar, inner_area, &mut scrollbar_state);
   }
}

/// Draw the right-bottom panel: script/function body
fn draw_script_panel(frame: &mut Frame, app: &mut App, area: Rect) {
   let is_active = app.active_panel == Panel::Script;

   let block = Block::default()
      .borders(Borders::ALL)
      .title(Span::styled(
         "[ Script ]",
         if is_active { Style::default().fg(Color::Rgb(220, 220, 220)) } else { Style::default() },
      ))
      .border_type(if is_active { BorderType::Double } else { BorderType::Plain })
      .border_style(if is_active {
         match app.filter {
            EntryFilter::Aliases => Style::default().fg(Color::Rgb(253, 90, 30)).add_modifier(Modifier::BOLD),
            EntryFilter::Functions => Style::default().fg(Color::Rgb(0, 199, 255)).add_modifier(Modifier::BOLD),
            _ => Style::default().white().add_modifier(Modifier::BOLD),
         }
      } else {
         Style::default()
      });

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
      .scroll((app.script_scroll_offset as u16, 0));

   frame.render_widget(paragraph, area);

   // Render scrollbar only if content extends beyond visible area
   if total_lines > visible_lines {
      let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
         .style(Style::default().fg(Color::Rgb(100, 100, 100)))
         .begin_symbol(Some("↑"))
         .end_symbol(Some("↓"));

      let mut scrollbar_state =
         ScrollbarState::new(total_lines.saturating_sub(visible_lines)).position(app.script_scroll_offset);

      // Render scrollbar inside the panel borders
      frame.render_stateful_widget(scrollbar, inner_area, &mut scrollbar_state);
   }
}
