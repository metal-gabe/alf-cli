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
   terminal::enable_raw_mode()?;
   let mut stdout = io::stdout();
   crossterm::execute!(stdout, EnterAlternateScreen)?;
   let backend = CrosstermBackend::new(stdout);
   let mut terminal = Terminal::new(backend)?;

   // Try to load configuration
   let config = crate::config::load_config()
      .map_err(|e| {
         log::debug!("Failed to load config, using defaults: {}", e);
         e
      })
      .ok();

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
   app.enter_search_mode();

   // If an initial query was provided, populate it and search
   if let Some(query) = initial_query {
      app.set_search_query(query);
   }

   // Create event handler (tick every 250ms)
   let event_handler = EventHandler::new(Duration::from_millis(250));

   // Main event loop
   let result = run_loop(&mut terminal, &mut app, &event_handler);

   // Write selected entry to output file if applicable
   if let Some(action) = &app.exit_action {
      if let Some(entry) = app.selected_entry() {
         let action_str = match action {
            crate::tui::app::ExitAction::Execute => "execute",
            crate::tui::app::ExitAction::Populate => "populate",
         };
         let alias_expansion = config.as_ref().map(|c| c.general.alias_expansion).unwrap_or_default();
         let output_value = match action {
            crate::tui::app::ExitAction::Populate
               if entry.entry_type == crate::models::EntryType::Alias
                  && matches!(alias_expansion, crate::config::AliasExpansion::Script) =>
            {
               &entry.value
            }
            _ => &entry.name,
         };
         if let Ok(output_path) = std::env::var("ALF_OUTPUT") {
            if let Err(e) = std::fs::write(&output_path, format!("{}\n{}", action_str, output_value)) {
               log::warn!("Failed to write output to {}: {}", output_path, e);
            }
         }
      }
   }

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
      let shell_file = expand_path(file_path_str);
      log::debug!("Handled file path string:\n- orig: {},\n- expanded: {}", file_path_str, shell_file.display());

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

fn expand_path(file_path_str: &str) -> PathBuf {
   let expanded = if let Some(home_dir) = dirs::home_dir() {
      let path = if let Some(rest) = file_path_str.strip_prefix("~/") {
         home_dir.join(rest)
      } else if file_path_str == "~" {
         home_dir.clone()
      } else if let Some(rest) = file_path_str.strip_prefix("$HOME/") {
         home_dir.join(rest)
      } else if file_path_str == "$HOME" {
         home_dir.clone()
      } else {
         PathBuf::from(file_path_str)
      };
      path
   } else {
      PathBuf::from(file_path_str)
   };

   expanded
}

#[cfg(test)]
mod tests {
    use super::expand_path;
    use std::env;

    fn home() -> String {
        env::var("HOME").unwrap_or_else(|_| "/tmp".to_string())
    }

    #[test]
    fn test_expand_path_tilde_slash_prefix() {
        let result = expand_path("~/foo/bar");
        assert_eq!(result, std::path::PathBuf::from(home()).join("foo/bar"));
    }

    #[test]
    fn test_expand_path_home_env_slash_prefix() {
        let result = expand_path("$HOME/foo/bar");
        assert_eq!(result, std::path::PathBuf::from(home()).join("foo/bar"));
    }

    #[test]
    fn test_expand_path_tilde_alone() {
        let result = expand_path("~");
        assert_eq!(result, std::path::PathBuf::from(home()));
    }

    #[test]
    fn test_expand_path_home_env_alone() {
        let result = expand_path("$HOME");
        assert_eq!(result, std::path::PathBuf::from(home()));
    }

    #[test]
    fn test_expand_path_absolute_passthrough() {
        let result = expand_path("/etc/shells");
        assert_eq!(result, std::path::PathBuf::from("/etc/shells"));
    }

    #[test]
    fn test_expand_path_relative_passthrough() {
        let result = expand_path("relative/path");
        assert_eq!(result, std::path::PathBuf::from("relative/path"));
    }
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
