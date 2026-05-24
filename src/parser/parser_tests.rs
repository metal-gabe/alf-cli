//! Tests for shell file parser

use crate::models::{AliasEntry, EntryType};
use std::path::Path;
use tempfile::NamedTempFile;

fn assert_entries_snapshot(name: &str, entries: &[AliasEntry]) {
   insta::with_settings!({
       filters => vec![(r"source_file: [^\n]+", "source_file: [REDACTED]")]
   }, {
       insta::assert_debug_snapshot!(name, entries);
   });
}

fn write_temp_shell(content: &str) -> NamedTempFile {
   use std::io::Write;
   let mut f = NamedTempFile::new().expect("Failed to create temp file");
   write!(f, "{}", content).expect("Failed to write");
   f
}

#[test]
fn test_parse_sample_bashrc() {
   let path = Path::new("tests/fixtures/sample.bashrc");
   let entries = super::parse_shell_file(path).expect("Failed to parse bashrc");
   assert!(!entries.is_empty(), "Should parse at least some entries");
   let ll_alias = entries.iter().find(|e| e.name == "ll");
   assert!(ll_alias.is_some(), "Should find 'll' alias in bashrc");
   if let Some(entry) = ll_alias {
      assert_eq!(entry.entry_type, EntryType::Alias);
      assert_eq!(entry.value, "ls -lah");
   }
   let extract_func = entries.iter().find(|e| e.name == "extract");
   assert!(extract_func.is_some(), "Should find 'extract' function in bashrc");
   if let Some(entry) = extract_func {
      assert_eq!(entry.entry_type, EntryType::Function);
      assert!(entry.value.contains('{'));
      assert!(entry.value.contains('}'));
   }
}

#[test]
fn test_parse_sample_zshrc() {
   let path = Path::new("tests/fixtures/sample.zshrc");
   let entries = super::parse_shell_file(path).expect("Failed to parse zshrc");
   assert!(!entries.is_empty(), "Should parse at least some entries from zshrc");
   let gs_alias = entries.iter().find(|e| e.name == "gs");
   assert!(gs_alias.is_some(), "Should find 'gs' alias in zshrc");
   if let Some(entry) = gs_alias {
      assert_eq!(entry.entry_type, EntryType::Alias);
   }
   let gbr_func = entries.iter().find(|e| e.name == "gbr");
   assert!(gbr_func.is_some(), "Should find 'gbr' function in zshrc");
   if let Some(entry) = gbr_func {
      assert_eq!(entry.entry_type, EntryType::Function);
   }
}

#[test]
fn test_parse_nonexistent_file_returns_error() {
   let result = super::parse_shell_file(Path::new("/nonexistent/path/file.sh"));
   assert!(result.is_err());
}

#[test]
fn test_parse_empty_file_returns_empty() {
   let temp_file = NamedTempFile::new().expect("Failed to create temp file");
   let entries = super::parse_shell_file(temp_file.path()).expect("Failed to parse empty file");
   assert!(entries.is_empty(), "Empty file should produce no entries");
}

#[test]
fn test_alias_extraction_with_quotes() {
   let path = Path::new("tests/fixtures/sample.bashrc");
   let entries = super::parse_shell_file(path).expect("Failed to parse bashrc");
   let aliases: Vec<_> = entries.iter().filter(|e| e.entry_type == EntryType::Alias).collect();
   assert!(!aliases.is_empty(), "Should find at least one alias");
   for alias in aliases {
      assert!(!alias.value.is_empty(), "Alias '{}' should have a command", alias.name);
   }
}

#[test]
fn test_function_extraction_multiline() {
   let path = Path::new("tests/fixtures/sample.bashrc");
   let entries = super::parse_shell_file(path).expect("Failed to parse bashrc");
   let functions: Vec<_> = entries.iter().filter(|e| e.entry_type == EntryType::Function).collect();
   assert!(!functions.is_empty(), "Should find at least one function");
   for func in functions {
      let open_braces = func.value.matches('{').count();
      let close_braces = func.value.matches('}').count();
      assert_eq!(open_braces, close_braces, "Function '{}' should have balanced braces", func.name);
   }
}

#[test]
fn test_comment_extraction() {
   let path = Path::new("tests/fixtures/sample.bashrc");
   let entries = super::parse_shell_file(path).expect("Failed to parse bashrc");
   let with_comments: Vec<_> = entries.iter().filter(|e| e.comments.is_some()).collect();
   assert!(!with_comments.is_empty(), "Sample file has comments, should be parsed");
}

#[test]
fn test_source_file_tracking() {
   let path = Path::new("tests/fixtures/sample.bashrc");
   let entries = super::parse_shell_file(path).expect("Failed to parse bashrc");
   for entry in entries {
      assert!(entry.source_file.ends_with("sample.bashrc"), "Each entry should track its source file");
   }
}

#[test]
fn test_dot_notation_function_names() {
   let f =
      write_temp_shell("t.helper() {\n  echo \"helper function\"\n}\nmy-alias() {\n  echo \"alias with dash\"\n}\n");
   let entries = super::parse_shell_file(f.path()).expect("Failed to parse temp file");
   assert_entries_snapshot("dot_notation_function_names", &entries);
}

#[test]
fn test_concise_comment_format() {
   let f = write_temp_shell(
      "#@: search with color highlighting :f#\nalias grep='grep --color=auto'\n#@: list files with long format :f#\nalias ll='ls -lah'\n",
   );
   let entries = super::parse_shell_file(f.path()).expect("Failed to parse temp file");
   assert_entries_snapshot("concise_comment_format", &entries);
}

#[test]
fn test_multiline_comment_format() {
   let f =
      write_temp_shell("# alf\n# extract archives of various formats\n# fla\nextract() {\n  echo \"extracting\"\n}\n");
   let entries = super::parse_shell_file(f.path()).expect("Failed to parse temp file");
   assert_entries_snapshot("multiline_comment_format", &entries);
}

#[test]
fn test_non_alf_comments_ignored() {
   let f =
      write_temp_shell("# This is just a regular comment\n# Another regular comment\nalias myalias='echo hello'\n");
   let entries = super::parse_shell_file(f.path()).expect("Failed to parse temp file");
   assert_entries_snapshot("non_alf_comments_ignored", &entries);
}

#[test]
fn test_mixed_comment_formats() {
   let f = write_temp_shell(
      "#@: concise format comment :f#\nalias a1='cmd1'\n# alf\n# multiline format comment\n# fla\nalias a2='cmd2'\n",
   );
   let entries = super::parse_shell_file(f.path()).expect("Failed to parse temp file");
   assert_entries_snapshot("mixed_comment_formats", &entries);
}

#[test]
fn test_empty_line_stops_multiline_comment_parsing() {
   let f = write_temp_shell(
      "# alf\n# search for file paths\n# fla\nfunction lsfp() {\n  find . -type f\n}\n\nfunction md() {\n  mkdir -p \"$1\"\n}\n",
   );
   let entries = super::parse_shell_file(f.path()).expect("Failed to parse temp file");
   assert_entries_snapshot("empty_line_stops_multiline_comment", &entries);
}
