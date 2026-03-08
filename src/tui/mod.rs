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
/// Sets up the terminal, loads configuration, attempts to load shell configuration files,
/// falls back to mock data if parsing fails, and runs the main event loop.
///
/// # Arguments
/// * `initial_query` - Optional search query to populate and filter results on startup
pub fn run(initial_query: Option<String>) -> Result<()> {
   // Setup terminal
   terminal::enable_raw_mode()?;
   let mut stdout = io::stdout();
   crossterm::execute!(stdout, EnterAlternateScreen)?;
   let backend = CrosstermBackend::new(stdout);
   let mut terminal = Terminal::new(backend)?;

   // Try to load configuration
   let config = crate::config::load_config().ok();

   // Try to load real shell configuration files, fall back to mock data
   let entries = if let Some(ref cfg) = config {
      load_shell_entries_from_config(cfg).unwrap_or_else(|_| {
         eprintln!("Failed to load shell files from config, using mock data");
         mock::mock_entries()
      })
   } else {
      load_shell_entries().unwrap_or_else(|_| {
         eprintln!("Failed to load shell files, using mock data");
         mock::mock_entries()
      })
   };

   // Get theme from config or use default
   let theme = config.as_ref().and_then(|cfg| Theme::from_name(&cfg.ui.theme)).unwrap_or_else(Theme::default_theme);

   let mut app = App::new(entries, theme);

   // If an initial query was provided, populate it and search
   if let Some(query) = initial_query {
      app.set_search_query(query);
   }

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

/// Try to load entries from shell configuration files specified in config
fn load_shell_entries_from_config(config: &crate::config::Config) -> Result<Vec<crate::models::AliasEntry>> {
   let mut entries = Vec::new();

   // Parse each shell file from config
   for file_path_str in &config.general.shell_files {
      let shell_file = PathBuf::from(file_path_str);
      if shell_file.exists() {
         match crate::parser::parse_shell_file(&shell_file) {
            Ok(file_entries) => entries.extend(file_entries),
            Err(e) => eprintln!("Warning: Failed to parse {}: {}", shell_file.display(), e),
         }
      } else {
         eprintln!("Warning: Shell file not found: {}", shell_file.display());
      }
   }

   if entries.is_empty() {
      anyhow::bail!("No shell configuration files found or parsed successfully")
   }

   Ok(entries)
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
