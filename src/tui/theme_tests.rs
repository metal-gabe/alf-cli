//! Tests for Theme loading and cycling

use super::Theme;

// ===== available_themes =====

#[test]
fn test_available_themes_returns_eight_themes() {
    let themes = Theme::available_themes();
    assert_eq!(themes.len(), 8);
}

#[test]
fn test_available_themes_are_alphabetically_sorted() {
    let themes = Theme::available_themes();
    let mut sorted = themes.clone();
    sorted.sort();
    assert_eq!(themes, sorted, "available_themes() should be in alphabetical order");
}

#[test]
fn test_available_themes_contains_expected_names() {
    let themes = Theme::available_themes();
    let expected = ["catppuccin", "default", "dracula", "gruvbox", "nord", "shades_of_purple", "solarized", "tokyonight"];
    for name in &expected {
        assert!(themes.contains(&name.to_string()), "Should contain theme '{}'", name);
    }
}

// ===== from_name =====

#[test]
fn test_from_name_returns_some_for_all_valid_names() {
    let names = Theme::available_themes();
    for name in &names {
        assert!(Theme::from_name(name).is_some(), "from_name('{}') should return Some", name);
    }
}

#[test]
fn test_from_name_returns_none_for_unknown_name() {
    assert!(Theme::from_name("unknown_theme_xyz").is_none());
}

#[test]
fn test_from_name_returns_none_for_empty_string() {
    assert!(Theme::from_name("").is_none());
}

#[test]
fn test_from_name_is_case_insensitive() {
    assert!(Theme::from_name("DRACULA").is_some());
    assert!(Theme::from_name("Gruvbox").is_some());
    assert!(Theme::from_name("NORD").is_some());
}

// ===== Theme properties =====

#[test]
fn test_each_theme_has_non_empty_name() {
    for theme_name in Theme::available_themes() {
        let theme = Theme::from_name(&theme_name).unwrap();
        assert!(!theme.name.is_empty(), "Theme '{}' should have non-empty name field", theme_name);
    }
}

#[test]
fn test_each_theme_has_non_empty_display_name() {
    for theme_name in Theme::available_themes() {
        let theme = Theme::from_name(&theme_name).unwrap();
        assert!(!theme.display_name.is_empty(), "Theme '{}' should have non-empty display_name", theme_name);
    }
}

#[test]
fn test_theme_name_field_matches_lookup_key() {
    for theme_name in Theme::available_themes() {
        let theme = Theme::from_name(&theme_name).unwrap();
        assert_eq!(theme.name, theme_name, "Theme name field should match the key used to look it up");
    }
}

// ===== Default theme =====

#[test]
fn test_default_theme_name_is_default() {
    let theme = Theme::default();
    assert_eq!(theme.name, "default");
}

#[test]
fn test_default_theme_matches_from_name_default() {
    let via_default = Theme::default();
    let via_from_name = Theme::from_name("default").unwrap();
    assert_eq!(via_default.name, via_from_name.name);
    assert_eq!(via_default.display_name, via_from_name.display_name);
}
