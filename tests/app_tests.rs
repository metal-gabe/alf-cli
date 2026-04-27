//! Integration tests for the App state manager

use alf::models::{AliasEntry, EntryType};
use alf::tui::app::{App, ExitAction};
use alf::tui::state::{EntryFilter, GroupMode, InputMode, Panel, SortOrder};
use alf::tui::Theme;
use std::path::PathBuf;

fn make_entry(name: &str, entry_type: EntryType, value: &str) -> AliasEntry {
    AliasEntry {
        name: name.to_string(),
        entry_type,
        value: value.to_string(),
        comments: None,
        source_file: PathBuf::from("test.sh"),
    }
}

/// Creates an App with 2 aliases and 2 functions.
/// Default ordering (GroupMode::Aliases first, A-Z):
///   alpha (Alias), beta (Alias), gamma (Function), delta (Function)
/// Wait - sorted A-Z within groups: alpha, beta for aliases; delta, gamma for functions
fn make_app() -> App {
    let entries = vec![
        make_entry("beta", EntryType::Alias, "cmd_beta"),
        make_entry("alpha", EntryType::Alias, "cmd_alpha"),
        make_entry("gamma", EntryType::Function, "{ body_gamma }"),
        make_entry("delta", EntryType::Function, "{ body_delta }"),
    ];
    App::new(entries, Theme::default())
}

fn make_empty_app() -> App {
    App::new(vec![], Theme::default())
}

// ===== Initial state =====

#[test]
fn test_new_app_selected_index_is_zero() {
    let app = make_app();
    assert_eq!(app.selected_index(), 0);
}

#[test]
fn test_new_app_should_not_quit() {
    let app = make_app();
    assert!(!app.should_quit);
}

#[test]
fn test_new_app_exit_action_is_none() {
    let app = make_app();
    assert!(app.exit_action.is_none());
}

#[test]
fn test_new_app_input_mode_is_normal() {
    let app = make_app();
    assert_eq!(app.input_mode(), InputMode::Normal);
}

#[test]
fn test_new_app_all_entries_visible() {
    let app = make_app();
    assert_eq!(app.visible_indices().len(), 4);
}

#[test]
fn test_new_app_search_query_is_empty() {
    let app = make_app();
    assert_eq!(app.search_query(), "");
}

// ===== Navigation =====

#[test]
fn test_move_down_increments_selection() {
    let mut app = make_app();
    app.move_down();
    assert_eq!(app.selected_index(), 1);
}

#[test]
fn test_move_up_decrements_selection() {
    let mut app = make_app();
    app.move_down();
    app.move_down();
    app.move_up();
    assert_eq!(app.selected_index(), 1);
}

#[test]
fn test_move_up_at_top_stays_at_zero() {
    let mut app = make_app();
    app.move_up();
    assert_eq!(app.selected_index(), 0);
}

#[test]
fn test_move_down_at_last_stays_at_last() {
    let mut app = make_app();
    for _ in 0..10 {
        app.move_down();
    }
    assert_eq!(app.selected_index(), 3, "Should not exceed visible_count - 1");
}

#[test]
fn test_move_top_jumps_to_zero() {
    let mut app = make_app();
    app.move_down();
    app.move_down();
    app.move_top();
    assert_eq!(app.selected_index(), 0);
}

#[test]
fn test_move_bottom_jumps_to_last() {
    let mut app = make_app();
    app.move_bottom();
    assert_eq!(app.selected_index(), 3);
}

#[test]
fn test_move_down_on_empty_app_no_panic() {
    let mut app = make_empty_app();
    app.move_down();
    assert_eq!(app.selected_index(), 0);
}

#[test]
fn test_move_bottom_on_empty_app_no_panic() {
    let mut app = make_empty_app();
    app.move_bottom();
    assert_eq!(app.selected_index(), 0);
}

// ===== Scroll =====

