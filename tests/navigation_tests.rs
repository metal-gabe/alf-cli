//! Tests for NavigationState

use alf::tui::state::NavigationState;

// ===== move_up =====

#[test]
fn test_move_up_decrements_from_non_zero() {
    let mut nav = NavigationState::new();
    nav.move_bottom(5);
    nav.move_up();
    assert_eq!(nav.selected_index(), 3);
}

#[test]
fn test_move_up_stops_at_zero() {
    let mut nav = NavigationState::new();
    nav.move_up();
    assert_eq!(nav.selected_index(), 0, "Should not wrap or underflow below 0");
}

// ===== move_down =====

#[test]
fn test_move_down_increments_from_zero() {
    let mut nav = NavigationState::new();
    nav.move_down(5);
    assert_eq!(nav.selected_index(), 1);
}

#[test]
fn test_move_down_stops_at_last_item() {
    let mut nav = NavigationState::new();
    nav.move_bottom(5);
    nav.move_down(5);
    assert_eq!(nav.selected_index(), 4, "Should not exceed count - 1");
}

#[test]
fn test_move_down_with_empty_list_stays_at_zero() {
    let mut nav = NavigationState::new();
    nav.move_down(0);
    assert_eq!(nav.selected_index(), 0);
}

// ===== move_top / move_bottom =====

#[test]
fn test_move_top_resets_to_zero() {
    let mut nav = NavigationState::new();
    nav.move_bottom(10);
    nav.move_top();
    assert_eq!(nav.selected_index(), 0);
}

#[test]
fn test_move_bottom_jumps_to_last() {
    let mut nav = NavigationState::new();
    nav.move_bottom(10);
    assert_eq!(nav.selected_index(), 9);
}

#[test]
fn test_move_bottom_with_empty_list_stays_at_zero() {
    let mut nav = NavigationState::new();
    nav.move_bottom(0);
    assert_eq!(nav.selected_index(), 0);
}

#[test]
fn test_move_bottom_with_single_item() {
    let mut nav = NavigationState::new();
    nav.move_bottom(1);
    assert_eq!(nav.selected_index(), 0);
}

// ===== scroll_up =====

#[test]
fn test_scroll_up_decrements_by_amount() {
    let mut nav = NavigationState::new();
    nav.move_bottom(10);
    nav.scroll_up(3);
    assert_eq!(nav.selected_index(), 6);
}

#[test]
fn test_scroll_up_saturates_at_zero() {
    let mut nav = NavigationState::new();
    nav.move_down(5);
    nav.scroll_up(100);
    assert_eq!(nav.selected_index(), 0, "scroll_up should saturate at 0");
}

// ===== scroll_down =====

#[test]
fn test_scroll_down_increments_by_amount() {
    let mut nav = NavigationState::new();
    nav.scroll_down(3, 10);
    assert_eq!(nav.selected_index(), 3);
}

#[test]
fn test_scroll_down_clamps_to_last_item() {
    let mut nav = NavigationState::new();
    nav.scroll_down(100, 5);
    assert_eq!(nav.selected_index(), 4, "scroll_down should clamp to count - 1");
}

#[test]
fn test_scroll_down_with_empty_list_stays_at_zero() {
    let mut nav = NavigationState::new();
    nav.scroll_down(5, 0);
    assert_eq!(nav.selected_index(), 0);
}

// ===== clamp =====

#[test]
fn test_clamp_within_range_no_change() {
    let mut nav = NavigationState::new();
    nav.move_bottom(10);
    nav.clamp(10);
    assert_eq!(nav.selected_index(), 9);
}

#[test]
fn test_clamp_above_range_reduces_to_last() {
    let mut nav = NavigationState::new();
    nav.move_bottom(10);
    nav.clamp(5);
    assert_eq!(nav.selected_index(), 4, "Should clamp to count - 1");
}

#[test]
fn test_clamp_zero_count_resets_to_zero() {
    let mut nav = NavigationState::new();
    nav.move_bottom(10);
    nav.clamp(0);
    assert_eq!(nav.selected_index(), 0);
}

// ===== round-trip =====

#[test]
fn test_move_down_then_up_returns_to_start() {
    let mut nav = NavigationState::new();
    nav.move_down(5);
    nav.move_down(5);
    nav.move_up();
    nav.move_up();
    assert_eq!(nav.selected_index(), 0);
}
