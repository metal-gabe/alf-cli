//! Tests for configuration loading, saving, and defaults

use super::{
   expand_path, get_config_lock_path, get_config_path, is_first_run, load_config, save_config, AliasExpansion,
   CaseMatching, Config, ConfigLock, GeneralConfig,
};
use crate::test_support::TempHome;
use std::fs;
use std::path::PathBuf;

// ===== Default value tests =====

#[test]
fn test_config_default_theme() {
   let config = Config::default();
   assert_eq!(config.ui.theme, "default");
}

#[test]
fn test_config_default_keybind_mode() {
   let config = Config::default();
   assert_eq!(config.ui.keybind_mode, "vim");
}

#[test]
fn test_config_default_case_matching_is_smart() {
   let config = Config::default();
   assert!(matches!(config.search.case_matching, CaseMatching::Smart));
}

#[test]
fn test_config_default_normalize_is_true() {
   let config = Config::default();
   assert!(config.search.normalize);
}

#[test]
fn test_config_default_enable_regex_is_true() {
   let config = Config::default();
   assert!(config.search.enable_regex);
}

#[test]
fn test_config_default_substring_matching_is_true() {
   let config = Config::default();
   assert!(config.search.substring_matching);
}

#[test]
fn test_config_default_show_type_badges() {
   let config = Config::default();
   assert!(config.display.show_type_badges);
}

#[test]
fn test_config_default_syntax_highlighting() {
   let config = Config::default();
   assert!(config.display.syntax_highlighting);
}

#[test]
fn test_config_default_parse_comments() {
   let config = Config::default();
   assert!(config.display.parse_comments);
}

#[test]
fn test_config_default_shell_files_empty() {
   let config = Config::default();
   assert!(config.general.shell_files.is_empty());
}

// ===== TOML serialization tests =====

#[test]
fn test_config_default_toml_format() {
   let toml_str = toml::to_string_pretty(&Config::default()).expect("Should serialize");
   insta::assert_snapshot!(toml_str);
}

#[test]
fn test_config_parse_valid_toml_with_custom_values() {
   let toml_content = r#"
[general]
shell_files = ["~/.bashrc", "~/.zshrc"]

[search]
case_matching = "respect"
normalize = false
enable_regex = false
substring_matching = false

[ui]
theme = "dracula"
keybind_mode = "vim"

[display]
show_type_badges = false
syntax_highlighting = false
parse_comments = false
"#;
   let config: Config = toml::from_str(toml_content).expect("Should parse valid TOML");
   assert_eq!(config.ui.theme, "dracula");
   assert_eq!(config.general.shell_files, vec!["~/.bashrc", "~/.zshrc"]);
   assert!(!config.display.show_type_badges);
   assert!(!config.display.syntax_highlighting);
   assert!(!config.search.enable_regex);
   assert!(!config.search.normalize);
   assert!(matches!(config.search.case_matching, CaseMatching::Respect));
}

#[test]
fn test_config_parse_invalid_toml_returns_error() {
   let invalid_toml = "this is not valid [[[ toml !!!";
   let result: Result<Config, _> = toml::from_str(invalid_toml);
   assert!(result.is_err(), "Invalid TOML should fail to parse");
}

// ===== AliasExpansion tests =====

#[test]
fn test_config_default_alias_expansion_is_name() {
   let config = Config::default();
   assert!(matches!(config.general.alias_expansion, AliasExpansion::Name));
}

#[test]
fn test_config_script_expansion_toml_format() {
   let config = Config {
      general: GeneralConfig {
         alias_expansion: AliasExpansion::Script,
         ..Default::default()
      },
      ..Config::default()
   };
   let toml_str = toml::to_string_pretty(&config).expect("Should serialize");
   insta::assert_snapshot!(toml_str);
}

#[test]
fn test_config_parse_toml_with_alias_expansion_script() {
   let toml_content = r#"
[general]
alias_expansion = "script"

[search]
case_matching = "smart"
normalize = true
enable_regex = true
substring_matching = true

[ui]
theme = "default"
keybind_mode = "vim"

[display]
show_type_badges = true
syntax_highlighting = true
parse_comments = true
"#;
   let config: Config = toml::from_str(toml_content).expect("Should parse");
   assert!(matches!(config.general.alias_expansion, AliasExpansion::Script));
}

// ===== File path tests =====

#[test]
fn test_get_config_path_contains_alf_segment() {
   let path = get_config_path().expect("Should succeed when HOME is set");
   let path_str = path.to_str().unwrap();
   assert!(path_str.contains("alf"), "Path should contain 'alf': {}", path_str);
}

