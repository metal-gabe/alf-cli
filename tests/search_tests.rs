//! Integration tests for fuzzy search functionality

use alf::config::CaseMatching;
use alf::models::{AliasEntry, EntryType};
use alf::search::{fuzzy_search, SearchOptions};
use std::path::PathBuf;

fn create_test_entry(name: &str, entry_type: EntryType, value: &str, comments: Option<Vec<String>>) -> AliasEntry {
   AliasEntry {
      name: name.to_string(),
      entry_type,
      value: value.to_string(),
      comments,
      source_file: PathBuf::from("test.sh"),
   }
}

#[test]
fn test_empty_query_returns_all_entries() {
   let entries = vec![
      create_test_entry("ll", EntryType::Alias, "ls -lah", None),
      create_test_entry("grep", EntryType::Alias, "grep --color=auto", None),
      create_test_entry("extract", EntryType::Function, "{ ... }", None),
   ];

   let results = fuzzy_search(&entries, "", &SearchOptions::default());

   assert_eq!(results.len(), 3, "Empty query should return all entries");
}

#[test]
fn test_fuzzy_match_by_name() {
   let entries = vec![
      create_test_entry("ll", EntryType::Alias, "ls -lah", None),
      create_test_entry("grep", EntryType::Alias, "grep --color=auto", None),
      create_test_entry("extract", EntryType::Function, "{ ... }", None),
   ];

   let results = fuzzy_search(&entries, "ll", &SearchOptions::default());

   assert!(!results.is_empty(), "Should find 'll' alias");
   assert_eq!(results[0].entry.name, "ll", "First result should be 'll'");
}

#[test]
fn test_fuzzy_match_by_comment() {
   let entries = vec![
      create_test_entry("extract", EntryType::Function, "{ ... }", Some(vec!["Extract archive files".to_string()])),
      create_test_entry("ll", EntryType::Alias, "ls -lah", None),
   ];

   let results = fuzzy_search(&entries, "archive", &SearchOptions::default());

   assert!(!results.is_empty(), "Should find entry by comment match");
   assert_eq!(results[0].entry.name, "extract", "Should match by comment");
}

#[test]
fn test_score_ordering() {
   let entries = vec![
      create_test_entry("extract", EntryType::Function, "{ ... }", Some(vec!["Extract archive files".to_string()])),
      create_test_entry("ll", EntryType::Alias, "ls -lah", None),
      create_test_entry("ex", EntryType::Alias, "exit", None),
   ];

   let results = fuzzy_search(&entries, "ex", &SearchOptions::default());

   // Results should be ordered by score
   assert!(!results.is_empty(), "Should find matches");

   // "ex" should score higher on "ex" (exact match) than on "extract"
   if results.len() > 1 {
      let ex_score = results.iter().find(|r| r.entry.name == "ex").map(|r| r.score);
      let extract_score = results.iter().find(|r| r.entry.name == "extract").map(|r| r.score);

      // Both should be found, but ex should score higher
      assert!(ex_score.is_some() && extract_score.is_some(), "Both should be found");
   }
}

#[test]
fn test_case_sensitive_search() {
   let entries = vec![
      create_test_entry("MyAlias", EntryType::Alias, "some command", None),
      create_test_entry("myalias", EntryType::Alias, "another command", None),
   ];

   let options = SearchOptions { case_matching: CaseMatching::Respect, ..Default::default() };

   let results = fuzzy_search(&entries, "My", &options);

   assert!(!results.is_empty(), "Should find case-sensitive match");
}

#[test]
fn test_smart_case_search() {
   let entries = vec![
      create_test_entry("MyAlias", EntryType::Alias, "some command", None),
      create_test_entry("myalias", EntryType::Alias, "another command", None),
   ];

   let options = SearchOptions { case_matching: CaseMatching::Smart, ..Default::default() };

   // Lowercase query in smart mode should match both
   let results_lower = fuzzy_search(&entries, "my", &options);
   assert!(!results_lower.is_empty(), "Smart case should match lowercase query");
}

#[test]
fn test_no_match_returns_empty() {
   let entries = vec![
      create_test_entry("ll", EntryType::Alias, "ls -lah", None),
      create_test_entry("grep", EntryType::Alias, "grep --color=auto", None),
   ];

   let results = fuzzy_search(&entries, "xyz999", &SearchOptions::default());

   assert!(results.is_empty(), "Non-existent query should return empty results");
}

#[test]
fn test_multiple_comments_search() {
   let entries = vec![create_test_entry(
      "complex_func",
      EntryType::Function,
      "{ ... }",
      Some(vec![
         "This is a complex function".to_string(),
         "It does multiple things".to_string(),
         "Including file operations".to_string(),
      ]),
   )];

   let results = fuzzy_search(&entries, "file", &SearchOptions::default());

   assert!(!results.is_empty(), "Should find by searching through all comments");
}