#[test]
fn test_scroll_down_on_list_panel_jumps_selection() {
    let mut app = make_app();
    assert_eq!(app.active_panel(), Panel::List);
    app.scroll_down(2);
    assert_eq!(app.selected_index(), 2);
}

#[test]
fn test_scroll_up_on_list_panel_saturates_at_zero() {
    let mut app = make_app();
    app.scroll_up(100);
    assert_eq!(app.selected_index(), 0);
}

// ===== Panel cycling =====

#[test]
fn test_cycle_panel_forward_three_times_returns_to_list() {
    let mut app = make_app();
    app.cycle_panel();
    app.cycle_panel();
    app.cycle_panel();
    assert_eq!(app.active_panel(), Panel::List);
}

#[test]
fn test_cycle_panel_backward_three_times_returns_to_list() {
    let mut app = make_app();
    app.cycle_panel_backward();
    app.cycle_panel_backward();
    app.cycle_panel_backward();
    assert_eq!(app.active_panel(), Panel::List);
}

#[test]
fn test_cycle_panel_forward_sequence() {
    let mut app = make_app();
    assert_eq!(app.active_panel(), Panel::List);
    app.cycle_panel();
    assert_eq!(app.active_panel(), Panel::Description);
    app.cycle_panel();
    assert_eq!(app.active_panel(), Panel::Script);
}

// ===== Filter =====

#[test]
fn test_set_filter_aliases_reduces_visible_count() {
    let mut app = make_app();
    app.set_filter(EntryFilter::Aliases);
    assert_eq!(app.filter(), EntryFilter::Aliases);
    assert_eq!(app.visible_indices().len(), 2);
}

#[test]
fn test_set_filter_functions_reduces_visible_count() {
    let mut app = make_app();
    app.set_filter(EntryFilter::Functions);
    assert_eq!(app.filter(), EntryFilter::Functions);
    assert_eq!(app.visible_indices().len(), 2);
}

#[test]
fn test_set_filter_all_restores_full_count() {
    let mut app = make_app();
    app.set_filter(EntryFilter::Aliases);
    app.set_filter(EntryFilter::All);
    assert_eq!(app.visible_indices().len(), 4);
}

#[test]
fn test_cycle_filter_forward_full_cycle() {
    let mut app = make_app();
    assert_eq!(app.filter(), EntryFilter::All);
    app.cycle_filter();
    assert_eq!(app.filter(), EntryFilter::Aliases);
    app.cycle_filter();
    assert_eq!(app.filter(), EntryFilter::Functions);
    app.cycle_filter();
    assert_eq!(app.filter(), EntryFilter::All);
}

#[test]
fn test_cycle_filter_backward_full_cycle() {
    let mut app = make_app();
    app.cycle_filter_backward();
    assert_eq!(app.filter(), EntryFilter::Functions);
    app.cycle_filter_backward();
    assert_eq!(app.filter(), EntryFilter::Aliases);
    app.cycle_filter_backward();
    assert_eq!(app.filter(), EntryFilter::All);
}

#[test]
fn test_filter_clamps_selection_when_count_shrinks() {
    let mut app = make_app();
    app.move_bottom(); // Select last (index 3)
    app.set_filter(EntryFilter::Aliases); // Now only 2 visible
    assert!(app.selected_index() < 2, "Selection should clamp to new visible count");
}

// ===== Search mode =====

#[test]
fn test_enter_search_mode_sets_search_input_mode() {
    let mut app = make_app();
    app.enter_search_mode();
    assert_eq!(app.input_mode(), InputMode::Search);
}

#[test]
fn test_exit_search_keep_query_returns_to_normal() {
    let mut app = make_app();
    app.enter_search_mode();
    app.search_insert_char('a');
    app.exit_search_keep_query();
    assert_eq!(app.input_mode(), InputMode::Normal);
    assert_eq!(app.search_query(), "a", "Query should be preserved");
}

#[test]
fn test_exit_search_clear_query_returns_to_normal_and_clears() {
    let mut app = make_app();
    app.enter_search_mode();
    app.search_insert_char('a');
    app.exit_search_clear_query();
    assert_eq!(app.input_mode(), InputMode::Normal);
    assert_eq!(app.search_query(), "", "Query should be cleared");
}