#[test]
fn test_get_config_path_ends_with_config_toml() {
   let path = get_config_path().expect("Should succeed when HOME is set");
   let path_str = path.to_str().unwrap();
   assert!(path_str.ends_with("config.toml"), "Path should end with config.toml: {}", path_str);
}

// ===== Path expansion tests =====

#[test]
fn test_expand_path_resolves_home_prefixes() {
   let home = TempHome::new();

   for (raw_path, expected) in [
      ("~/foo/bar", home.path().join("foo/bar")),
      ("$HOME/foo/bar", home.path().join("foo/bar")),
      ("~", home.path()),
      ("$HOME", home.path()),
   ] {
      assert_eq!(expand_path(raw_path), expected, "Unexpected expansion for {raw_path}");
   }
}

#[test]
fn test_expand_path_uses_the_same_home_as_the_config_path() {
   let home = TempHome::new();
   let config_path = get_config_path().expect("Should build config path");
   assert!(
      config_path.starts_with(expand_path("~")),
      "`expand_path` and `get_config_path` should agree on home, got {} vs {}",
      expand_path("~").display(),
      config_path.display()
   );
   assert_eq!(expand_path("~"), home.path(), "Home should come from the environment, not the platform default");
}

#[test]
fn test_expand_path_passes_through_paths_without_a_home_prefix() {
   let _home = TempHome::new();

   for raw_path in ["/etc/shells", "relative/path"] {
      assert_eq!(expand_path(raw_path), PathBuf::from(raw_path), "Unexpected expansion for {raw_path}");
   }
}

// ===== File I/O tests =====

#[test]
fn test_save_and_load_config_roundtrip() {
   let _home = TempHome::new();
   let mut config = Config::default();
   config.ui.theme = "gruvbox".to_string();
   config.display.syntax_highlighting = false;
   save_config(&config).expect("Should save config");
   let loaded = load_config().expect("Should load saved config");
   assert_eq!(loaded.ui.theme, "gruvbox");
   assert!(!loaded.display.syntax_highlighting);
}

#[test]
fn test_load_config_fails_when_missing() {
   let _home = TempHome::new();
   let result = load_config();
   assert!(result.is_err(), "Should fail when config file does not exist");
}

#[test]
fn test_is_first_run_returns_true_when_no_config() {
   let _home = TempHome::new();
   let result = is_first_run().expect("Should succeed");
   assert!(result, "Should be first run when no config file exists");
}

#[test]
fn test_is_first_run_returns_false_after_save() {
   let _home = TempHome::new();
   save_config(&Config::default()).expect("Should save config");
   let result = is_first_run().expect("Should succeed");
   assert!(!result, "Should not be first run after config is saved");
}

#[test]
fn test_save_config_leaves_no_temporary_files_behind() {
   let _home = TempHome::new();
   save_config(&Config::default()).expect("Should save config");
   let config_dir =
      get_config_path().expect("Should build config path").parent().expect("Should have a parent").to_path_buf();
   let leftovers: Vec<String> = fs::read_dir(&config_dir)
      .expect("Should read config dir")
      .filter_map(|entry| entry.ok())
      .map(|entry| entry.file_name().to_string_lossy().to_string())
      .filter(|name| name.ends_with(".tmp"))
      .collect();
   assert!(leftovers.is_empty(), "A completed save should leave no temp files, found {leftovers:?}");
}

// ===== Config lock tests =====

#[test]
fn test_config_lock_path_sits_beside_the_config_file() {
   let _home = TempHome::new();
   let config_path = get_config_path().expect("Should build config path");
   let lock_path = get_config_lock_path().expect("Should build lock path");
   assert_eq!(lock_path.parent(), config_path.parent(), "The lock should live in the config directory");
   assert_eq!(lock_path.file_name().expect("Should have a file name"), "config.toml.lock");
}

#[test]
fn test_config_lock_is_created_before_any_config_exists() {
   let _home = TempHome::new();
   let lock = ConfigLock::acquire().expect("Should acquire the lock without a config file");
   assert!(get_config_lock_path().expect("Should build lock path").exists(), "Acquiring should create the lock file");
   drop(lock);
   assert!(is_first_run().expect("Should succeed"), "The lock file alone should not count as a config");
}

#[test]
fn test_config_lock_can_be_reacquired_after_each_release() {
   let _home = TempHome::new();
   for attempt in 1..=3 {
      let lock = ConfigLock::acquire().unwrap_or_else(|_| panic!("Should acquire the lock on attempt {attempt}"));
      drop(lock);
   }
}
