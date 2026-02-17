//! UI rendering logic for the TUI.
//!
//! Implements the full layout matching the design:
//! - Header bar with filter badges and shell indicator
//! - Search bar with cursor support
//! - Main body: left list panel + right detail panels (description + script)
//! - Footer bar with help text

pub mod body;
mod colors;
mod components;
mod footer;
mod header;
mod help;
mod search;

use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Modifier, Style, Stylize};
use ratatui::widgets::Block;
use ratatui::Frame;

use crate::tui::app::App;
use crate::tui::state::{EntryFilter, InputMode};
use colors::*;

/// Get the border style for the current filter
pub(super) fn get_border_style(filter: &EntryFilter) -> Style {
   match filter {
      EntryFilter::Aliases => Style::default().fg(COLOR_ALIAS).add_modifier(Modifier::BOLD),
      EntryFilter::Functions => Style::default().fg(COLOR_FUNCTION).add_modifier(Modifier::BOLD),
      _ => Style::default().white().add_modifier(Modifier::BOLD),
   }
}

// Re-export the draw functions for use by other modules
pub use body::draw_main_body;
pub use footer::draw_footer;
pub use header::draw_header;
pub use help::draw_help_modal;
pub use search::draw_search_bar;

/// Draw the complete TUI interface
pub fn draw(frame: &mut Frame, app: &mut App) {
   // Apply global background color to entire TUI
   let background = Block::default().style(Style::default().bg(COLOR_BACKGROUND));
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
   if app.show_help() {
      draw_help_modal(frame, app);
   }

   // Place cursor in search bar when in search mode (and help is not showing)
   if app.input_mode() == InputMode::Search && !app.show_help() {
      // Cursor position: inside the search block (1 char border + 1 char padding + cursor_position)
      frame.set_cursor_position((outer_chunks[1].x + 2 + app.cursor_position() as u16, outer_chunks[1].y + 1));
   }
}
