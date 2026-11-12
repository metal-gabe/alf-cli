//! Tests for configuration loading, saving, and defaults

use super::{
   expand_path, get_config_lock_path, get_config_path, is_first_run, load_config, load_optional_config, save_config,
   AliasExpansion, CaseMatching, Config, ConfigLock, GeneralConfig, ParseComments,
};
use crate::test_support::TempHome;
use std::fs;
use std::path::PathBuf;

// ===== Default value tests =====

#[test]
fn config_defaults() {
   let config = Config::default();
   assert_eq!(config.ui.theme, "default");
   assert_eq!(config.ui.keybind_mode, "vim");
   assert!(matches!(config.search.case_matching, CaseMatching::Smart));
   assert!(config.general.shell_files.is_empty());
   let enabled_flags = [
      ("should_normalize", config.search.should_normalize),
      ("is_regex_enabled", config.search.is_regex_enabled),
      ("is_substring_matching", config.search.is_substring_matching),
      ("should_show_type_badges", config.display.should_show_type_badges),
      ("is_syntax_highlighting_enabled", config.display.is_syntax_highlighting_enabled),
   ];
   for (name, is_enabled) in enabled_flags {
      assert!(is_enabled, "{name} should default to true");
   }
   assert_eq!(config.display.parse_comments, ParseComments::Always);
}

// ===== TOML serialization tests =====

#[test]
fn config_default_toml_format() {
   let toml_str = toml::to_string_pretty(&Config::default()).expect("Should serialize");
   insta::assert_snapshot!(toml_str);
}

#[test]
fn config_parse_valid_toml_with_custom_values() {
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
   assert!(!config.display.should_show_type_badges);
   assert!(!config.display.is_syntax_highlighting_enabled);
   assert!(!config.search.is_regex_enabled);
   assert!(!config.search.should_normalize);
   assert!(matches!(config.search.case_matching, CaseMatching::Respect));
   assert_eq!(config.display.parse_comments, ParseComments::Never);
}

#[test]
fn config_parse_comments_accepts_each_value_and_writes_it_back() {
   let cases = [
      ("true", ParseComments::Always, true, [true, true]),
      ("\"dynamic\"", ParseComments::Dynamic, true, [false, true]),
      ("false", ParseComments::Never, false, [false, false]),
   ];
   for (raw, expected, is_parsing_enabled, visibility) in cases {
      let content = format!("[display]\nparse_comments = {raw}\n");
      let config: Config = toml::from_str(&content).expect("Should parse parse_comments");
      let setting = config.display.parse_comments;
      assert_eq!(setting, expected, "{raw} should parse to {expected:?}");
      assert_eq!(setting.is_parsing_enabled(), is_parsing_enabled, "{raw} parsing");
      assert_eq!(
         [setting.is_description_visible(false), setting.is_description_visible(true)],
         visibility,
         "{raw} description visibility without and with comments"
      );
      let written = toml::to_string_pretty(&config).expect("Should serialize");
      assert!(written.contains(&format!("parse_comments = {raw}")), "{raw} should be written back unchanged");
   }
}

#[test]
fn parse_comments_next_cycles_dynamic_true_false() {
   let cases = [
      (ParseComments::Dynamic, ParseComments::Always),
      (ParseComments::Always, ParseComments::Never),
      (ParseComments::Never, ParseComments::Dynamic),
   ];
   for (current, expected) in cases {
      assert_eq!(current.next(), expected, "{current:?} should cycle to {expected:?}");
   }
}

#[test]
fn config_parse_comments_rejects_unknown_values() {
   for raw in ["\"always\"", "\"\"", "1"] {
      let content = format!("[display]\nparse_comments = {raw}\n");
      assert!(toml::from_str::<Config>(&content).is_err(), "{raw} should be rejected");
   }
}

#[test]
fn config_parse_invalid_toml_returns_error() {
   let invalid_toml = "this is not valid [[[ toml !!!";
   let result: Result<Config, _> = toml::from_str(invalid_toml);
   assert!(result.is_err(), "Invalid TOML should fail to parse");
}

// ===== AliasExpansion tests =====

#[test]
fn config_default_alias_expansion_is_name() {
   let config = Config::default();
   assert!(matches!(config.general.alias_expansion, AliasExpansion::Name));
}

