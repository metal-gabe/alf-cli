//! Terminal User Interface (TUI) implementation.

pub mod app;
pub mod events;
pub mod keybinds;
pub mod mock;
pub mod state;
pub mod syntax;
pub mod themes;
pub mod ui;

pub use app::App;
pub use events::{Event, EventHandler};
pub use themes::Theme;

use anyhow::Result;
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;
use std::time::Duration;

/// Initialize and run the TUI application.
///
/// Sets up the terminal, creates the application with mock data,
/// runs the main event loop, and restores the terminal on exit.
pub fn run() -> Result<()> {
   // Setup terminal
   terminal::enable_raw_mode()?;
   let mut stdout = io::stdout();
   crossterm::execute!(stdout, EnterAlternateScreen)?;
   let backend = CrosstermBackend::new(stdout);
   let mut terminal = Terminal::new(backend)?;

    // Create app with mock data and default theme
    let entries = mock::mock_entries();
    let theme = Theme::default_theme();
    let mut app = App::new(entries, theme);

   // Create event handler (tick every 250ms)
   let event_handler = EventHandler::new(Duration::from_millis(250));

   // Main event loop
   let result = run_loop(&mut terminal, &mut app, &event_handler);

   // Restore terminal regardless of result
   terminal::disable_raw_mode()?;
   crossterm::execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
   terminal.show_cursor()?;

   result
}

/// The main event loop
fn run_loop(
   terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
   app: &mut App,
   event_handler: &EventHandler,
) -> Result<()> {
   while !app.should_quit {
      // Check and clear expired pending keys before drawing
      if app.is_pending_key_expired() {
         app.clear_pending_key();
      }

      // Draw the UI
      terminal.draw(|frame| ui::draw(frame, &mut *app))?;

      // Handle events
      match event_handler.next()? {
         Event::Key(key) => keybinds::handle_key_event(app, key),
         Event::Resize(_, _) => {
            // ratatui handles terminal resize automatically on next draw
         }
         Event::Tick => app.tick(),
      }
   }

   Ok(())
}
