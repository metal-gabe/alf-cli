//! UI rendering logic for the TUI.

use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::{Block, Borders};
use ratatui::Frame;

use crate::tui::App;

/// Draw the TUI interface
pub fn draw(_frame: &mut Frame, _app: &App) {
    // TODO: Implement UI rendering
    // - Create main layout with Layout::default()
    // - Top section: Search input box
    // - Middle section: Split into two columns
    //   - Left: Results list (scrollable)
    //   - Right: Detail view (scrollable)
    // - Bottom section: Help/status bar

    // Example layout structure:
    let _chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Search bar
            Constraint::Min(0),    // Main content
            Constraint::Length(1), // Help bar
        ])
        .split(_frame.area());

    // TODO: Render widgets into each chunk
    let _block = Block::default().borders(Borders::ALL).title("alf");
}
