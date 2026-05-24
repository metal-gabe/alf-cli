//! Shared UI component helpers.
//!
//! Reusable functions for common UI patterns to eliminate code duplication.

use ratatui::{
   layout::Rect,
   style::{Color, Modifier, Style},
   text::Span,
   widgets::{Block, BorderType, Borders, Scrollbar, ScrollbarOrientation, ScrollbarState},
   Frame,
};

use super::get_border_style;
use crate::tui::state::EntryFilter;
use crate::tui::themes::Theme;

/// Build a standard panel block with borders, title, and active/inactive styling.
///
/// # Arguments
/// * `title` - The title text to display (e.g., "[ Entries ]")
/// * `is_active` - Whether this panel is currently active
/// * `filter` - The current entry filter for border color styling
/// * `theme` - The current theme for color selection
pub fn panel_block<'a>(
   title: &'a str,
   is_active: bool,
   filter: &EntryFilter,
   theme: &Theme,
) -> Block<'a> {
   Block::default()
      .borders(Borders::ALL)
      .title(Span::styled(title, if is_active { Style::default().fg(theme.foreground) } else { Style::default() }))
      .border_type(if is_active { BorderType::Double } else { BorderType::Plain })
      .border_style(if is_active { get_border_style(filter, theme) } else { Style::default() })
}

/// Render a vertical scrollbar if content overflows the visible area.
///
/// # Arguments
/// * `frame` - The ratatui Frame to render to
/// * `area` - The area where the scrollbar should be rendered
/// * `total_lines` - Total number of lines in the content
/// * `visible_lines` - Number of lines visible in the viewport
/// * `scroll_offset` - Current scroll position
/// * `color` - Color for the scrollbar
pub fn render_scrollbar(
   frame: &mut Frame,
   area: Rect,
   total_lines: usize,
   visible_lines: usize,
   scroll_offset: usize,
   color: Color,
) {
   if total_lines <= visible_lines {
      return;
   }

   let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
      .style(Style::default().fg(color))
      .begin_symbol(Some("↑"))
      .end_symbol(Some("↓"));

   let mut state = ScrollbarState::new(total_lines.saturating_sub(visible_lines)).position(scroll_offset);

   frame.render_stateful_widget(scrollbar, area, &mut state);
}

/// Get the appropriate text style for active/inactive panels.
///
/// # Arguments
/// * `is_active` - Whether the panel is active
/// * `use_bold` - If true, adds BOLD modifier when inactive (becomes BOLD | DIM); when active returns empty Style
///
/// # Returns
/// * If `is_active` and `use_bold` is true: empty Style with BOLD
/// * If `is_active` and `use_bold` is false: empty Style
/// * If inactive and `use_bold` is true: BOLD | DIM
/// * If inactive and `use_bold` is false: DIM only
pub fn active_style(
   is_active: bool,
   use_bold: bool,
) -> Style {
   match (is_active, use_bold) {
      (true, true) => Style::default().add_modifier(Modifier::BOLD),
      (true, false) => Style::default(),
      (false, true) => Style::default().add_modifier(Modifier::BOLD | Modifier::DIM),
      (false, false) => Style::default().add_modifier(Modifier::DIM),
   }
}