#[test]
fn test_search_insert_char_filters_visible_entries() {
    let mut app = make_app();
    app.set_search_query("alpha".to_string());
    assert_eq!(app.visible_indices().len(), 1);
}

#[test]
fn test_search_delete_char_updates_visible_entries() {
    let mut app = make_app();
    app.set_search_query("alpha".to_string());
    // Delete until query is empty
    for _ in 0.."alpha".len() {
        app.search_delete_char();
    }
    assert_eq!(app.search_query(), "");
    assert_eq!(app.visible_indices().len(), 4);
}

#[test]
fn test_set_search_query_filters_by_name() {
    let mut app = make_app();
    app.set_search_query("gamma".to_string());
    assert_eq!(app.visible_indices().len(), 1);
    assert_eq!(app.selected_entry().unwrap().name, "gamma");
}

#[test]
fn test_set_search_query_no_match_empties_visible() {
    let mut app = make_app();
    app.set_search_query("zzznomatch".to_string());
    assert_eq!(app.visible_indices().len(), 0);
}

#[test]
fn test_clear_search_restores_all_entries() {
    let mut app = make_app();
    app.set_search_query("alpha".to_string());
    app.clear_search();
    assert_eq!(app.search_query(), "");
    assert_eq!(app.visible_indices().len(), 4);
}

#[test]
fn test_search_cursor_left_and_right() {
    let mut app = make_app();
    app.set_search_query("abc".to_string());
    assert_eq!(app.cursor_position(), 3);
    app.search_cursor_left();
    assert_eq!(app.cursor_position(), 2);
    app.search_cursor_right();
    assert_eq!(app.cursor_position(), 3);
}

// ===== Help modal =====

#[test]
fn test_toggle_help_twice_hides_modal() {
    let mut app = make_app();
    assert!(!app.show_help());
    app.toggle_help();
    assert!(app.show_help());
    app.toggle_help();
    assert!(!app.show_help());
}

#[test]
fn test_help_scroll_operations_with_max_set() {
    let mut app = make_app();
    app.update_help_max_scroll(30, 10); // max = 20
    app.help_jump_bottom();
    assert_eq!(app.help_scroll_offset(), 20);
    app.help_jump_top();
    assert_eq!(app.help_scroll_offset(), 0);
}

#[test]
fn test_help_scroll_down_increments() {
    let mut app = make_app();
    app.update_help_max_scroll(20, 5); // max = 15
    app.help_scroll_down();
    assert_eq!(app.help_scroll_offset(), 1);
}

#[test]
fn test_help_scroll_up_decrements() {
    let mut app = make_app();
    app.update_help_max_scroll(20, 5);
    app.help_scroll_down();
    app.help_scroll_down();
    app.help_scroll_up();
    assert_eq!(app.help_scroll_offset(), 1);
}

// ===== Group mode and sort order =====

#[test]
fn test_cycle_group_mode_changes_group() {
    let mut app = make_app();
    assert_eq!(app.group_mode(), GroupMode::Aliases);
    app.cycle_group_mode();
    assert_eq!(app.group_mode(), GroupMode::Functions);
}

#[test]
fn test_cycle_group_mode_backward_changes_group() {
    let mut app = make_app();
    app.cycle_group_mode_backward();
    assert_eq!(app.group_mode(), GroupMode::None);
}

#[test]
fn test_toggle_sort_order_changes_order() {
    let mut app = make_app();
    assert_eq!(app.sort_order(), SortOrder::Ascending);
    app.toggle_sort_order();
    assert_eq!(app.sort_order(), SortOrder::Descending);
}

