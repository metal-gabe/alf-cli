//! Configuration management commands (add, show, edit, reset).

use crate::cli::init;
use crate::cli::ConfigAction;
use crate::config::{
   expand_path, get_config_path, is_first_run, load_config, save_config, Config, ConfigLock, GeneralConfig,
};
use anyhow::Result;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

/// Run a configuration management action
pub fn run_config_action(action: ConfigAction) -> Result<()> {
   match action {
      ConfigAction::Add {
         paths,
      } => add_source_files(&paths),
      ConfigAction::Edit => edit_config(),
      ConfigAction::Reset => reset_config(),
      ConfigAction::Show => show_config(),
   }
}

/// Add one or more shell source files to the configured `shell_files` list
///
/// The load-modify-save cycle runs under an exclusive [`ConfigLock`], so a concurrent `alf config
/// add` cannot read the same starting configuration and overwrite the entries this run appends.
/// The guard releases the lock on every exit path, including the early returns and the `?` bails.
fn add_source_files(raw_paths: &[String]) -> Result<()> {
   let config_path = get_config_path()?;

   if is_first_run()? {
      anyhow::bail!("No config found at {}. Run `alf init` to create one.", config_path.display());
   }

   let _lock = ConfigLock::acquire()?;
   let mut config = load_config()?;
   let (to_add, duplicates) = resolve_new_source_files(&config, raw_paths)?;

   for duplicate in &duplicates {
      println!("Already configured: {}", duplicate);
   }

   if to_add.is_empty() {
      println!("No new source files added.");
      return Ok(());
   }

   config.general.shell_files.extend(to_add.iter().cloned());
   save_config(&config)?;

   for added in &to_add {
      println!("Added: {}", added);
   }

   println!("Config saved to {}", config_path.display());
   Ok(())
}

/// Split the given paths into new source files and ones the config already tracks
///
/// Paths are compared in canonical form, so entries that reach the same file through a symlink,
/// a `..` segment, or a different `~`/`$HOME` spelling count as duplicates. The caller's original
/// spelling is what gets returned, and therefore what gets stored.
///
/// # Errors
/// Returns an error if any path is relative, does not exist on disk, or is not a regular file,
/// before classifying any of them, so a single bad path leaves the configuration untouched.
/// Relative paths are rejected first, so their message is never masked by a missing-file error.
///
/// Directories, sockets and FIFOs are rejected because the parser reads each configured entry as a
/// file: a directory would warn on every launch, and a FIFO would block the read. Both checks
/// follow symlinks, so a symlink to a regular file is still accepted.
fn resolve_new_source_files(
   config: &Config,
   raw_paths: &[String],
) -> Result<(Vec<String>, Vec<String>)> {
   for raw_path in raw_paths {
      if !expand_path(raw_path).is_absolute() {
         anyhow::bail!(
            "Relative paths are not allowed: {}. Use an absolute path or one starting with `~` or `$HOME`.",
            raw_path
         );
      }
   }

   for raw_path in raw_paths {
      let expanded = expand_path(raw_path);

      if !expanded.exists() {
         anyhow::bail!("Shell file not found: {}", expanded.display());
      }

      if !expanded.is_file() {
         anyhow::bail!("Shell source path is not a regular file: {}", expanded.display());
      }
   }

   let mut configured: Vec<PathBuf> =
      config.general.shell_files.iter().map(String::as_str).map(canonical_key).collect();

   let mut to_add = Vec::new();
   let mut duplicates = Vec::new();

   for raw_path in raw_paths {
      let key = canonical_key(raw_path);

      if configured.contains(&key) {
         duplicates.push(raw_path.clone());
      } else {
         configured.push(key);
         to_add.push(raw_path.clone());
      }
   }

   Ok((to_add, duplicates))
}

/// Resolve a configured path to the key used for duplicate comparison
///
/// Falls back to the merely expanded path when the file cannot be canonicalized, which happens for
/// stale `shell_files` entries pointing at files that no longer exist.
fn canonical_key(raw_path: &str) -> PathBuf {
   let expanded = expand_path(raw_path);
   fs::canonicalize(&expanded).unwrap_or(expanded)
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

   let config = Config {
      general: GeneralConfig {
         shell_files: detected_files,
         ..Default::default()
      },
      ..Default::default()
   };

   save_config(&config)?;

   let config_path = get_config_path()?;
   println!("Config reset to defaults and saved to {}", config_path.display());

   Ok(())
}

#[cfg(test)]
#[path = "config_cmd_tests.rs"]
mod config_cmd_tests;
