//! Tests for InputState (mode transitions and pending key handling)

use super::{InputMode, InputState};

// ===== Default state =====

#[test]
fn test_default_mode_is_normal() {
    let state = InputState::new();
    assert_eq!(state.mode(), InputMode::Normal);
}

#[test]
fn test_default_no_pending_key() {
    let state = InputState::new();
    assert!(state.pending_key().is_none());
    assert!(state.pending_key_time().is_none());
}

#[test]
fn test_default_not_searching() {
    let state = InputState::new();
    assert!(!state.is_searching());
}

// ===== Mode transitions =====

#[test]
fn test_enter_search_sets_search_mode() {
    let mut state = InputState::new();
    state.enter_search();
    assert_eq!(state.mode(), InputMode::Search);
    assert!(state.is_searching());
}

#[test]
fn test_exit_search_returns_to_normal() {
    let mut state = InputState::new();
    state.enter_search();
    state.exit_search();
    assert_eq!(state.mode(), InputMode::Normal);
    assert!(!state.is_searching());
}

#[test]
fn test_exit_search_from_normal_stays_normal() {
    let mut state = InputState::new();
    state.exit_search();
    assert_eq!(state.mode(), InputMode::Normal);
}

#[test]
fn test_enter_search_twice_stays_in_search() {
    let mut state = InputState::new();
    state.enter_search();
    state.enter_search();
    assert_eq!(state.mode(), InputMode::Search);
}

// ===== Pending key =====

#[test]
fn test_set_pending_key_stores_key() {
    let mut state = InputState::new();
    state.set_pending_key('g');
    assert_eq!(state.pending_key(), Some('g'));
}

#[test]
fn test_set_pending_key_records_timestamp() {
    let mut state = InputState::new();
    state.set_pending_key('g');
    assert!(state.pending_key_time().is_some());
}

#[test]
fn test_clear_pending_key_removes_key() {
    let mut state = InputState::new();
    state.set_pending_key('g');
    state.clear_pending_key();
    assert!(state.pending_key().is_none());
}

#[test]
fn test_clear_pending_key_removes_timestamp() {
    let mut state = InputState::new();
    state.set_pending_key('g');
    state.clear_pending_key();
    assert!(state.pending_key_time().is_none());
}

#[test]
fn test_is_pending_key_expired_false_when_no_key() {
    let state = InputState::new();
    assert!(!state.is_pending_key_expired());
}

#[test]
fn test_is_pending_key_expired_false_immediately_after_set() {
    let mut state = InputState::new();
    state.set_pending_key('g');
    assert!(!state.is_pending_key_expired(), "Key just set should not be expired");
}

#[test]
fn test_overwrite_pending_key() {
    let mut state = InputState::new();
    state.set_pending_key('g');
    state.set_pending_key('G');
    assert_eq!(state.pending_key(), Some('G'));
}
