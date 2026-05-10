//! Tests for FilterState (filter cycling, grouping, sorting, and visible entry updates)

use crate::models::{AliasEntry, EntryType};
use std::path::PathBuf;
use super::{EntryData, EntryFilter, FilterState, GroupMode, SortOrder};

fn make_entry(name: &str, entry_type: EntryType) -> AliasEntry {
    AliasEntry {
        name: name.to_string(),
        entry_type,
        value: format!("value_for_{}", name),
        comments: None,
        source_file: PathBuf::from("test.sh"),
    }
}

fn make_mixed_data() -> EntryData {
    EntryData::new(vec![
        make_entry("gamma", EntryType::Function),
        make_entry("alpha", EntryType::Alias),
        make_entry("delta", EntryType::Function),
        make_entry("beta", EntryType::Alias),
    ])
}

// ===== cycle_filter =====

#[test]
fn test_cycle_filter_forward_all_to_aliases() {
    let mut state = FilterState::new();
    assert_eq!(state.filter(), EntryFilter::All);
    state.cycle_filter();
    assert_eq!(state.filter(), EntryFilter::Aliases);
}

#[test]
fn test_cycle_filter_forward_aliases_to_functions() {
    let mut state = FilterState::new();
    state.cycle_filter();
    state.cycle_filter();
    assert_eq!(state.filter(), EntryFilter::Functions);
}

#[test]
fn test_cycle_filter_forward_functions_back_to_all() {
    let mut state = FilterState::new();
    state.cycle_filter();
    state.cycle_filter();
    state.cycle_filter();
    assert_eq!(state.filter(), EntryFilter::All);
}

// ===== cycle_filter_backward =====

#[test]
fn test_cycle_filter_backward_all_to_functions() {
    let mut state = FilterState::new();
    state.cycle_filter_backward();
    assert_eq!(state.filter(), EntryFilter::Functions);
}

#[test]
fn test_cycle_filter_backward_functions_to_aliases() {
    let mut state = FilterState::new();
    state.cycle_filter_backward();
    state.cycle_filter_backward();
    assert_eq!(state.filter(), EntryFilter::Aliases);
}

#[test]
fn test_cycle_filter_backward_aliases_back_to_all() {
    let mut state = FilterState::new();
    state.cycle_filter_backward();
    state.cycle_filter_backward();
    state.cycle_filter_backward();
    assert_eq!(state.filter(), EntryFilter::All);
}

// ===== set_filter =====

#[test]
fn test_set_filter_directly() {
    let mut state = FilterState::new();
    state.set_filter(EntryFilter::Functions);
    assert_eq!(state.filter(), EntryFilter::Functions);
    state.set_filter(EntryFilter::All);
    assert_eq!(state.filter(), EntryFilter::All);
}

// ===== cycle_group_mode =====

#[test]
fn test_cycle_group_mode_none_to_aliases() {
    let mut state = FilterState::new();
    state.cycle_group_mode();
    state.cycle_group_mode();
    assert_eq!(state.group_mode(), GroupMode::None);
    state.cycle_group_mode();
    assert_eq!(state.group_mode(), GroupMode::Aliases);
}

#[test]
fn test_cycle_group_mode_full_cycle() {
    let mut state = FilterState::new();
    assert_eq!(state.group_mode(), GroupMode::Aliases);
    state.cycle_group_mode();
    assert_eq!(state.group_mode(), GroupMode::Functions);
    state.cycle_group_mode();
    assert_eq!(state.group_mode(), GroupMode::None);
    state.cycle_group_mode();
    assert_eq!(state.group_mode(), GroupMode::Aliases);
}

// ===== cycle_group_mode_backward =====

#[test]
fn test_cycle_group_mode_backward_full_cycle() {
    let mut state = FilterState::new();
    state.cycle_group_mode_backward();
    assert_eq!(state.group_mode(), GroupMode::None);
    state.cycle_group_mode_backward();
    assert_eq!(state.group_mode(), GroupMode::Functions);
    state.cycle_group_mode_backward();
    assert_eq!(state.group_mode(), GroupMode::Aliases);
}

// ===== toggle_sort_order =====

#[test]
fn test_toggle_sort_order_ascending_to_descending() {
    let mut state = FilterState::new();
    assert_eq!(state.sort_order(), SortOrder::Ascending);
    state.toggle_sort_order();
    assert_eq!(state.sort_order(), SortOrder::Descending);
}

#[test]
fn test_toggle_sort_order_descending_to_ascending() {
    let mut state = FilterState::new();
    state.toggle_sort_order();
    state.toggle_sort_order();
    assert_eq!(state.sort_order(), SortOrder::Ascending);
}

// ===== update_visible_entries with filter =====

#[test]
fn test_filter_all_returns_all_entries() {
    let state = FilterState::new();
    let mut data = make_mixed_data();
    state.update_visible_entries(&mut data, "");
    assert_eq!(data.visible_count(), 4);
}