#[test]
fn config_script_expansion_toml_format() {
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
fn config_parse_toml_with_alias_expansion_script() {
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
fn get_config_path_contains_alf_segment() {
   let path = get_config_path().expect("Should succeed when HOME is set");
   let path_str = path.to_str().unwrap();
   assert!(path_str.contains("alf"), "Path should contain 'alf': {}", path_str);
}

#[test]
fn get_config_path_ends_with_config_toml() {
   let path = get_config_path().expect("Should succeed when HOME is set");
   let path_str = path.to_str().unwrap();
   assert!(path_str.ends_with("config.toml"), "Path should end with config.toml: {}", path_str);
}

// ===== Home resolution tests =====

#[test]
fn get_config_path_falls_back_to_userprofile_when_home_is_empty() {
   let mut home = TempHome::new();
   let profile = home.path().join("profile");
   home.set_env("USERPROFILE", &profile);
   home.set_env("HOME", "");
   let path = get_config_path().expect("Should resolve a home from USERPROFILE");
   assert!(path.is_absolute(), "An empty HOME should not produce a relative config path, got {}", path.display());
   assert_eq!(path, profile.join(".config").join("alf").join("config.toml"));
   assert_eq!(expand_path("~"), profile, "Both resolvers should agree on the fallback home");
}

#[test]
fn get_config_path_falls_back_when_home_is_unset() {
   let mut home = TempHome::new();
   let profile = home.path().join("profile");
   home.set_env("USERPROFILE", &profile);
   home.unset_env("HOME");
   assert_eq!(
      get_config_path().expect("Should resolve a home"),
      profile.join(".config").join("alf").join("config.toml")
   );
}

#[cfg(unix)]
#[test]
fn home_resolution_accepts_non_unicode_values() {
   use std::ffi::OsString;
   use std::os::unix::ffi::OsStringExt;

   let mut home = TempHome::new();
   let mut bytes = home.path().into_os_string().into_vec();
   bytes.extend_from_slice(b"/home\xff");
   let non_unicode = PathBuf::from(OsString::from_vec(bytes));
   home.set_env("HOME", &non_unicode);
   assert_eq!(
      get_config_path().expect("A non-Unicode HOME should still resolve"),
      non_unicode.join(".config").join("alf").join("config.toml")
   );
   assert_eq!(expand_path("~"), non_unicode, "Both resolvers should accept a non-Unicode home");
}

#[cfg(unix)]
#[test]
fn get_config_path_errors_when_no_home_can_be_resolved() {
   use std::ffi::OsString;
   use std::os::unix::ffi::OsStringExt;

   let mut home = TempHome::new();
   home.set_env("HOME", "");
   home.set_env("USERPROFILE", OsString::from_vec(Vec::new()));
   let resolved = get_config_path();
   match dirs::home_dir() {
      Some(fallback) => {
         assert_eq!(
            resolved.expect("Should use the platform fallback"),
            fallback.join(".config").join("alf").join("config.toml")
         );
      },
      None => {
         assert!(resolved.is_err(), "With no home anywhere, the config path should be an error");
      },
   }
}

// ===== Path expansion tests =====

#[test]
fn expand_path_resolves_home_prefixes() {
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
fn expand_path_uses_the_same_home_as_the_config_path() {
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
fn expand_path_passes_through_paths_without_a_home_prefix() {
   let _home = TempHome::new();

   for raw_path in ["/etc/shells", "relative/path"] {
      assert_eq!(expand_path(raw_path), PathBuf::from(raw_path), "Unexpected expansion for {raw_path}");
   }
}

#[test]
fn config_partial_toml_fills_missing_values_with_defaults() {
   let config: Config =
      toml::from_str("[general]\nshell_files = [\"~/.zshrc\"]\n").expect("Should parse only [general]");
   assert_eq!(config.general.shell_files, vec!["~/.zshrc".to_string()]);
   assert_eq!(config.ui.theme, "default");
   assert_eq!(config.display.parse_comments, ParseComments::Always);
   let config: Config = toml::from_str("[search]\nnormalize = false\n").expect("Should parse a partial section");
   assert!(!config.search.should_normalize);
   assert!(config.search.is_regex_enabled);
}

#[test]
fn config_renamed_fields_keep_their_toml_keys() {
   let keys =
      ["normalize", "enable_regex", "substring_matching", "show_type_badges", "syntax_highlighting", "parse_comments"];
   let content = keys
      .iter()
      .enumerate()
      .map(|(idx, key)| {
         format!(
            "{}{key} = false\n",
            if idx == 0 {
               "[search]\n"
            } else if idx == 3 {
               "[display]\n"
            } else {
               ""
            }
         )
      })
      .collect::<String>();
   let config: Config = toml::from_str(&content).expect("Should parse the original keys");
   let flags = [
      config.search.should_normalize,
      config.search.is_regex_enabled,
      config.search.is_substring_matching,
      config.display.should_show_type_badges,
      config.display.is_syntax_highlighting_enabled,
   ];
   assert!(flags.iter().all(|is_enabled| !is_enabled), "Every original key should set its field");
   assert_eq!(config.display.parse_comments, ParseComments::Never);
   let written = toml::to_string_pretty(&config).expect("Should serialize");
   for key in keys {
      assert!(written.contains(&format!("{key} = false")), "{key} should be written back unchanged");
   }
}

// ===== File I/O tests =====

#[test]
fn save_and_load_config_roundtrip() {
   let _home = TempHome::new();
   let mut config = Config::default();
   config.ui.theme = "gruvbox".to_string();
   config.display.is_syntax_highlighting_enabled = false;
   save_config(&config).expect("Should save config");
   let loaded = load_config().expect("Should load saved config");
   assert_eq!(loaded.ui.theme, "gruvbox");
   assert!(!loaded.display.is_syntax_highlighting_enabled);
}

#[test]
fn load_config_fails_when_missing() {
   let _home = TempHome::new();
   let result = load_config();
   assert!(result.is_err(), "Should fail when config file does not exist");
}

#[test]
fn load_optional_config_distinguishes_missing_valid_and_broken() {
   let _home = TempHome::new();
   assert!(load_optional_config().expect("A missing config is not an error").is_none());
   save_config(&Config::default()).expect("Should save config");
   assert!(load_optional_config().expect("A valid config should load").is_some());
   let path = get_config_path().expect("Should resolve config path");
   fs::write(&path, "this is not valid [[[ toml !!!").expect("Should write broken config");
   let error = load_optional_config().expect_err("A broken config should be an error");
   assert!(format!("{error:#}").contains(&path.display().to_string()), "The error should name the file");
}

#[test]
fn is_first_run_returns_true_when_no_config() {
   let _home = TempHome::new();
   let result = is_first_run().expect("Should succeed");
   assert!(result, "Should be first run when no config file exists");
}

#[test]
fn is_first_run_returns_false_after_save() {
   let _home = TempHome::new();
   save_config(&Config::default()).expect("Should save config");
   let result = is_first_run().expect("Should succeed");
   assert!(!result, "Should not be first run after config is saved");
}

#[test]
fn save_config_leaves_no_temporary_files_behind() {
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
fn config_lock_path_sits_beside_the_config_file() {
   let _home = TempHome::new();
   let config_path = get_config_path().expect("Should build config path");
   let lock_path = get_config_lock_path().expect("Should build lock path");
   assert_eq!(lock_path.parent(), config_path.parent(), "The lock should live in the config directory");
   assert_eq!(lock_path.file_name().expect("Should have a file name"), "config.toml.lock");
}

#[test]
fn config_lock_is_created_before_any_config_exists() {
   let _home = TempHome::new();
   let lock = ConfigLock::acquire().expect("Should acquire the lock without a config file");
   assert!(get_config_lock_path().expect("Should build lock path").exists(), "Acquiring should create the lock file");
   drop(lock);
   assert!(is_first_run().expect("Should succeed"), "The lock file alone should not count as a config");
}

#[test]
fn config_lock_can_be_reacquired_after_each_release() {
   let _home = TempHome::new();
   for attempt in 1..=3 {
      let lock = ConfigLock::acquire().unwrap_or_else(|_| panic!("Should acquire the lock on attempt {attempt}"));
      drop(lock);
   }
}
