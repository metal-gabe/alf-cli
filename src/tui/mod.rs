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

use anyhow::{Context, Result};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;
use std::time::Duration;

/// Restores the terminal to its normal state when dropped, including during a panic unwind
struct TerminalGuard;

impl Drop for TerminalGuard {
   fn drop(&mut self) {
      restore_terminal();
   }
}

/// Initialize and run the TUI application.
///
/// Loads configuration and shell files before touching the terminal, so load warnings and errors
/// stay visible on the normal screen, then sets up the terminal and runs the main event loop.
/// The terminal is restored on return, on error and on panic.
///
/// # Arguments
/// * `initial_query` - Optional search query to populate and filter results on startup
///
/// # Errors
/// Returns an error when no aliases or functions could be loaded, or when the terminal fails
pub fn run(initial_query: Option<String>) -> Result<()> {
   let config = crate::config::load_optional_config()?;
   let entries = load_entries(config.as_ref())
      .context("No shell files could be read. Add one with `alf config add <file>` or run `alf init`")?;

   let theme = config.as_ref().and_then(|cfg| Theme::from_name(&cfg.ui.theme)).unwrap_or_else(Theme::default_theme);

   let mut app = App::new(entries, theme);
   app.set_parse_comments(config.as_ref().map(|cfg| cfg.display.parse_comments).unwrap_or_default());
   app.enter_search_mode();

   if let Some(query) = initial_query {
      app.set_search_query(query);
   }

   let event_handler = EventHandler::new(Duration::from_millis(250));

   install_panic_hook();
   terminal::enable_raw_mode()?;
   let guard = TerminalGuard;
   crossterm::execute!(io::stdout(), EnterAlternateScreen)?;
   let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
   let result = run_loop(&mut terminal, &mut app, &event_handler);
   drop(guard);
   result?;

   if let Some(action) = app.exit_action() {
      if let Some(entry) = app.selected_entry() {
         let alias_expansion = config.as_ref().map(|c| c.general.alias_expansion).unwrap_or_default();
         if let Ok(output_path) = std::env::var("ALF_OUTPUT") {
            std::fs::write(&output_path, build_exit_output(action, entry, alias_expansion))
               .with_context(|| format!("Failed to write the selected entry to {}", output_path))?;
         }
      }
   }

   Ok(())
}

/// Build the `ALF_OUTPUT` file contents the shell wrapper reads: the action, then the value to act on
///
/// The value is the entry's name, except that populating an alias uses its script when `alias_expansion`
/// asks for it.
fn build_exit_output(
   action: app::ExitAction,
   entry: &crate::models::AliasEntry,
   alias_expansion: crate::config::AliasExpansion,
) -> String {
   let action_str = match action {
      app::ExitAction::Execute => "execute",
      app::ExitAction::Populate => "populate",
   };
   let output_value = match action {
      app::ExitAction::Populate
         if entry.entry_type == crate::models::EntryType::Alias
            && matches!(alias_expansion, crate::config::AliasExpansion::Script) =>
      {
         &entry.value
      },
      _ => &entry.name,
   };

   format!("{}\n{}", action_str, output_value)
}

/// Leave raw mode and the alternate screen and show the cursor, ignoring each step's error so every step runs
fn restore_terminal() {
   let _ = terminal::disable_raw_mode();
   let _ = crossterm::execute!(io::stdout(), LeaveAlternateScreen, crossterm::cursor::Show);
}

/// Install a panic hook that restores the terminal before the previous hook prints the panic message
fn install_panic_hook() {
   let previous_hook = std::panic::take_hook();
   std::panic::set_hook(Box::new(move |info| {
      restore_terminal();
      previous_hook(info);
   }));
}

/// The main event loop
fn run_loop(
   terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
   app: &mut App,
   event_handler: &EventHandler,
) -> Result<()> {
   while app.is_running() {
      if app.is_pending_key_expired() {
         app.clear_pending_key();
      }

      terminal.draw(|frame| ui::draw(frame, &mut *app))?;

      match event_handler.next()? {
         Event::Key(key) => keybinds::handle_key_event(app, key),
         Event::Resize(_, _) => {},
         Event::Tick => app.tick(),
      }
   }

   Ok(())
}

/// Load entries from the configured shell files, or from the standard shell files when no config exists
///
/// A file that parses but defines no aliases or functions is not a failure, so the result can be empty.
///
/// # Errors
/// Returns an error when no shell file could be found or parsed
pub fn load_entries(config: Option<&crate::config::Config>) -> Result<Vec<crate::models::AliasEntry>> {
   match config {
      Some(cfg) => load_shell_entries_from_config(cfg),
      None => load_shell_entries(),
   }
}

/// Try to load entries from shell configuration files specified in config
fn load_shell_entries_from_config(config: &crate::config::Config) -> Result<Vec<crate::models::AliasEntry>> {
   let mut entries = Vec::new();
   let mut was_any_file_parsed = false;

   for file_path_str in &config.general.shell_files {
      let shell_file = crate::config::expand_path(file_path_str);
      log::debug!("Handled file path string:\n- orig: {},\n- expanded: {}", file_path_str, shell_file.display());

      if shell_file.exists() {
         match crate::parser::parse_shell_file(&shell_file) {
            Ok(file_entries) => {
               entries.extend(file_entries);
               was_any_file_parsed = true;
            },
            Err(e) => eprintln!("Warning: Failed to parse {}: {}", shell_file.display(), e),
         }
      } else {
         eprintln!("Warning: Shell file not found: {}", shell_file.display());
      }
   }

   if !was_any_file_parsed {
      anyhow::bail!("No shell configuration files found or parsed successfully")
   }

   Ok(entries)
}

/// Try to load entries from shell configuration files
fn load_shell_entries() -> Result<Vec<crate::models::AliasEntry>> {
   let mut entries = Vec::new();
   let mut was_any_file_parsed = false;

   let home =
      crate::config::resolve_home_dir().context("Could not determine the home directory: set HOME or USERPROFILE")?;

   for shell_file in crate::cli::init::detect_shell_files(&home.to_string_lossy()) {
      match crate::parser::parse_shell_file(std::path::Path::new(&shell_file)) {
         Ok(file_entries) => {
            entries.extend(file_entries);
            was_any_file_parsed = true;
         },
         Err(e) => eprintln!("Warning: Failed to parse {}: {}", shell_file, e),
      }
   }

   if !was_any_file_parsed {
      anyhow::bail!("No shell configuration files found or parsed successfully")
   }

   Ok(entries)
}

#[cfg(test)]
#[path = "output_tests.rs"]
mod output_tests;
