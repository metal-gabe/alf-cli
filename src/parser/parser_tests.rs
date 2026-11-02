//! Tests for shell file parser

use crate::models::{AliasEntry, EntryType};
use std::path::Path;
use tempfile::NamedTempFile;

type BodyCase = (&'static str, &'static str, &'static [(&'static str, &'static str)]);

const FIXTURE: &str = "tests/fixtures/sample.bashrc";

fn assert_entries_snapshot(
   name: &str,
   entries: &[AliasEntry],
) {
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
fn parse_sample_bashrc() {
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
fn parse_sample_zshrc() {
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
fn parse_nonexistent_file_returns_error() {
   let result = super::parse_shell_file(Path::new("/nonexistent/path/file.sh"));
   assert!(result.is_err());
}

#[test]
fn parse_empty_file_returns_empty() {
   let temp_file = NamedTempFile::new().expect("Failed to create temp file");
   let entries = super::parse_shell_file(temp_file.path()).expect("Failed to parse empty file");
   assert!(entries.is_empty(), "Empty file should produce no entries");
}

#[test]
fn source_file_tracking() {
   let path = Path::new("tests/fixtures/sample.bashrc");
   let entries = super::parse_shell_file(path).expect("Failed to parse bashrc");
   for entry in entries {
      assert!(entry.source_file.ends_with("sample.bashrc"), "Each entry should track its source file");
   }
}

#[test]
fn dot_notation_function_names() {
   let f =
      write_temp_shell("t.helper() {\n  echo \"helper function\"\n}\nmy-alias() {\n  echo \"alias with dash\"\n}\n");
   let entries = super::parse_shell_file(f.path()).expect("Failed to parse temp file");
   assert_entries_snapshot("dot_notation_function_names", &entries);
}

#[test]
fn concise_comment_format() {
   let f = write_temp_shell(
      "#@: search with color highlighting :f#\nalias grep='grep --color=auto'\n#@: list files with long format :f#\nalias ll='ls -lah'\n",
   );
   let entries = super::parse_shell_file(f.path()).expect("Failed to parse temp file");
   assert_entries_snapshot("concise_comment_format", &entries);
}

#[test]
fn multiline_comment_format() {
   let f =
      write_temp_shell("# alf\n# extract archives of various formats\n# fla\nextract() {\n  echo \"extracting\"\n}\n");
   let entries = super::parse_shell_file(f.path()).expect("Failed to parse temp file");
   assert_entries_snapshot("multiline_comment_format", &entries);
}

#[test]
fn non_alf_comments_ignored() {
   let f =
      write_temp_shell("# This is just a regular comment\n# Another regular comment\nalias myalias='echo hello'\n");
   let entries = super::parse_shell_file(f.path()).expect("Failed to parse temp file");
   assert_entries_snapshot("non_alf_comments_ignored", &entries);
}

#[test]
fn mixed_comment_formats() {
   let f = write_temp_shell(
      "#@: concise format comment :f#\nalias a1='cmd1'\n# alf\n# multiline format comment\n# fla\nalias a2='cmd2'\n",
   );
   let entries = super::parse_shell_file(f.path()).expect("Failed to parse temp file");
   assert_entries_snapshot("mixed_comment_formats", &entries);
}

#[test]
fn empty_line_stops_multiline_comment_parsing() {
   let f = write_temp_shell(
      "# alf\n# search for file paths\n# fla\nfunction lsfp() {\n  find . -type f\n}\n\nfunction md() {\n  mkdir -p \"$1\"\n}\n",
   );
   let entries = super::parse_shell_file(f.path()).expect("Failed to parse temp file");
   assert_entries_snapshot("empty_line_stops_multiline_comment", &entries);
}

#[test]
fn fixture_aliases_have_exact_values() {
   let entries = super::parse_shell_file(Path::new(FIXTURE)).expect("Failed to parse bashrc");
   let aliases: Vec<(&str, &str)> = entries
      .iter()
      .filter(|e| e.entry_type == EntryType::Alias)
      .map(|e| (e.name.as_str(), e.value.as_str()))
      .collect();
   assert_eq!(aliases, vec![("ll", "ls -lah"), ("grep", "grep --color=auto")]);
}

#[test]
fn fixture_function_has_exact_multiline_body() {
   let fixture = std::fs::read_to_string(FIXTURE).expect("Failed to read fixture");
   let start = fixture.find("function extract()").expect("Fixture should define extract");
   let entries = super::parse_shell_file(Path::new(FIXTURE)).expect("Failed to parse bashrc");
   let functions: Vec<&AliasEntry> = entries.iter().filter(|e| e.entry_type == EntryType::Function).collect();
   assert_eq!(functions.len(), 1);
   assert_eq!(functions[0].name, "extract");
   assert_eq!(functions[0].value, fixture[start..].trim_end());
}

#[test]
fn fixture_comments_have_exact_values() {
   let entries = super::parse_shell_file(Path::new(FIXTURE)).expect("Failed to parse bashrc");
   let cases = [
      ("ll", "list files in long format with hidden files"),
      ("grep", "search with color highlighting"),
      ("extract", "extract archives of various formats"),
   ];
   for (name, expected) in cases {
      let entry = entries.iter().find(|e| e.name == name).expect("Fixture entry should exist");
      assert_eq!(entry.comments, Some(vec![expected.to_string()]), "{name}");
   }
}

#[test]
fn function_bodies_end_at_their_closing_brace() {
   let cases: [BodyCase; 4] = [
      (
         "one-line function before an alias",
         "foo() { echo hi; }\nalias a='b'\n",
         &[("a", "b"), ("foo", "foo() { echo hi; }")],
      ),
      (
         "nested braces",
         "outer() {\n  if true; then { echo a; }; fi\n}\nalias x='y'\n",
         &[("x", "y"), ("outer", "outer() {\n  if true; then { echo a; }; fi\n}")],
      ),
      (
         "multi-line function before a one-line function",
         "a() {\n  echo a\n}\nb() { echo b; }\n",
         &[("a", "a() {\n  echo a\n}"), ("b", "b() { echo b; }")],
      ),
      (
         "two braces opened on the first line",
         "c() { if x; then {\n  y\n}; }\nalias z='w'\n",
         &[("z", "w"), ("c", "c() { if x; then {\n  y\n}; }")],
      ),
   ];
   for (label, content, expected) in cases {
      let f = write_temp_shell(content);
      let entries = super::parse_shell_file(f.path()).expect("Failed to parse temp file");
      let actual: Vec<(&str, &str)> = entries.iter().map(|e| (e.name.as_str(), e.value.as_str())).collect();
      assert_eq!(actual, expected, "{label}");
   }
}

#[test]
fn descriptions_only_apply_to_the_next_definition() {
   let cases = [
      ("# alf\n# desc A\n# fla\nalias a='x'\nalias b='y'\n", "a", Some("desc A")),
      ("# alf\n# desc A\n# fla\nalias a='x'\nalias b='y'\n", "b", None),
      ("# alf\n# desc A\n# fla\nalias a='x'\nb() {\n  y\n}\n", "b", None),
      ("#@: desc A :f#\nalias a='x'\nalias b='y'\n", "b", None),
      ("# alf\nalias a='x'\n", "a", None),
   ];
   for (content, name, expected) in cases {
      let f = write_temp_shell(content);
      let entries = super::parse_shell_file(f.path()).expect("Failed to parse temp file");
      let entry = entries.iter().find(|e| e.name == name).expect("Entry should be parsed");
      assert_eq!(entry.comments, expected.map(|text| vec![text.to_string()]), "{name} in {content:?}");
   }
}

#[test]
fn non_utf8_bytes_do_not_lose_the_file() {
   use std::io::Write;
   let mut f = NamedTempFile::new().expect("Failed to create temp file");
   f.write_all(b"# caf\xe9\nalias u='v'\n").expect("Failed to write");
   let entries = super::parse_shell_file(f.path()).expect("Non-UTF-8 bytes should not fail the parse");
   assert_eq!(entries.len(), 1);
   assert_eq!(entries[0].name, "u");
   assert_eq!(entries[0].value, "v");
}
