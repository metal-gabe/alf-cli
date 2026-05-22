//! Configuration management commands (show, edit, reset).

use crate::cli::init;
use crate::cli::ConfigAction;
use crate::config::{get_config_path, load_config, save_config, Config, GeneralConfig};
use anyhow::Result;
use std::io::{self, Write};
use std::process::Command;

/// Run a configuration management action
pub fn run_config_action(action: ConfigAction) -> Result<()> {
   match action {
      ConfigAction::Show => show_config(),
      ConfigAction::Edit => edit_config(),
      ConfigAction::Reset => reset_config(),
   }
}

/// Show the current configuration
fn show_config() -> Result<()> {
   let config = load_config()?;
   let config_path = get_config_path()?;

   println!("Location: {}\n", config_path.display());
   println!("{}", toml::to_string_pretty(&config)?);

   Ok(())
}

/// Edit the configuration file in the user's preferred editor
fn edit_config() -> Result<()> {
   let config_path = get_config_path()?;

   // Try $EDITOR first, then fall back to common editors
   let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());

   let status = Command::new(&editor).arg(config_path.to_string_lossy().to_string()).status()?;

   if !status.success() {
      anyhow::bail!("Editor exited with non-zero status");
   }

   Ok(())
}

/// Reset configuration to defaults
fn reset_config() -> Result<()> {
   print!("Are you sure you want to reset configuration? (y/N) ");
   io::stdout().flush()?;

   let mut response = String::new();
   io::stdin().read_line(&mut response)?;

   if !response.trim().eq_ignore_ascii_case("y") {
      println!("Cancelled.");
      return Ok(());
   }

   // Auto-detect standard shell files
   let home = std::env::var("HOME").map_err(|_| anyhow::anyhow!("HOME environment variable is not set"))?;
   let detected_files = init::detect_shell_files(&home);

   let config =
      Config { general: GeneralConfig { shell_files: detected_files, ..Default::default() }, ..Default::default() };

   save_config(&config)?;

   let config_path = get_config_path()?;
   println!("Config reset to defaults and saved to {}", config_path.display());

   Ok(())
}
