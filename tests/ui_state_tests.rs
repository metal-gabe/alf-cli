//! Tests for UiState and ScrollManager

use alf::tui::state::{NavigationState, Panel, ScrollManager, UiState};

// ===== UiState defaults =====

#[test]
fn test_ui_state_default_panel_is_list() {
    let ui = UiState::new();
    assert_eq!(ui.active_panel(), Panel::List);
}

#[test]
fn test_ui_state_default_all_offsets_zero() {
    let ui = UiState::new();
    assert_eq!(ui.list_scroll_offset(), 0);
    assert_eq!(ui.description_scroll_offset(), 0);
    assert_eq!(ui.script_scroll_offset(), 0);
    assert_eq!(ui.help_scroll_offset(), 0);
}

#[test]
fn test_ui_state_default_help_hidden() {
    let ui = UiState::new();
    assert!(!ui.show_help());
}

// ===== Panel cycling =====

#[test]
fn test_cycle_panel_forward_list_to_description() {
    let mut ui = UiState::new();
    ui.cycle_panel();
    assert_eq!(ui.active_panel(), Panel::Description);
}

#[test]
fn test_cycle_panel_forward_description_to_script() {
    let mut ui = UiState::new();
    ui.cycle_panel();
    ui.cycle_panel();
    assert_eq!(ui.active_panel(), Panel::Script);
}

#[test]
fn test_cycle_panel_forward_script_wraps_to_list() {
    let mut ui = UiState::new();
    ui.cycle_panel();
    ui.cycle_panel();
    ui.cycle_panel();
    assert_eq!(ui.active_panel(), Panel::List);
}

#[test]
fn test_cycle_panel_backward_list_to_script() {
    let mut ui = UiState::new();
    ui.cycle_panel_backward();
    assert_eq!(ui.active_panel(), Panel::Script);
}

#[test]
fn test_cycle_panel_backward_script_to_description() {
    let mut ui = UiState::new();
    ui.cycle_panel_backward();
    ui.cycle_panel_backward();
    assert_eq!(ui.active_panel(), Panel::Description);
}

#[test]
fn test_cycle_panel_backward_description_wraps_to_list() {
    let mut ui = UiState::new();
    ui.cycle_panel_backward();
    ui.cycle_panel_backward();
    ui.cycle_panel_backward();
    assert_eq!(ui.active_panel(), Panel::List);
}

// ===== Help modal =====

#[test]
fn test_toggle_help_opens_modal() {
    let mut ui = UiState::new();
    ui.toggle_help();
    assert!(ui.show_help());
}

#[test]
fn test_toggle_help_closes_modal() {
    let mut ui = UiState::new();
    ui.toggle_help();
    ui.toggle_help();
    assert!(!ui.show_help());
}

#[test]
fn test_toggle_help_resets_scroll_when_opening() {
    let mut ui = UiState::new();
    ui.update_help_max_scroll(20, 5);
    ui.set_help_scroll_offset(3);
    ui.toggle_help(); // open
    assert_eq!(ui.help_scroll_offset(), 0, "Opening help should reset scroll");
}

#[test]
fn test_toggle_help_does_not_reset_scroll_when_closing() {
    let mut ui = UiState::new();
    ui.toggle_help(); // open
    ui.update_help_max_scroll(20, 5);
    ui.set_help_scroll_offset(3);
    ui.toggle_help(); // close
    assert_eq!(ui.help_scroll_offset(), 3, "Closing help should preserve scroll offset");
}

// ===== reset_detail_scroll =====

#[test]
fn test_reset_detail_scroll_zeroes_description_offset() {
    let mut ui = UiState::new();
    ui.set_description_scroll_offset(5);
    ui.reset_detail_scroll();
    assert_eq!(ui.description_scroll_offset(), 0);
}

#[test]
fn test_reset_detail_scroll_zeroes_script_offset() {
    let mut ui = UiState::new();
    ui.set_script_scroll_offset(7);
    ui.reset_detail_scroll();
    assert_eq!(ui.script_scroll_offset(), 0);
}

// ===== Max scroll updates =====