#[test]
fn test_toggle_sort_order_reverses_visible_entry_order() {
    let mut app = make_app();
    // With GroupMode::Aliases (default), Ascending: alpha, beta, delta, gamma
    // After toggle to Descending: beta, alpha, gamma, delta
    app.set_filter(EntryFilter::Aliases); // Only aliases: alpha, beta
    let first_asc = app.selected_entry().unwrap().name.clone();
    app.toggle_sort_order();
    let first_desc = app.selected_entry().unwrap().name.clone();
    assert_ne!(first_asc, first_desc, "Toggling sort order should change first entry");
}

// ===== Select entry =====

#[test]
fn test_select_entry_execute_sets_quit_and_action() {
    let mut app = make_app();
    app.select_entry(ExitAction::Execute);
    assert!(app.should_quit);
    assert!(matches!(app.exit_action, Some(ExitAction::Execute)));
}

#[test]
fn test_select_entry_populate_sets_quit_and_action() {
    let mut app = make_app();
    app.select_entry(ExitAction::Populate);
    assert!(app.should_quit);
    assert!(matches!(app.exit_action, Some(ExitAction::Populate)));
}

#[test]
fn test_select_entry_on_empty_app_sets_quit_but_no_action() {
    let mut app = make_empty_app();
    app.select_entry(ExitAction::Execute);
    assert!(app.should_quit, "should_quit should still be set");
    assert!(app.exit_action.is_none(), "exit_action should be None when no entry selected");
}

// ===== Selected entry =====

#[test]
fn test_selected_entry_returns_first_visible_by_default() {
    let app = make_app();
    // Default: GroupMode::Aliases + Ascending → first is "alpha"
    assert_eq!(app.selected_entry().unwrap().name, "alpha");
}

#[test]
fn test_selected_entry_returns_none_when_empty() {
    let app = make_empty_app();
    assert!(app.selected_entry().is_none());
}

// ===== Tick =====

#[test]
fn test_tick_with_no_pending_key_no_panic() {
    let mut app = make_app();
    app.tick(); // Should not panic
}

#[test]
fn test_tick_does_not_alter_state_when_key_fresh() {
    let mut app = make_app();
    app.set_pending_key('g');
    app.tick();
    // Key was just set, should not be expired
    assert!(app.pending_key().is_some());
}

// ===== Pending key =====

#[test]
fn test_set_and_clear_pending_key() {
    let mut app = make_app();
    app.set_pending_key('g');
    assert_eq!(app.pending_key(), Some('g'));
    app.clear_pending_key();
    assert!(app.pending_key().is_none());
}

// ===== Theme cycling =====

#[test]
fn test_cycle_theme_next_changes_theme_name() {
    let mut app = make_app();
    let initial_name = app.theme().name.clone();
    app.cycle_theme_next();
    assert_ne!(app.theme().name, initial_name, "Theme should change after cycling next");
}

#[test]
fn test_cycle_theme_prev_changes_theme_name() {
    let mut app = make_app();
    let initial_name = app.theme().name.clone();
    app.cycle_theme_prev();
    assert_ne!(app.theme().name, initial_name, "Theme should change after cycling prev");
}

#[test]
fn test_cycle_theme_next_full_cycle_returns_to_start() {
    let mut app = make_app();
    let initial_name = app.theme().name.clone();
    let theme_count = Theme::available_themes().len();
    for _ in 0..theme_count {
        app.cycle_theme_next();
    }
    assert_eq!(app.theme().name, initial_name, "Full cycle should return to starting theme");
}

#[test]
fn test_cycle_theme_prev_full_cycle_returns_to_start() {
    let mut app = make_app();
    let initial_name = app.theme().name.clone();
    let theme_count = Theme::available_themes().len();
    for _ in 0..theme_count {
        app.cycle_theme_prev();
    }
    assert_eq!(app.theme().name, initial_name, "Full backward cycle should return to starting theme");
}

#[test]
fn test_cycle_theme_next_then_prev_returns_to_original() {
    let mut app = make_app();
    let initial_name = app.theme().name.clone();
    app.cycle_theme_next();
    app.cycle_theme_prev();
    assert_eq!(app.theme().name, initial_name);
}
