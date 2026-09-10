//! Configuration management for alf.

use anyhow::Result;
use fs4::FileExt;
use serde::{Deserialize, Serialize};
use std::env::var_os;
use std::fs::{self, File, OpenOptions};
use std::path::PathBuf;
use std::process::id;

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
   pub display: DisplayConfig,
   pub general: GeneralConfig,
   pub search: SearchConfig,
   pub ui: UiConfig,
}

/// Controls what gets populated when Tab is pressed on an alias
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum AliasExpansion {
   #[default]
   Name,
   Script,
}

/// General configuration options
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct GeneralConfig {
   /// List of shell files to parse (supports glob patterns)
   pub shell_files: Vec<String>,
   pub alias_expansion: AliasExpansion,
}

/// Search behavior configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchConfig {
   /// Case matching strategy
   pub case_matching: CaseMatching,
   /// Enable Unicode normalization
   pub normalize: bool,
   /// Enable regex support
   pub enable_regex: bool,
   /// Enable substring matching
   pub substring_matching: bool,
}

/// Case matching options for search
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CaseMatching {
   /// Ignore case entirely
   Ignore,
   /// Smart case (case-insensitive unless query has uppercase)
   Smart,
   /// Respect case exactly
   Respect,
}

/// UI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
   /// Selected theme name
   pub theme: String,
   /// Keybinding mode (currently only "vim" is supported)
   pub keybind_mode: String,
}

/// Display preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
   /// Show type badges (Alias/Function)
   pub show_type_badges: bool,
   /// Enable syntax highlighting
   pub syntax_highlighting: bool,
   /// Parse and display comments
   pub parse_comments: bool,
}

impl Default for Config {
   fn default() -> Self {
      Self {
         display: DisplayConfig {
            parse_comments: true,
            show_type_badges: true,
            syntax_highlighting: true,
         },
         general: GeneralConfig::default(),
         search: SearchConfig {
            case_matching: CaseMatching::Smart,
            enable_regex: true,
            normalize: true,
            substring_matching: true,
         },
         ui: UiConfig {
            keybind_mode: "vim".to_string(),
            theme: "default".to_string(),
         },
      }
   }
}

/// Get the platform-specific configuration file path
///
/// - Linux/macOS: `$HOME/.config/alf/config.toml`
/// - Windows: `%USERPROFILE%\.config\alf\config.toml`
pub fn get_config_path() -> Result<PathBuf> {
   let home = std::env::var("HOME")
      .or_else(|_| std::env::var("USERPROFILE"))
      .map_err(|_| anyhow::anyhow!("HOME or USERPROFILE environment variable not set"))?;

   let config_dir = PathBuf::from(home).join(".config").join("alf");
   Ok(config_dir.join("config.toml"))
}

/// Get the path of the lock file guarding the configuration file
///
/// Sits beside the configuration file as `config.toml.lock` so the lock survives the config file
/// being replaced by a rename.
pub fn get_config_lock_path() -> Result<PathBuf> {
   Ok(get_config_path()?.with_extension("toml.lock"))
}

/// An exclusive, process-safe lock over the configuration file
///
/// Hold this across a read-modify-write cycle — from before [`load_config`] until after
/// [`save_config`] — so two concurrent `alf` processes cannot both load the same configuration and
/// have the later save silently discard the earlier one's changes.
///
/// The lock is advisory: it excludes other holders of this same lock, not an unrelated process or
/// a hand edit of the file. It is released when the guard is dropped, which covers early returns,
/// errors and panics alike, and the operating system releases it if the process dies while holding
/// it, so a crash cannot leave the lock stuck.
pub struct ConfigLock {
   file: File,
}

impl ConfigLock {
   /// Acquire the lock, blocking until any other holder releases it
   ///
   /// # Errors
   /// Returns an error if the configuration directory cannot be created, the lock file cannot be
   /// opened, or the underlying lock cannot be acquired.
   pub fn acquire() -> Result<Self> {
      let path = get_config_lock_path()?;

      if let Some(parent) = path.parent() {
         fs::create_dir_all(parent)?;
      }

      let file = OpenOptions::new().create(true).read(true).write(true).truncate(false).open(&path)?;
      FileExt::lock_exclusive(&file)?;

      Ok(Self {
         file,
      })
   }
}

impl Drop for ConfigLock {
   fn drop(&mut self) {
      let _ = FileExt::unlock(&self.file);
   }
}

/// Resolve the home directory used to expand `~` and `$HOME`
///
/// `HOME` and `USERPROFILE` are consulted first, in that order, which matches how
/// [`get_config_path`] locates the configuration file, so both agree on where home is and a caller
/// that overrides the environment is honoured on every platform.
///
/// `dirs::home_dir` is only the fallback. On Windows it reads the profile known folder and ignores
/// both variables, so relying on it alone would expand `~` to the real user profile even when the
/// environment points somewhere else — which silently defeats an isolated test home.
fn resolve_home_dir() -> Option<PathBuf> {
   for key in ["HOME", "USERPROFILE"] {
      match var_os(key) {
         Some(value) if !value.is_empty() => return Some(PathBuf::from(value)),
         _ => {},
      }
   }

   dirs::home_dir()
}

/// Expand a leading `~` or `$HOME` in a configured file path into an absolute path
///
/// Paths without a home prefix are returned unchanged.
pub fn expand_path(file_path_str: &str) -> PathBuf {
   let expanded = if let Some(home_dir) = resolve_home_dir() {
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

/// Load configuration from disk
pub fn load_config() -> Result<Config> {
   let path = get_config_path()?;
   let content = fs::read_to_string(&path)?;
   let config: Config = toml::from_str(&content)?;
   Ok(config)
}

/// Save configuration to disk
///
/// The new contents are written to a sibling temporary file and then renamed over the target, so a
/// failure part-way through leaves the previous configuration intact rather than a truncated file.
/// The temporary name carries the process id to keep concurrent writers from sharing it.
pub fn save_config(config: &Config) -> Result<()> {
   let path = get_config_path()?;

   // `parent()` yields the config file's directory, or `None` for a path with no parent at all.
   // `create_dir_all` builds every missing ancestor and succeeds when they already exist, so it
   // acts as the existence check itself
   if let Some(parent) = path.parent() {
      fs::create_dir_all(parent)?;
   }

   let content = toml::to_string_pretty(config)?;
   let temp_path = path.with_extension(format!("toml.{}.tmp", id()));

   if let Err(error) = fs::write(&temp_path, content) {
      let _ = fs::remove_file(&temp_path);
      return Err(error.into());
   }

   if let Err(error) = fs::rename(&temp_path, &path) {
      let _ = fs::remove_file(&temp_path);
      return Err(error.into());
   }

   Ok(())
}

/// Check if this is the first run (config doesn't exist)
pub fn is_first_run() -> Result<bool> {
   let path = get_config_path()?;
   Ok(!path.exists())
}

#[cfg(test)]
mod config_tests;
