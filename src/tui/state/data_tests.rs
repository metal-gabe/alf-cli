//! Tests for EntryData (entry storage and visible index management)

use crate::models::{AliasEntry, EntryType};
use std::path::PathBuf;
use super::EntryData;

fn make_entry(name: &str) -> AliasEntry {
    AliasEntry {
        name: name.to_string(),
        entry_type: EntryType::Alias,
        value: "cmd".to_string(),
        comments: None,
        source_file: PathBuf::from("test.sh"),
    }
}

// ===== new =====

#[test]
fn test_new_stores_entries() {
    let data = EntryData::new(vec![make_entry("a"), make_entry("b")]);
    assert_eq!(data.entries().len(), 2);
}

#[test]
fn test_new_visible_indices_starts_empty() {
    let data = EntryData::new(vec![make_entry("a"), make_entry("b")]);
    assert_eq!(data.visible_indices().len(), 0);
}

// ===== visible_count / is_empty =====

#[test]
fn test_visible_count_is_zero_initially() {
    let data = EntryData::new(vec![make_entry("a")]);
    assert_eq!(data.visible_count(), 0);
}

#[test]
fn test_is_empty_true_initially() {
    let data = EntryData::new(vec![make_entry("a")]);
    assert!(data.is_empty());
}

#[test]
fn test_is_empty_false_after_adding_visible_index() {
    let mut data = EntryData::new(vec![make_entry("a")]);
    data.visible_indices_mut().push(0);
    assert!(!data.is_empty());
}

#[test]
fn test_visible_count_reflects_indices() {
    let mut data = EntryData::new(vec![make_entry("a"), make_entry("b"), make_entry("c")]);
    data.visible_indices_mut().extend([0, 1, 2]);
    assert_eq!(data.visible_count(), 3);
}

// ===== get_visible_entry =====

#[test]
fn test_get_visible_entry_returns_correct_entry() {
    let mut data = EntryData::new(vec![make_entry("alpha"), make_entry("beta"), make_entry("gamma")]);
    data.visible_indices_mut().push(2);
    let entry = data.get_visible_entry(0).unwrap();
    assert_eq!(entry.name, "gamma");
}

#[test]
fn test_get_visible_entry_out_of_range_returns_none() {
    let mut data = EntryData::new(vec![make_entry("a")]);
    data.visible_indices_mut().push(0);
    assert!(data.get_visible_entry(5).is_none());
}

#[test]
fn test_get_visible_entry_empty_visible_returns_none() {
    let data = EntryData::new(vec![make_entry("a")]);
    assert!(data.get_visible_entry(0).is_none());
}

// ===== sort_visible_indices =====

#[test]
fn test_sort_visible_indices_by_name_ascending() {
    let mut data = EntryData::new(vec![make_entry("charlie"), make_entry("alice"), make_entry("bob")]);
    data.visible_indices_mut().extend([0, 1, 2]);
    data.sort_visible_indices(|a, b| a.name.cmp(&b.name));
    assert_eq!(data.get_visible_entry(0).unwrap().name, "alice");
    assert_eq!(data.get_visible_entry(1).unwrap().name, "bob");
    assert_eq!(data.get_visible_entry(2).unwrap().name, "charlie");
}

#[test]
fn test_sort_visible_indices_by_name_descending() {
    let mut data = EntryData::new(vec![make_entry("charlie"), make_entry("alice"), make_entry("bob")]);
    data.visible_indices_mut().extend([0, 1, 2]);
    data.sort_visible_indices(|a, b| b.name.cmp(&a.name));
    assert_eq!(data.get_visible_entry(0).unwrap().name, "charlie");
    assert_eq!(data.get_visible_entry(1).unwrap().name, "bob");
    assert_eq!(data.get_visible_entry(2).unwrap().name, "alice");
}

#[test]
fn test_sort_visible_indices_subset_of_entries() {
    let mut data =
        EntryData::new(vec![make_entry("charlie"), make_entry("alice"), make_entry("bob"), make_entry("diana")]);
    data.visible_indices_mut().extend([0, 2]);
    data.sort_visible_indices(|a, b| a.name.cmp(&b.name));
    assert_eq!(data.get_visible_entry(0).unwrap().name, "bob");
    assert_eq!(data.get_visible_entry(1).unwrap().name, "charlie");
}
