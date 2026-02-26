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
use std::path::PathBuf;
use std::time::Duration;

/// Initialize and run the TUI application.
///
/// Sets up the terminal, attempts to load shell configuration files,
/// falls back to mock data if parsing fails, and runs the main event loop.
pub fn run() -> Result<()> {
   // Setup terminal
   terminal::enable_raw_mode()?;
   let mut stdout = io::stdout();
   crossterm::execute!(stdout, EnterAlternateScreen)?;
   let backend = CrosstermBackend::new(stdout);
   let mut terminal = Terminal::new(backend)?;

   // Try to load real shell configuration files, fall back to mock data
   let entries = load_shell_entries().unwrap_or_else(|_| {
      eprintln!("Failed to load shell files, using mock data");
      mock::mock_entries()
   });
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

/// Try to load entries from shell configuration files
fn load_shell_entries() -> Result<Vec<crate::models::AliasEntry>> {
   let mut entries = Vec::new();

   // Try to load from common shell config files
   let shell_files = [
      PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".to_string())).join(".bashrc"),
      PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".to_string())).join(".zshrc"),
      PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".to_string())).join(".kshrc"),
      PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".to_string())).join(".fishrc"),
      PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".to_string())).join(".profile"),
      PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".to_string())).join(".zprofile"),
   ];

   // Parse each shell file if it exists
   for shell_file in &shell_files {
      if shell_file.exists() {
         match crate::parser::parse_shell_file(shell_file) {
            Ok(file_entries) => entries.extend(file_entries),
            Err(e) => eprintln!("Warning: Failed to parse {}: {}", shell_file.display(), e),
         }
      }
   }

   if entries.is_empty() {
      anyhow::bail!("No shell configuration files found or parsed successfully")
   }

   Ok(entries)
}
