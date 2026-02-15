//! UI rendering logic for the TUI.
//!
//! Implements the full layout matching the design:
//! - Header bar with filter badges and shell indicator
//! - Search bar with cursor support
//! - Main body: left list panel + right detail panels (description + script)
//! - Footer bar with help text

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, BorderType, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap};
use ratatui::Frame;

use crate::models::EntryType;
use crate::tui::app::{App, EntryFilter, GroupMode, InputMode, Panel, SortOrder};
use crate::tui::syntax;

/// Draw the complete TUI interface
pub fn draw(frame: &mut Frame, app: &App) {
   // Apply global background color to entire TUI
   let background = Block::default().style(Style::default().bg(Color::Rgb(17, 17, 17)));
   frame.render_widget(background, frame.area());

   // Top-level vertical layout
   let outer_chunks = Layout::default()
      .direction(Direction::Vertical)
      .constraints([
         Constraint::Length(1), // Header bar
         Constraint::Length(3), // Search bar
         Constraint::Min(0),    // Main content body
         Constraint::Length(1), // Footer bar
      ])
      .split(frame.area());

   draw_header(frame, app, outer_chunks[0]);
   draw_search_bar(frame, app, outer_chunks[1]);
   draw_main_body(frame, app, outer_chunks[2]);
   draw_footer(frame, app, outer_chunks[3]);

   // Draw help modal overlay if active (must be last to overlay everything)
   if app.show_help {
      draw_help_modal(frame);
   }

   // Place cursor in search bar when in search mode (and help is not showing)
   if app.input_mode == InputMode::Search && !app.show_help {
      // Cursor position: inside the search block (1 char border + cursor_position)
      frame.set_cursor_position((outer_chunks[1].x + 1 + app.cursor_position as u16, outer_chunks[1].y + 1));
   }
}

/// Draw the header bar with filter badges and shell indicator
fn draw_header(frame: &mut Frame, app: &App, area: Rect) {
   let filter_color: Color = match app.filter {
      EntryFilter::Aliases => Color::Rgb(253, 90, 30),
      EntryFilter::All => Color::Rgb(220, 220, 220),
      EntryFilter::Functions => Color::Rgb(0, 199, 255),
   };

   let badge_style = Style::default().fg(Color::Rgb(17, 17, 17)).bg(filter_color).bold();

   // Build the left side: filter badges
   let badges = vec![
      Span::raw("FILTERS: "),
      Span::styled(" & ", if matches!(app.filter, EntryFilter::Aliases) { badge_style } else { Style::default() }),
      Span::raw(" "),
      Span::styled(" f ", if matches!(app.filter, EntryFilter::Functions) { badge_style } else { Style::default() }),
      Span::raw(" "),
      Span::styled(" * ", if matches!(app.filter, EntryFilter::All) { badge_style } else { Style::default() }),
   ];

   // Build the right side: shell indicator
   let shell_label_prefix = " $SHELL: ";
   let shell_name = "zsh";
   let shell_label_suffix = " ";

   // Calculate padding to right-align the shell label
   let badges_width: usize = badges.iter().map(|s| s.width()).sum();
   let shell_width = shell_label_prefix.len() + shell_name.len() + shell_label_suffix.len();

   let padding = if area.width as usize > badges_width + shell_width {
      area.width as usize - badges_width - shell_width
   } else {
      1
   };

   let mut spans = badges;
   spans.push(Span::raw(" ".repeat(padding)));
   spans.push(Span::raw(shell_label_prefix));
   spans.push(Span::styled(shell_name, Style::default().fg(filter_color)));
   spans.push(Span::raw(shell_label_suffix));

   let header = Paragraph::new(Line::from(spans));
   frame.render_widget(header, area);
}

fn draw_search_bar(frame: &mut Frame, app: &App, area: Rect) {
   let (title, style) = match app.input_mode {
      InputMode::Normal => (" Search (press / to search) ", Style::default()),
      InputMode::Search => (" Search ", Style::default().add_modifier(Modifier::BOLD)),
   };

   let block = Block::default()
      .borders(Borders::ALL)
      .title(title)
      .border_style(if app.input_mode == InputMode::Search { get_border_style(&app.filter) } else { Style::default() });

   let search_text = if app.search_query.is_empty() && app.input_mode == InputMode::Normal {
      Paragraph::new(Span::styled(" Enter your search term...", Style::default().add_modifier(Modifier::DIM)))
   } else {
      Paragraph::new(Span::raw(&app.search_query))
   };

   let search_widget = search_text.style(style).block(block);
   frame.render_widget(search_widget, area);
}

