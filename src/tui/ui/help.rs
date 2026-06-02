//! Help modal rendering.
//!
//! Draws the help screen overlay with keybinding documentation.

use ratatui::layout::Rect;
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap};
use ratatui::Frame;

use super::components::render_scrollbar;
use crate::tui::app::App;
use crate::tui::themes::Theme;

/// Draw the help modal overlay (70% width, 90% height, centered)
pub fn draw_help_modal(
   frame: &mut Frame,
   app: &mut App,
   theme: &Theme,
) {
   let area = frame.area();

   // Calculate modal dimensions: 70% width, 90% height
   let modal_width = (area.width as f32 * 0.70) as u16;
   let modal_height = (area.height as f32 * 0.90) as u16;

   // Center the modal
   let modal_x = (area.width.saturating_sub(modal_width)) / 2;
   let modal_y = (area.height.saturating_sub(modal_height)) / 2;

   let modal_area = Rect {
      x: modal_x,
      y: modal_y,
      width: modal_width,
      height: modal_height,
   };

   // Clear the entire screen area first (this removes all underlying content)
   frame.render_widget(Clear, area);

   // Draw full-screen dark background
   let full_bg = Block::default().style(Style::default().bg(theme.background));
   frame.render_widget(full_bg, area);

   // Clear the modal area to ensure clean rendering
   frame.render_widget(Clear, modal_area);

   // Draw modal block with border and background
   let modal_block = Block::default()
      .borders(Borders::ALL)
      .border_type(BorderType::Double)
      .border_style(Style::default().fg(theme.foreground).bold())
      .title(Span::styled(" Help ('?', 'q' or 'esc' to close)", Style::default().fg(theme.foreground).bold()))
      .style(Style::default().bg(theme.background))
      .padding(ratatui::widgets::Padding::horizontal(2));

   // Content with padding consistent with the app
   let help_text = vec![
      Line::from(""),
      Line::from(vec![Span::styled(
         "ALF - Alias & Function Search Tool",
         Style::default().bold().fg(theme.function_color),
      )]),
      Line::from("  Read the docs @ https://example.com"),
      Line::from(""),
      Line::from(vec![Span::styled("NAVIGATION", Style::default().bold().fg(theme.alias_color))]),
      Line::from("  j / ↓          Scroll down 1 line in active panel"),
      Line::from("  k / ↑          Scroll up 1 line in active panel"),
      Line::from("  g              Jump to top of list"),
      Line::from("  shift-g        Jump to bottom of list"),
      Line::from("  ctrl-f         Scroll down full page (20 lines)"),
      Line::from("  ctrl-b         Scroll up full page (20 lines)"),
      Line::from("  ctrl-j         Scroll down half page (10 lines)"),
      Line::from("  ctrl-k         Scroll up half page (10 lines)"),
      Line::from(""),
      Line::from(vec![Span::styled("PANELS & FILTERS", Style::default().bold().fg(theme.alias_color))]),
      Line::from("  n              Cycle panel focus forward (List → Description → Script)"),
      Line::from("  p              Cycle panel focus backward"),
      Line::from("  h              Cycle filter backward (All ← Functions ← Aliases)"),
      Line::from("  l              Cycle filter forward (All → Aliases → Functions)"),
      Line::from("  1              Select 'Aliases' filter"),
      Line::from("  2              Select 'Functions' filter"),
      Line::from("  3              Select 'All' filter"),
      Line::from(""),
      Line::from(vec![Span::styled("GROUPING & SORTING", Style::default().bold().fg(theme.alias_color))]),
      Line::from("  og / ctrl-g    Cycle group mode forward (None → Aliases → Functions)"),
      Line::from("  o shift-g      Cycle group mode backward"),
      Line::from("  os / ctrl-s    Toggle sort order (Ascending ↔ Descending)"),
      Line::from(""),
      Line::from(vec![Span::styled("SEARCH", Style::default().bold().fg(theme.alias_color))]),
      Line::from("  / or i         Enter search mode"),
      Line::from("  esc            Exit search mode (keep query)"),
      Line::from("  ctrl-u         Clear search query (any mode)"),
      Line::from("  shift-n        Cycle panels while in search mode"),
      Line::from("  shift-p        Cycle panels backward while in search mode"),
      Line::from("  shift-h        Cycle filters backward while in search mode"),
      Line::from("  shift-l        Cycle filters forward while in search mode"),
      Line::from("  ctrl-j         Scroll down 1 line in list while in search mode"),
      Line::from("  ctrl-k         Scroll up 1 line in list while in search mode"),
      Line::from(""),
      Line::from(vec![Span::styled("THEMES", Style::default().bold().fg(theme.alias_color))]),
      Line::from("  tj             Cycle to next theme"),
      Line::from("  tk             Cycle to previous theme"),
      Line::from(""),
      Line::from(vec![Span::styled("QUIT", Style::default().bold().fg(theme.alias_color))]),
      Line::from("  q              Quit application (normal mode only)"),
      Line::from("  ctrl-c / ctrl-d  Force quit (works in any mode, including search and help)"),
      Line::from(""),
      Line::from(vec![Span::styled("GENERAL", Style::default().bold().fg(theme.alias_color))]),
      Line::from("  ?              Toggle this help screen"),
      Line::from("  esc            Exit search mode OR clear pending key state"),
      Line::from(""),
      Line::from(vec![Span::styled("TIPS", Style::default().bold().fg(theme.function_color))]),
      Line::from("  • Search is case-insensitive (uppercase letters auto-convert to lowercase)"),
      Line::from("  • Two-key sequences (gg, og, etc.) show hints in footer while waiting"),
      Line::from("  • Active panel is indicated by double-line border"),
      Line::from("  • Group mode: '@' shows aliases first, 'ƒ' shows functions first"),
      Line::from(""),
   ];

   // Calculate content dimensions for scrollbar
   let total_lines = help_text.len();
   let inner_area = modal_block.inner(modal_area);
   let visible_lines = inner_area.height as usize;

   // Update max scroll in app state
   app.update_help_max_scroll(total_lines, visible_lines);

   let content = Paragraph::new(help_text)
      .block(modal_block)
      .style(Style::default().fg(Color::White).bg(theme.background))
      .wrap(Wrap {
         trim: false,
      })
      .scroll((app.help_scroll_offset() as u16, 0));

   frame.render_widget(content, modal_area);

   // Render scrollbar only if content extends beyond visible area
   render_scrollbar(frame, inner_area, total_lines, visible_lines, app.help_scroll_offset(), theme.highlight);
}
