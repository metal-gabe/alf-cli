//! Configuration management for alf.

use anyhow::{Context, Result};
use fs4::FileExt;
use serde::{Deserialize, Serialize};
use std::env::var_os;
use std::fs::{self, File, OpenOptions};
use std::path::PathBuf;
use std::process::id;

/// Main configuration structure
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
   pub display: DisplayConfig,
   pub general: GeneralConfig,
   pub search: SearchConfig,
   pub ui: UiConfig,
   pub which: WhichConfig,
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
   /// List of shell files to parse; a leading `~` or `$HOME` is expanded, glob patterns are not
   pub shell_files: Vec<String>,
   pub alias_expansion: AliasExpansion,
}

/// Search behavior configuration, reserved for future use: no setting here has an effect yet
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SearchConfig {
   /// Case matching strategy
   pub case_matching: CaseMatching,
   /// Enable Unicode normalization
   #[serde(rename = "normalize")]
   pub should_normalize: bool,
   /// Enable regex support
   #[serde(rename = "enable_regex")]
   pub is_regex_enabled: bool,
   /// Enable substring matching
   #[serde(rename = "substring_matching")]
   pub is_substring_matching: bool,
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
#[serde(default)]
pub struct UiConfig {
   /// Selected theme name
   pub theme: String,
   /// Keybinding mode, reserved for future use: it has no effect yet and only "vim" is supported
   pub keybind_mode: String,
}

/// Display preferences; only `parse_comments` has an effect so far, the other settings are reserved
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DisplayConfig {
   /// Show type badges (Alias/Function)
   #[serde(rename = "show_type_badges")]
   pub should_show_type_badges: bool,
   /// Enable syntax highlighting
   #[serde(rename = "syntax_highlighting")]
   pub is_syntax_highlighting_enabled: bool,
   /// Whether comments are parsed and when the TUI description panel is shown
   pub parse_comments: ParseComments,
}

/// Whether to parse alf comments and show the TUI description panel, written as `true`, `false` or `"dynamic"`
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "ParseCommentsValue", into = "ParseCommentsValue")]
pub enum ParseComments {
   /// Parse comments and always show the description panel
   #[default]
   Always,
   /// Parse comments and show the description panel only for entries that have an alf comment
   Dynamic,
   /// Skip parsing comments and always hide the description panel
   Never,
}

impl ParseComments {
   /// Whether the description panel should be shown, given whether the entry has an alf comment
   pub fn is_description_visible(
      self,
      does_have_comments: bool,
   ) -> bool {
      match self {
         Self::Always => true,
         Self::Dynamic => does_have_comments,
         Self::Never => false,
      }
   }

   /// Whether comments should be extracted while parsing shell files
   pub fn is_parsing_enabled(self) -> bool {
      !matches!(self, Self::Never)
   }

   /// The next setting in the TUI cycle order: `"dynamic"` → `true` → `false` → `"dynamic"`
   pub fn next(self) -> Self {
      match self {
         Self::Always => Self::Never,
         Self::Dynamic => Self::Always,
         Self::Never => Self::Dynamic,
      }
   }
}

/// The raw TOML form of a [`ParseComments`]: a boolean or a keyword string
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum ParseCommentsValue {
   Flag(bool),
   Keyword(String),
}

impl From<ParseComments> for ParseCommentsValue {
   fn from(setting: ParseComments) -> Self {
      match setting {
         ParseComments::Always => Self::Flag(true),
         ParseComments::Dynamic => Self::Keyword("dynamic".to_string()),
         ParseComments::Never => Self::Flag(false),
      }
   }
}

impl TryFrom<ParseCommentsValue> for ParseComments {
   type Error = String;

   fn try_from(value: ParseCommentsValue) -> Result<Self, Self::Error> {
      match value {
         ParseCommentsValue::Flag(true) => Ok(Self::Always),
         ParseCommentsValue::Flag(false) => Ok(Self::Never),
         ParseCommentsValue::Keyword(keyword) if keyword == "dynamic" => Ok(Self::Dynamic),
         ParseCommentsValue::Keyword(keyword) => {
            Err(format!("invalid parse_comments setting `{}`, expected `true`, `false` or \"dynamic\"", keyword))
         },
      }
   }
}

/// `alf which` output preferences
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct WhichConfig {
   /// When to hide the `type` result box
   pub hide_type: HideSetting,
   /// When to hide the `which` result box
   pub hide_which: HideSetting,
}

/// When to hide an `alf which` lookup box, written as `true`, `false` or `"is_none"`
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "HideSettingValue", into = "HideSettingValue")]
pub enum HideSetting {
   /// Always hide the box
   Always,
   /// Hide the box only when its lookup found nothing
   IsNone,
   /// Never hide the box
   #[default]
   Never,
}

