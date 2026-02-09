//! UI rendering logic for the TUI.
//!
//! Implements the full layout matching the design:
//! - Header bar with filter badges and shell indicator
//! - Search bar with cursor support
//! - Main body: left list panel + right detail panels (description + script)
//! - Footer bar with help text

use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Wrap};
use ratatui::Frame;

use crate::models::EntryType;
use crate::tui::app::{App, EntryFilter, InputMode, Panel};

/// Draw the complete TUI interface
pub fn draw(frame: &mut Frame, app: &App) {
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
   draw_footer(frame, outer_chunks[3]);

   // Place cursor in search bar when in search mode
   if app.input_mode == InputMode::Search {
      // Cursor position: inside the search block (1 char border + cursor_position)
      frame.set_cursor_position((outer_chunks[1].x + 1 + app.cursor_position as u16, outer_chunks[1].y + 1));
   }
}

/// Draw the header bar with filter badges and shell indicator
fn draw_header(frame: &mut Frame, app: &App, area: Rect) {
   let alias_style = if matches!(app.filter, EntryFilter::All | EntryFilter::Aliases) {
      Style::default().add_modifier(Modifier::BOLD | Modifier::REVERSED)
   } else {
      Style::default().add_modifier(Modifier::DIM)
   };

   let function_style = if matches!(app.filter, EntryFilter::All | EntryFilter::Functions) {
      Style::default().add_modifier(Modifier::BOLD | Modifier::REVERSED)
   } else {
      Style::default().add_modifier(Modifier::DIM)
   };

   let all_style = if matches!(app.filter, EntryFilter::All) {
      Style::default().add_modifier(Modifier::BOLD | Modifier::REVERSED)
   } else {
      Style::default().add_modifier(Modifier::DIM)
   };

   // Build the left side: filter badges
   let badges = vec![
      Span::raw(" "),
      Span::styled(" & ", alias_style),
      Span::raw(" "),
      Span::styled(" f ", function_style),
      Span::raw(" "),
      Span::styled(" * ", all_style),
   ];

   // Build the right side: shell indicator
   let shell_label = " $SHELL: zsh ";

   // Calculate padding to right-align the shell label
   let badges_width: usize = badges.iter().map(|s| s.width()).sum();
   let shell_width = shell_label.len();
   let padding = if area.width as usize > badges_width + shell_width {
      area.width as usize - badges_width - shell_width
   } else {
      1
   };

   let mut spans = badges;
   spans.push(Span::raw(" ".repeat(padding)));
   spans.push(Span::styled(shell_label, Style::default().add_modifier(Modifier::DIM)));

   let header = Paragraph::new(Line::from(spans));
   frame.render_widget(header, area);
}

/// Draw the search bar
fn draw_search_bar(frame: &mut Frame, app: &App, area: Rect) {
   let (title, style) = match app.input_mode {
      InputMode::Normal => (" Search (press / to search) ", Style::default()),
      InputMode::Search => (" Search ", Style::default().add_modifier(Modifier::BOLD)),
   };

   let block =
      Block::default().borders(Borders::ALL).title(title).border_style(if app.input_mode == InputMode::Search {
         Style::default().add_modifier(Modifier::BOLD)
      } else {
         Style::default()
      });

   let search_text = if app.search_query.is_empty() && app.input_mode == InputMode::Normal {
      Paragraph::new(Span::styled("Type / to search...", Style::default().add_modifier(Modifier::DIM)))
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
      .title(" Entries ")
      .border_type(if is_active { BorderType::Double } else { BorderType::Plain })
      .border_style(if is_active { Style::default().add_modifier(Modifier::BOLD) } else { Style::default() });

   let items: Vec<ListItem> = app
      .visible_indices
      .iter()
      .map(|&idx| {
         let entry = &app.entries[idx];
         let badge = match entry.entry_type {
            EntryType::Alias => Span::styled("[&] ", Style::default().add_modifier(Modifier::BOLD)),
            EntryType::Function => Span::styled("[f] ", Style::default().add_modifier(Modifier::BOLD)),
         };
         let name = Span::raw(&entry.name);
         ListItem::new(Line::from(vec![badge, name]))
      })
      .collect();

   let list = List::new(items)
      .block(block)
      .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
      .highlight_symbol("▸ ");

   let mut list_state = ListState::default();
   if !app.visible_indices.is_empty() {
      list_state.select(Some(app.selected_index));
   }

   frame.render_stateful_widget(list, area, &mut list_state);
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
      .title(" Description ")
      .border_type(if is_active { BorderType::Double } else { BorderType::Plain })
      .border_style(if is_active { Style::default().add_modifier(Modifier::BOLD) } else { Style::default() });

   let description_text = match app.selected_entry() {
      Some(entry) => match &entry.comments {
         Some(comments) => comments.join("\n"),
         None => "(No description available)".to_string(),
      },
      None => "(No entry selected)".to_string(),
   };

   let paragraph = Paragraph::new(description_text)
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
      .title(" Script ")
      .border_type(if is_active { BorderType::Double } else { BorderType::Plain })
      .border_style(if is_active { Style::default().add_modifier(Modifier::BOLD) } else { Style::default() });

   let script_text = match app.selected_entry() {
      Some(entry) => entry.value.clone(),
      None => "(No entry selected)".to_string(),
   };

   let paragraph =
      Paragraph::new(script_text).block(block).wrap(Wrap { trim: false }).scroll((app.script_scroll_offset as u16, 0));

   frame.render_widget(paragraph, area);
}

/// Draw the footer bar with help text
fn draw_footer(frame: &mut Frame, area: Rect) {
   let footer = Paragraph::new(Span::styled("Press \"?\" for Help ", Style::default().add_modifier(Modifier::DIM)))
      .alignment(Alignment::Right);

   frame.render_widget(footer, area);
}
