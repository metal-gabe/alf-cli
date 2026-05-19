//! Tests for configuration loading, saving, and defaults

use super::{get_config_path, is_first_run, load_config, save_config, AliasExpansion, CaseMatching, Config, GeneralConfig};

static ENV_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());

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
        general: GeneralConfig { alias_expansion: AliasExpansion::Script, ..Default::default() },
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

// ===== File I/O tests =====

#[test]
fn test_save_and_load_config_roundtrip() {
    let _guard = ENV_MUTEX.lock().unwrap();
    use std::env;
    use tempfile::TempDir;
    let temp_dir = TempDir::new().expect("Should create temp dir");
    let old_home = env::var("HOME").ok();
    env::set_var("HOME", temp_dir.path());
    let mut config = Config::default();
    config.ui.theme = "gruvbox".to_string();
    config.display.syntax_highlighting = false;
    save_config(&config).expect("Should save config");
    let loaded = load_config().expect("Should load saved config");
    assert_eq!(loaded.ui.theme, "gruvbox");
    assert!(!loaded.display.syntax_highlighting);
    if let Some(home) = old_home {
        env::set_var("HOME", home);
    } else {
        env::remove_var("HOME");
    }
}

#[test]
fn test_load_config_fails_when_missing() {
    let _guard = ENV_MUTEX.lock().unwrap();
    use std::env;
    use tempfile::TempDir;
    let temp_dir = TempDir::new().expect("Should create temp dir");
    let old_home = env::var("HOME").ok();
    env::set_var("HOME", temp_dir.path());
    let result = load_config();
    assert!(result.is_err(), "Should fail when config file does not exist");
    if let Some(home) = old_home {
        env::set_var("HOME", home);
    } else {
        env::remove_var("HOME");
    }
}

#[test]
fn test_is_first_run_returns_true_when_no_config() {
    let _guard = ENV_MUTEX.lock().unwrap();
    use std::env;
    use tempfile::TempDir;
    let temp_dir = TempDir::new().expect("Should create temp dir");
    let old_home = env::var("HOME").ok();
    env::set_var("HOME", temp_dir.path());
    let result = is_first_run().expect("Should succeed");
    assert!(result, "Should be first run when no config file exists");
    if let Some(home) = old_home {
        env::set_var("HOME", home);
    } else {
        env::remove_var("HOME");
    }
}

#[test]
fn test_is_first_run_returns_false_after_save() {
    let _guard = ENV_MUTEX.lock().unwrap();
    use std::env;
    use tempfile::TempDir;
    let temp_dir = TempDir::new().expect("Should create temp dir");
    let old_home = env::var("HOME").ok();
    env::set_var("HOME", temp_dir.path());
    save_config(&Config::default()).expect("Should save config");
    let result = is_first_run().expect("Should succeed");
    assert!(!result, "Should not be first run after config is saved");
    if let Some(home) = old_home {
        env::set_var("HOME", home);
    } else {
        env::remove_var("HOME");
    }
}
