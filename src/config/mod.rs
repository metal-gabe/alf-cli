//! Configuration management for alf.

use anyhow::Result;
use serde::{Deserialize, Serialize};
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
/// - Linux: `~/.config/alf/config.toml`
/// - macOS: `~/.config/alf/config.toml`
/// - Windows: `%APPDATA%\alf\config.toml`
pub fn get_config_path() -> Result<PathBuf> {
   // TODO: Implement using directories crate
   todo!("Implement config path resolution")
}

/// Load configuration from disk
pub fn load_config() -> Result<Config> {
   // TODO: Implement config loading
   todo!("Implement config loading")
}

/// Save configuration to disk
pub fn save_config(_config: &Config) -> Result<()> {
   // TODO: Implement config saving
   todo!("Implement config saving")
}

/// Check if this is the first run (config doesn't exist)
pub fn is_first_run() -> Result<bool> {
   // TODO: Implement first-run detection
   todo!("Implement first-run detection")
}
