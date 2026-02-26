//! Configuration management for alf.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
   pub general: GeneralConfig,
   pub search: SearchConfig,
   pub ui: UiConfig,
   pub display: DisplayConfig,
}

/// General configuration options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
   /// List of shell files to parse (supports glob patterns)
   pub shell_files: Vec<String>,
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
         general: GeneralConfig {
            shell_files: Vec::new(), // Configured on first run
         },
         search: SearchConfig {
            case_matching: CaseMatching::Smart,
            normalize: true,
            enable_regex: true,
            substring_matching: true,
         },
         ui: UiConfig { theme: "default".to_string(), keybind_mode: "vim".to_string() },
         display: DisplayConfig { show_type_badges: true, syntax_highlighting: true, parse_comments: true },
      }
   }
}

/// Get the platform-specific configuration file path
///
/// - Linux/macOS: `~/.config/alf/config.toml`
/// - Windows: `%APPDATA%\alf\config.toml`
pub fn get_config_path() -> Result<PathBuf> {
   let home = std::env::var("HOME")
      .or_else(|_| std::env::var("USERPROFILE")) // Windows fallback
      .unwrap_or_else(|_| ".".to_string());

   let config_dir = PathBuf::from(home).join(".config").join("alf");
   Ok(config_dir.join("config.toml"))
}

/// Load configuration from disk
pub fn load_config() -> Result<Config> {
   let path = get_config_path()?;
   let content = fs::read_to_string(&path)?;
   let config: Config = toml::from_str(&content)?;
   Ok(config)
}

/// Save configuration to disk
pub fn save_config(config: &Config) -> Result<()> {
   let path = get_config_path()?;

   // Create the config directory if it doesn't exist
   if let Some(parent) = path.parent() {
      fs::create_dir_all(parent)?;
   }

   let content = toml::to_string_pretty(config)?;
   fs::write(&path, content)?;
   Ok(())
}

/// Check if this is the first run (config doesn't exist)
pub fn is_first_run() -> Result<bool> {
   let path = get_config_path()?;
   Ok(!path.exists())
}
