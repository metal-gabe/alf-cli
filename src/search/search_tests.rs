//! Tests for fuzzy search functionality

use super::{fuzzy_search, SearchOptions};
use crate::config::CaseMatching;
use crate::models::{AliasEntry, EntryType};
use std::path::PathBuf;

fn create_test_entry(
   name: &str,
   entry_type: EntryType,
   value: &str,
   comments: Option<Vec<String>>,
) -> AliasEntry {
   AliasEntry {
      name: name.to_string(),
      entry_type,
      value: value.to_string(),
      comments,
      source_file: PathBuf::from("test.sh"),
   }
}

#[test]
fn empty_query_returns_all_entries() {
   let entries = vec![
      create_test_entry("ll", EntryType::Alias, "ls -lah", None),
      create_test_entry("grep", EntryType::Alias, "grep --color=auto", None),
      create_test_entry("extract", EntryType::Function, "{ ... }", None),
   ];
   let results = fuzzy_search(&entries, "", &SearchOptions::default());
   assert_eq!(results.len(), 3, "Empty query should return all entries");
}

#[test]
fn fuzzy_match_by_name() {
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
fn fuzzy_match_by_comment() {
   let entries = vec![
      create_test_entry("extract", EntryType::Function, "{ ... }", Some(vec!["Extract archive files".to_string()])),
      create_test_entry("ll", EntryType::Alias, "ls -lah", None),
   ];
   let results = fuzzy_search(&entries, "archive", &SearchOptions::default());
   assert!(!results.is_empty(), "Should find entry by comment match");
   assert_eq!(results[0].entry.name, "extract", "Should match by comment");
}

#[test]
fn score_ordering() {
   let entries = vec![
      create_test_entry("extract", EntryType::Function, "{ ... }", Some(vec!["Extract archive files".to_string()])),
      create_test_entry("ll", EntryType::Alias, "ls -lah", None),
      create_test_entry("ex", EntryType::Alias, "exit", None),
   ];
   let results = fuzzy_search(&entries, "ex", &SearchOptions::default());
   assert!(results.len() >= 2, "Should find both 'ex' and 'extract'");
   let ex_score = results.iter().find(|r| r.entry.name == "ex").map(|r| r.score);
   let extract_score = results.iter().find(|r| r.entry.name == "extract").map(|r| r.score);
   assert!(ex_score.is_some() && extract_score.is_some(), "Both entries should match");
   assert!(ex_score.unwrap() >= extract_score.unwrap(), "'ex' should score >= 'extract' for query 'ex'");
}

#[test]
fn no_match_returns_empty() {
   let entries = vec![
      create_test_entry("ll", EntryType::Alias, "ls -lah", None),
      create_test_entry("grep", EntryType::Alias, "grep --color=auto", None),
   ];
   let results = fuzzy_search(&entries, "xyz999", &SearchOptions::default());
   assert!(results.is_empty(), "Non-existent query should return empty results");
}

#[test]
fn multiple_comments_search() {
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

#[test]
fn case_matching_modes_are_honoured() {
   let entries = vec![
      create_test_entry("MyAlias", EntryType::Alias, "some command", None),
      create_test_entry("myalias", EntryType::Alias, "another command", None),
   ];
   let cases: [(CaseMatching, &str, &[&str]); 5] = [
      (CaseMatching::Respect, "My", &["MyAlias"]),
      (CaseMatching::Respect, "my", &["myalias"]),
      (CaseMatching::Ignore, "MY", &["MyAlias", "myalias"]),
      (CaseMatching::Smart, "my", &["MyAlias", "myalias"]),
      (CaseMatching::Smart, "My", &["MyAlias"]),
   ];
   for (case_matching, query, expected) in cases {
      let options = SearchOptions {
         case_matching,
         ..Default::default()
      };
      let mut names: Vec<String> =
         fuzzy_search(&entries, query, &options).into_iter().map(|result| result.entry.name).collect();
      names.sort();
      assert_eq!(names, expected, "{case_matching:?} with query {query:?}");
   }
}

#[test]
fn non_ascii_names_and_queries_match() {
   let entries = vec![
      create_test_entry("caf\u{e9}", EntryType::Alias, "brew", None),
      create_test_entry("\u{65e5}\u{672c}\u{8a9e}", EntryType::Alias, "nihongo", None),
      create_test_entry("ll", EntryType::Alias, "ls -lah", None),
   ];
   let cases = [("caf\u{e9}", "caf\u{e9}"), ("\u{672c}", "\u{65e5}\u{672c}\u{8a9e}")];
   for (query, expected) in cases {
      let results = fuzzy_search(&entries, query, &SearchOptions::default());
      assert_eq!(results.len(), 1, "Query {query:?}");
      assert_eq!(results[0].entry.name, expected, "Query {query:?}");
   }
}

#[test]
fn normalization_follows_the_option() {
   let entries = vec![create_test_entry("caf\u{e9}", EntryType::Alias, "brew", None)];
   for (should_normalize, expected_count) in [(true, 1), (false, 0)] {
      let options = SearchOptions {
         should_normalize,
         ..Default::default()
      };
      assert_eq!(fuzzy_search(&entries, "cafe", &options).len(), expected_count, "should_normalize {should_normalize}");
   }
}