/// Draw the main body: left list panel + right detail panels
fn draw_main_body(frame: &mut Frame, app: &App, area: Rect) {
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
   let source_file = ".zshrc  ";
   let badge_width = 4; // "[&] " or "[f] "

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

         // Calculate padding between name and source file
         let text_width = badge_width + entry.name.len() + source_file.len();
         let padding_width = if content_width as usize > text_width { content_width as usize - text_width } else { 1 };

         let source = Span::styled(
            source_file,
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
fn draw_detail_panels(frame: &mut Frame, app: &App, area: Rect) {
   // Vertical split: top 30% description, bottom 70% script
   let detail_chunks = Layout::default()
      .direction(Direction::Vertical)
      .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
      .split(area);

   draw_description_panel(frame, app, detail_chunks[0]);
   draw_script_panel(frame, app, detail_chunks[1]);
}

/// Draw the right-top panel: comments/description
fn draw_description_panel(frame: &mut Frame, app: &App, area: Rect) {
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

   let paragraph = Paragraph::new(Text::styled(
      description_text,
      if is_active { Style::default() } else { Style::default().add_modifier(Modifier::DIM) },
   ))
   .block(block)
   .wrap(Wrap { trim: false })
   .scroll((app.description_scroll_offset as u16, 0));

   frame.render_widget(paragraph, area);
}

/// Draw the right-bottom panel: script/function body
fn draw_script_panel(frame: &mut Frame, app: &App, area: Rect) {
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

   // Apply syntax highlighting with optional dimming
   let highlighted_text = syntax::highlight_shell_script_with_style(&script_text, !is_active);

   let paragraph = Paragraph::new(highlighted_text)
      .block(block)
      .wrap(Wrap { trim: false })
      .scroll((app.script_scroll_offset as u16, 0));

   frame.render_widget(paragraph, area);
}

/// Draw the footer bar with help text
fn draw_footer(frame: &mut Frame, app: &App, area: Rect) {
   // Format group mode
   let group_text = match app.group_mode {
      GroupMode::None => "none",
      GroupMode::Aliases => "aliases",
      GroupMode::Functions => "functions",
   };

   // Format sort order
   let sort_text = match app.sort_order {
      SortOrder::Ascending => "asc",
      SortOrder::Descending => "desc",
   };

   let left_text = format!("GROUP: {} | SORT: {}", group_text, sort_text);

   // Show pending key indicator or default help text
   let right_text = if let Some(pending) = app.pending_key {
      match pending {
         'g' => "Waiting: g (go to top) ",
         'o' => "Waiting: g (group), G (back), s (sort) ",
         _ => "Press \"?\" for Help ", // Fallback for unknown pending keys
      }
   } else {
      "Press \"?\" for Help "
   };

   // Calculate padding between left and right text
   let total_text_width = left_text.len() + right_text.len();
   let padding = if area.width as usize > total_text_width { area.width as usize - total_text_width } else { 1 };

   let footer_line = Line::from(vec![Span::raw(left_text), Span::raw(" ".repeat(padding)), Span::raw(right_text)]);

   let footer = Paragraph::new(footer_line);
   frame.render_widget(footer, area);
}

fn get_border_style(filter: &EntryFilter) -> Style {
   match filter {
      EntryFilter::Aliases => Style::default().fg(Color::Rgb(253, 90, 30)).add_modifier(Modifier::BOLD),
      EntryFilter::Functions => Style::default().fg(Color::Rgb(0, 199, 255)).add_modifier(Modifier::BOLD),
      _ => Style::default().white().add_modifier(Modifier::BOLD),
   }
}

/// Draw the help modal overlay (75% width, 80% height, centered)
fn draw_help_modal(frame: &mut Frame) {
   let area = frame.area();

   // Calculate modal dimensions: 75% width, 80% height
   let modal_width = (area.width as f32 * 0.70) as u16;
   let modal_height = (area.height as f32 * 0.90) as u16;

   // Center the modal
   let modal_x = (area.width.saturating_sub(modal_width)) / 2;
   let modal_y = (area.height.saturating_sub(modal_height)) / 2;

   let modal_area = Rect { x: modal_x, y: modal_y, width: modal_width, height: modal_height };

   // Clear the entire screen area first (this removes all underlying content)
   frame.render_widget(Clear, area);

   // Draw full-screen dark background
   let full_bg = Block::default().style(Style::default().bg(Color::Rgb(17, 17, 17)));
   frame.render_widget(full_bg, area);

   // Clear the modal area to ensure clean rendering
   frame.render_widget(Clear, modal_area);

   // Draw modal block with border and background
   let modal_block = Block::default()
      .borders(Borders::ALL)
      .border_type(BorderType::Double)
      .border_style(Style::default().fg(Color::Rgb(220, 220, 220)).bold())
      .title(Span::styled(" Help ", Style::default().fg(Color::Rgb(220, 220, 220)).bold()))
      .style(Style::default().bg(Color::Rgb(17, 17, 17)));

   // Content with padding consistent with the app
   let help_text = "Welcome to Getting Help!";
   let content =
      Paragraph::new(help_text).block(modal_block).style(Style::default().fg(Color::White).bg(Color::Rgb(17, 17, 17)));

   frame.render_widget(content, modal_area);
}