#[test]
fn test_update_description_max_scroll_when_total_exceeds_visible() {
    let mut ui = UiState::new();
    ui.update_description_max_scroll(20, 10);
    assert_eq!(ui.description_max_scroll(), 10);
}

#[test]
fn test_update_description_max_scroll_when_total_equals_visible() {
    let mut ui = UiState::new();
    ui.update_description_max_scroll(10, 10);
    assert_eq!(ui.description_max_scroll(), 0);
}

#[test]
fn test_update_description_max_scroll_when_total_less_than_visible() {
    let mut ui = UiState::new();
    ui.update_description_max_scroll(5, 10);
    assert_eq!(ui.description_max_scroll(), 0);
}

#[test]
fn test_update_script_max_scroll() {
    let mut ui = UiState::new();
    ui.update_script_max_scroll(15, 5);
    assert_eq!(ui.script_max_scroll(), 10);
}

#[test]
fn test_update_help_max_scroll() {
    let mut ui = UiState::new();
    ui.update_help_max_scroll(30, 10);
    assert_eq!(ui.help_max_scroll(), 20);
}

// ===== ScrollManager: scroll_up on panels =====

#[test]
fn test_scroll_up_on_list_panel_moves_navigation() {
    let mut ui = UiState::new();
    let mut nav = NavigationState::new();
    nav.scroll_down(5, 10);
    assert_eq!(nav.selected_index(), 5);
    ScrollManager::scroll_up(&mut ui, &mut nav, 2);
    assert_eq!(nav.selected_index(), 3);
}

#[test]
fn test_scroll_up_on_description_panel_decrements_offset() {
    let mut ui = UiState::new();
    let mut nav = NavigationState::new();
    ui.cycle_panel(); // List -> Description
    ui.set_description_scroll_offset(5);
    ScrollManager::scroll_up(&mut ui, &mut nav, 2);
    assert_eq!(ui.description_scroll_offset(), 3);
}

#[test]
fn test_scroll_up_on_description_panel_saturates_at_zero() {
    let mut ui = UiState::new();
    let mut nav = NavigationState::new();
    ui.cycle_panel(); // Description
    ui.set_description_scroll_offset(1);
    ScrollManager::scroll_up(&mut ui, &mut nav, 10);
    assert_eq!(ui.description_scroll_offset(), 0);
}

#[test]
fn test_scroll_up_on_script_panel_decrements_offset() {
    let mut ui = UiState::new();
    let mut nav = NavigationState::new();
    ui.cycle_panel();
    ui.cycle_panel(); // Script
    ui.set_script_scroll_offset(8);
    ScrollManager::scroll_up(&mut ui, &mut nav, 3);
    assert_eq!(ui.script_scroll_offset(), 5);
}

// ===== ScrollManager: scroll_down on panels =====

#[test]
fn test_scroll_down_on_list_panel_moves_navigation() {
    let mut ui = UiState::new();
    let mut nav = NavigationState::new();
    ScrollManager::scroll_down(&mut ui, &mut nav, 3, 10);
    assert_eq!(nav.selected_index(), 3);
}

#[test]
fn test_scroll_down_on_description_panel_increments_offset() {
    let mut ui = UiState::new();
    let mut nav = NavigationState::new();
    ui.cycle_panel(); // Description
    ui.update_description_max_scroll(20, 5);
    ScrollManager::scroll_down(&mut ui, &mut nav, 3, 10);
    assert_eq!(ui.description_scroll_offset(), 3);
}

#[test]
fn test_scroll_down_on_description_panel_clamped_to_max() {
    let mut ui = UiState::new();
    let mut nav = NavigationState::new();
    ui.cycle_panel(); // Description
    ui.update_description_max_scroll(20, 15); // max = 5
    ScrollManager::scroll_down(&mut ui, &mut nav, 100, 10);
    assert_eq!(ui.description_scroll_offset(), 5, "Should clamp to max scroll");
}

#[test]
fn test_scroll_down_on_script_panel_increments_offset() {
    let mut ui = UiState::new();
    let mut nav = NavigationState::new();
    ui.cycle_panel();
    ui.cycle_panel(); // Script
    ui.update_script_max_scroll(20, 5);
    ScrollManager::scroll_down(&mut ui, &mut nav, 4, 10);
    assert_eq!(ui.script_scroll_offset(), 4);
}