impl HideSetting {
   /// Whether the box should be hidden, given whether its lookup found a result
   pub fn is_hidden(
      self,
      does_have_result: bool,
   ) -> bool {
      match self {
         Self::Always => true,
         Self::IsNone => !does_have_result,
         Self::Never => false,
      }
   }
}

/// The raw TOML form of a [`HideSetting`]: a boolean or a keyword string
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum HideSettingValue {
   Flag(bool),
   Keyword(String),
}

impl From<HideSetting> for HideSettingValue {
   fn from(setting: HideSetting) -> Self {
      match setting {
         HideSetting::Always => Self::Flag(true),
         HideSetting::IsNone => Self::Keyword("is_none".to_string()),
         HideSetting::Never => Self::Flag(false),
      }
   }
}

impl TryFrom<HideSettingValue> for HideSetting {
   type Error = String;

   fn try_from(value: HideSettingValue) -> Result<Self, Self::Error> {
      match value {
         HideSettingValue::Flag(true) => Ok(Self::Always),
         HideSettingValue::Flag(false) => Ok(Self::Never),
         HideSettingValue::Keyword(keyword) if keyword == "is_none" => Ok(Self::IsNone),
         HideSettingValue::Keyword(keyword) => {
            Err(format!("invalid hide setting `{}`, expected `true`, `false` or \"is_none\"", keyword))
         },
      }
   }
}

impl Default for DisplayConfig {
   fn default() -> Self {
      Self {
         parse_comments: ParseComments::Always,
         should_show_type_badges: true,
         is_syntax_highlighting_enabled: true,
      }
   }
}

impl Default for SearchConfig {
   fn default() -> Self {
      Self {
         case_matching: CaseMatching::Smart,
         is_regex_enabled: true,
         should_normalize: true,
         is_substring_matching: true,
      }
   }
}

impl Default for UiConfig {
   fn default() -> Self {
      Self {
         keybind_mode: "vim".to_string(),
         theme: "default".to_string(),
      }
   }
}

/// Get the platform-specific configuration file path
///
/// - Linux/macOS: `$HOME/.config/alf/config.toml`
/// - Windows: `%USERPROFILE%\.config\alf\config.toml`
///
/// Home is resolved by [`resolve_home_dir`], the same way `~` is expanded, so the two can never
/// disagree about where home is.
pub fn get_config_path() -> Result<PathBuf> {
   let home = resolve_home_dir()
      .ok_or_else(|| anyhow::anyhow!("Could not determine the home directory: set HOME or USERPROFILE"))?;

   let config_dir = home.join(".config").join("alf");
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

/// Resolve the home directory used for the config path and for expanding `~` and `$HOME`
///
/// `HOME` and `USERPROFILE` are consulted first, in that order, so a caller that overrides the
/// environment is honoured on every platform. An empty value is skipped rather than accepted, which
/// would otherwise yield a relative config path, and `var_os` is used so a home path that is not
/// valid Unicode — legal on Unix — still resolves instead of reading as unset.
///
/// `dirs::home_dir` is only the fallback. On Windows it reads the profile known folder and ignores
/// both variables, so relying on it alone would expand `~` to the real user profile even when the
/// environment points somewhere else — which silently defeats an isolated test home.
pub fn resolve_home_dir() -> Option<PathBuf> {
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
   let content = fs::read_to_string(&path).with_context(|| format!("Failed to read config at {}", path.display()))?;
   let config: Config =
      toml::from_str(&content).with_context(|| format!("Failed to parse config at {}", path.display()))?;
   Ok(config)
}

/// Load configuration from disk, returning `None` when no config file exists
///
/// # Errors
/// Returns an error when a config file exists but cannot be read or parsed, so a broken config is
/// never mistaken for a missing one. The check does not follow symlinks, so a dangling config
/// symlink is reported as a broken config rather than treated as a first run.
pub fn load_optional_config() -> Result<Option<Config>> {
   let path = get_config_path()?;

   if fs::symlink_metadata(&path).is_err_and(|error| error.kind() == std::io::ErrorKind::NotFound) {
      return Ok(None);
   }

   load_config().map(Some)
}

/// Save configuration to disk
///
/// The new contents are written to a sibling temporary file and then renamed over the target, so a
/// failure part-way through leaves the previous configuration intact rather than a truncated file.
/// The temporary name carries the process id to keep concurrent writers from sharing it. A symlinked
/// config file is resolved first, so the rename updates its target instead of replacing the link.
pub fn save_config(config: &Config) -> Result<()> {
   let path = get_config_path()?;

   if let Some(parent) = path.parent() {
      fs::create_dir_all(parent)?;
   }

   let path = fs::canonicalize(&path).unwrap_or(path);
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