#[test]
fn test_filter_aliases_returns_only_aliases() {
    let mut state = FilterState::new();
    state.set_filter(EntryFilter::Aliases);
    let mut data = make_mixed_data();
    state.update_visible_entries(&mut data, "");
    assert_eq!(data.visible_count(), 2);
    for i in 0..data.visible_count() {
        let entry = data.get_visible_entry(i).unwrap();
        assert_eq!(entry.entry_type, EntryType::Alias, "Filtered result should be alias");
    }
}

#[test]
fn test_filter_functions_returns_only_functions() {
    let mut state = FilterState::new();
    state.set_filter(EntryFilter::Functions);
    let mut data = make_mixed_data();
    state.update_visible_entries(&mut data, "");
    assert_eq!(data.visible_count(), 2);
    for i in 0..data.visible_count() {
        let entry = data.get_visible_entry(i).unwrap();
        assert_eq!(entry.entry_type, EntryType::Function, "Filtered result should be function");
    }
}

// ===== update_visible_entries with search query =====

#[test]
fn test_search_query_filters_by_name() {
    let state = FilterState::new();
    let mut data = make_mixed_data();
    state.update_visible_entries(&mut data, "alpha");
    assert_eq!(data.visible_count(), 1);
    assert_eq!(data.get_visible_entry(0).unwrap().name, "alpha");
}

#[test]
fn test_search_query_no_match_returns_empty() {
    let state = FilterState::new();
    let mut data = make_mixed_data();
    state.update_visible_entries(&mut data, "zzznomatch");
    assert_eq!(data.visible_count(), 0);
}

#[test]
fn test_search_query_matches_by_value() {
    let state = FilterState::new();
    let mut data = make_mixed_data();
    state.update_visible_entries(&mut data, "value_for_alpha");
    assert_eq!(data.visible_count(), 1);
    assert_eq!(data.get_visible_entry(0).unwrap().name, "alpha");
}

// ===== Grouping order =====

#[test]
fn test_group_mode_aliases_first_puts_aliases_before_functions() {
    let mut state = FilterState::new();
    state.set_filter(EntryFilter::All);
    assert_eq!(state.group_mode(), GroupMode::Aliases);
    let mut data = make_mixed_data();
    state.update_visible_entries(&mut data, "");
    let first = data.get_visible_entry(0).unwrap();
    let second = data.get_visible_entry(1).unwrap();
    assert_eq!(first.entry_type, EntryType::Alias);
    assert_eq!(second.entry_type, EntryType::Alias);
    let third = data.get_visible_entry(2).unwrap();
    let fourth = data.get_visible_entry(3).unwrap();
    assert_eq!(third.entry_type, EntryType::Function);
    assert_eq!(fourth.entry_type, EntryType::Function);
}

#[test]
fn test_group_mode_functions_first_puts_functions_before_aliases() {
    let mut state = FilterState::new();
    state.cycle_group_mode();
    assert_eq!(state.group_mode(), GroupMode::Functions);
    let mut data = make_mixed_data();
    state.update_visible_entries(&mut data, "");
    let first = data.get_visible_entry(0).unwrap();
    let second = data.get_visible_entry(1).unwrap();
    assert_eq!(first.entry_type, EntryType::Function);
    assert_eq!(second.entry_type, EntryType::Function);
    let third = data.get_visible_entry(2).unwrap();
    let fourth = data.get_visible_entry(3).unwrap();
    assert_eq!(third.entry_type, EntryType::Alias);
    assert_eq!(fourth.entry_type, EntryType::Alias);
}

// ===== Sorting within groups =====

#[test]
fn test_sort_ascending_orders_aliases_a_to_z() {
    let state = FilterState::new();
    let mut data = make_mixed_data();
    state.update_visible_entries(&mut data, "");
    assert_eq!(data.get_visible_entry(0).unwrap().name, "alpha");
    assert_eq!(data.get_visible_entry(1).unwrap().name, "beta");
}

#[test]
fn test_sort_descending_orders_aliases_z_to_a() {
    let mut state = FilterState::new();
    state.toggle_sort_order();
    assert_eq!(state.sort_order(), SortOrder::Descending);
    let mut data = make_mixed_data();
    state.update_visible_entries(&mut data, "");
    assert_eq!(data.get_visible_entry(0).unwrap().name, "beta");
    assert_eq!(data.get_visible_entry(1).unwrap().name, "alpha");
}

#[test]
fn test_sort_ascending_no_grouping_all_a_to_z() {
    let mut state = FilterState::new();
    state.cycle_group_mode();
    state.cycle_group_mode();
    assert_eq!(state.group_mode(), GroupMode::None);
    let mut data = make_mixed_data();
    state.update_visible_entries(&mut data, "");
    assert_eq!(data.get_visible_entry(0).unwrap().name, "alpha");
    assert_eq!(data.get_visible_entry(1).unwrap().name, "beta");
    assert_eq!(data.get_visible_entry(2).unwrap().name, "delta");
    assert_eq!(data.get_visible_entry(3).unwrap().name, "gamma");
}