// ===== ScrollManager: move_top / move_bottom =====

#[test]
fn test_move_top_on_list_resets_navigation() {
    let mut ui = UiState::new();
    let mut nav = NavigationState::new();
    nav.move_bottom(10);
    ScrollManager::move_top(&mut ui, &mut nav);
    assert_eq!(nav.selected_index(), 0);
}

#[test]
fn test_move_top_on_description_zeroes_offset() {
    let mut ui = UiState::new();
    let mut nav = NavigationState::new();
    ui.cycle_panel(); // Description
    ui.set_description_scroll_offset(7);
    ScrollManager::move_top(&mut ui, &mut nav);
    assert_eq!(ui.description_scroll_offset(), 0);
}

#[test]
fn test_move_top_on_script_zeroes_offset() {
    let mut ui = UiState::new();
    let mut nav = NavigationState::new();
    ui.cycle_panel();
    ui.cycle_panel(); // Script
    ui.set_script_scroll_offset(4);
    ScrollManager::move_top(&mut ui, &mut nav);
    assert_eq!(ui.script_scroll_offset(), 0);
}

#[test]
fn test_move_bottom_on_list_moves_navigation_to_last() {
    let mut ui = UiState::new();
    let mut nav = NavigationState::new();
    ScrollManager::move_bottom(&mut ui, &mut nav, 10);
    assert_eq!(nav.selected_index(), 9);
}

#[test]
fn test_move_bottom_on_description_sets_max() {
    let mut ui = UiState::new();
    let mut nav = NavigationState::new();
    ui.cycle_panel(); // Description
    ui.update_description_max_scroll(20, 5); // max = 15
    ScrollManager::move_bottom(&mut ui, &mut nav, 10);
    assert_eq!(ui.description_scroll_offset(), ui.description_max_scroll());
}

#[test]
fn test_move_bottom_on_script_sets_max() {
    let mut ui = UiState::new();
    let mut nav = NavigationState::new();
    ui.cycle_panel();
    ui.cycle_panel(); // Script
    ui.update_script_max_scroll(30, 10); // max = 20
    ScrollManager::move_bottom(&mut ui, &mut nav, 10);
    assert_eq!(ui.script_scroll_offset(), ui.script_max_scroll());
}

// ===== ScrollManager: help modal =====

#[test]
fn test_help_scroll_up_decrements_offset() {
    let mut ui = UiState::new();
    ui.update_help_max_scroll(20, 5);
    ui.set_help_scroll_offset(5);
    ScrollManager::help_scroll_up(&mut ui);
    assert_eq!(ui.help_scroll_offset(), 4);
}

#[test]
fn test_help_scroll_up_saturates_at_zero() {
    let mut ui = UiState::new();
    ScrollManager::help_scroll_up(&mut ui);
    assert_eq!(ui.help_scroll_offset(), 0);
}

#[test]
fn test_help_scroll_down_increments_offset() {
    let mut ui = UiState::new();
    ui.update_help_max_scroll(20, 5);
    ScrollManager::help_scroll_down(&mut ui);
    assert_eq!(ui.help_scroll_offset(), 1);
}

#[test]
fn test_help_scroll_down_clamped_to_max() {
    let mut ui = UiState::new();
    ui.update_help_max_scroll(20, 15); // max = 5
    ui.set_help_scroll_offset(5);
    ScrollManager::help_scroll_down(&mut ui);
    assert_eq!(ui.help_scroll_offset(), 5, "Should not exceed max");
}

#[test]
fn test_help_jump_top_sets_offset_to_zero() {
    let mut ui = UiState::new();
    ui.set_help_scroll_offset(10);
    ScrollManager::help_jump_top(&mut ui);
    assert_eq!(ui.help_scroll_offset(), 0);
}

#[test]
fn test_help_jump_bottom_sets_offset_to_max() {
    let mut ui = UiState::new();
    ui.update_help_max_scroll(30, 10); // max = 20
    ScrollManager::help_jump_bottom(&mut ui);
    assert_eq!(ui.help_scroll_offset(), 20);
}
