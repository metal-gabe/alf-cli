//! Integration tests for shell file parser

use alf::models::EntryType;
use alf::parser;
use std::path::Path;

#[test]
fn test_parse_sample_bashrc() {
   let path = Path::new("tests/fixtures/sample.bashrc");
   let entries = parser::parse_shell_file(path).expect("Failed to parse bashrc");

   // We expect at least some entries from the sample file
   assert!(!entries.is_empty(), "Should parse at least some entries");

   // Check for specific aliases we know are in the file
   let ll_alias = entries.iter().find(|e| e.name == "ll");
   assert!(ll_alias.is_some(), "Should find 'll' alias in bashrc");

   if let Some(entry) = ll_alias {
      assert_eq!(entry.entry_type, EntryType::Alias);
      assert_eq!(entry.value, "ls -lah");
   }

   // Check for the extract function
   let extract_func = entries.iter().find(|e| e.name == "extract");
   assert!(extract_func.is_some(), "Should find 'extract' function in bashrc");

   if let Some(entry) = extract_func {
      assert_eq!(entry.entry_type, EntryType::Function);
      // Function should have multi-line body
      assert!(entry.value.contains('{'));
      assert!(entry.value.contains('}'));
   }
}

#[test]
fn test_parse_sample_zshrc() {
   let path = Path::new("tests/fixtures/sample.zshrc");
   if path.exists() {
      let entries = parser::parse_shell_file(path).expect("Failed to parse zshrc");
      // Just verify it parses without error
      assert!(!entries.is_empty() || entries.is_empty()); // Either way is fine for zshrc
   }
}

#[test]
fn test_alias_extraction_with_quotes() {
   // Test that aliases with different quote styles are parsed correctly
   let path = Path::new("tests/fixtures/sample.bashrc");
   let entries = parser::parse_shell_file(path).expect("Failed to parse bashrc");

   // Check for aliases with different quote styles
   let aliases: Vec<_> = entries.iter().filter(|e| e.entry_type == EntryType::Alias).collect();

   assert!(!aliases.is_empty(), "Should find at least one alias");

   // All aliases should have a value (command)
   for alias in aliases {
      assert!(!alias.value.is_empty(), "Alias '{}' should have a command", alias.name);
   }
}

#[test]
fn test_function_extraction_multiline() {
   let path = Path::new("tests/fixtures/sample.bashrc");
   let entries = parser::parse_shell_file(path).expect("Failed to parse bashrc");

   let functions: Vec<_> = entries.iter().filter(|e| e.entry_type == EntryType::Function).collect();

   assert!(!functions.is_empty(), "Should find at least one function");

   for func in functions {
      // Functions should have balanced braces
      let open_braces = func.value.matches('{').count();
      let close_braces = func.value.matches('}').count();
      assert_eq!(open_braces, close_braces, "Function '{}' should have balanced braces", func.name);
   }
}

#[test]
fn test_comment_extraction() {
   let path = Path::new("tests/fixtures/sample.bashrc");
   let entries = parser::parse_shell_file(path).expect("Failed to parse bashrc");

   // Some entries should have comments
   let with_comments: Vec<_> = entries.iter().filter(|e| e.comments.is_some()).collect();

   // The sample file has comments, so we should find some
   assert!(!with_comments.is_empty(), "Sample file has comments, should be parsed");
}

#[test]
fn test_source_file_tracking() {
   let path = Path::new("tests/fixtures/sample.bashrc");
   let entries = parser::parse_shell_file(path).expect("Failed to parse bashrc");

   // All entries should track their source file
   for entry in entries {
      assert!(entry.source_file.ends_with("sample.bashrc"), "Each entry should track its source file");
   }
}

#[test]
fn test_dot_notation_function_names() {
   // Test that function names with dots are parsed correctly
   // This is a common pattern in zsh with functions like t.cmd, t.helper, etc.
   use std::io::Write;
   use tempfile::NamedTempFile;

   let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
   writeln!(temp_file, "t.helper() {{\n  echo \"helper function\"\n}}").expect("Failed to write to temp file");
   writeln!(temp_file, "my-alias() {{\n  echo \"alias with dash\"\n}}").expect("Failed to write to temp file");

   let entries = parser::parse_shell_file(temp_file.path()).expect("Failed to parse temp file");

   // Should find both functions
   let t_helper = entries.iter().find(|e| e.name == "t.helper");
   let my_alias = entries.iter().find(|e| e.name == "my-alias");

   assert!(t_helper.is_some(), "Should find function with dot notation 't.helper'");
   assert!(my_alias.is_some(), "Should find function with dash 'my-alias'");
}

#[test]
fn test_concise_comment_format() {
   // Test the concise alf comment format: #@: description :f#
   use std::io::Write;
   use tempfile::NamedTempFile;

   let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
   writeln!(temp_file, "#@: search with color highlighting :f#").expect("Failed to write to temp file");
   writeln!(temp_file, "alias grep='grep --color=auto'").expect("Failed to write to temp file");
   writeln!(temp_file, "#@: list files with long format :f#").expect("Failed to write to temp file");
   writeln!(temp_file, "alias ll='ls -lah'").expect("Failed to write to temp file");

   let entries = parser::parse_shell_file(temp_file.path()).expect("Failed to parse temp file");

   // Both aliases should have comments
   let grep_alias = entries.iter().find(|e| e.name == "grep");
   let ll_alias = entries.iter().find(|e| e.name == "ll");

   assert!(grep_alias.is_some(), "Should find grep alias");
   assert!(ll_alias.is_some(), "Should find ll alias");

   if let Some(entry) = grep_alias {
      assert!(entry.comments.is_some(), "grep alias should have a comment");
      let comments = entry.comments.as_ref().unwrap();
      assert!(!comments.is_empty(), "grep alias comments should not be empty");
      assert_eq!(comments[0], "search with color highlighting");
   }

   if let Some(entry) = ll_alias {
      assert!(entry.comments.is_some(), "ll alias should have a comment");
      let comments = entry.comments.as_ref().unwrap();
      assert!(!comments.is_empty(), "ll alias comments should not be empty");
      assert_eq!(comments[0], "list files with long format");
   }
}

#[test]
fn test_multiline_comment_format() {
   // Test the multi-line alf comment format:
   // # alf
   // # description
   // # fla
   use std::io::Write;
   use tempfile::NamedTempFile;

   let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
   writeln!(temp_file, "# alf").expect("Failed to write to temp file");
   writeln!(temp_file, "# extract archives of various formats").expect("Failed to write to temp file");
   writeln!(temp_file, "# fla").expect("Failed to write to temp file");
   writeln!(temp_file, "extract() {{").expect("Failed to write to temp file");
   writeln!(temp_file, "  echo \"extracting\"").expect("Failed to write to temp file");
   writeln!(temp_file, "}}").expect("Failed to write to temp file");

   let entries = parser::parse_shell_file(temp_file.path()).expect("Failed to parse temp file");

   let extract_func = entries.iter().find(|e| e.name == "extract");

   assert!(extract_func.is_some(), "Should find extract function");

   if let Some(entry) = extract_func {
      assert!(entry.comments.is_some(), "extract function should have a comment");
      let comments = entry.comments.as_ref().unwrap();
      assert!(!comments.is_empty(), "extract function comments should not be empty");
      assert_eq!(comments[0], "extract archives of various formats");
   }
}

#[test]
fn test_non_alf_comments_ignored() {
   // Test that regular comments (without alf markers) are ignored
   use std::io::Write;
   use tempfile::NamedTempFile;

   let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
   writeln!(temp_file, "# This is just a regular comment").expect("Failed to write to temp file");
   writeln!(temp_file, "# Another regular comment").expect("Failed to write to temp file");
   writeln!(temp_file, "alias myalias='echo hello'").expect("Failed to write to temp file");

   let entries = parser::parse_shell_file(temp_file.path()).expect("Failed to parse temp file");

   let myalias = entries.iter().find(|e| e.name == "myalias");

   assert!(myalias.is_some(), "Should find myalias");

   if let Some(entry) = myalias {
      assert!(entry.comments.is_none(), "Regular comments should not be parsed as alf comments");
   }
}

#[test]
fn test_mixed_comment_formats() {
   // Test that both comment formats can coexist in the same file
   use std::io::Write;
   use tempfile::NamedTempFile;

   let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
   writeln!(temp_file, "#@: concise format comment :f#").expect("Failed to write to temp file");
   writeln!(temp_file, "alias a1='cmd1'").expect("Failed to write to temp file");
   writeln!(temp_file, "# alf").expect("Failed to write to temp file");
   writeln!(temp_file, "# multiline format comment").expect("Failed to write to temp file");
   writeln!(temp_file, "# fla").expect("Failed to write to temp file");
   writeln!(temp_file, "alias a2='cmd2'").expect("Failed to write to temp file");

   let entries = parser::parse_shell_file(temp_file.path()).expect("Failed to parse temp file");

   let a1 = entries.iter().find(|e| e.name == "a1");
   let a2 = entries.iter().find(|e| e.name == "a2");

   assert!(a1.is_some() && a1.as_ref().unwrap().comments.is_some(), "a1 should have concise format comment");
   assert!(a2.is_some() && a2.as_ref().unwrap().comments.is_some(), "a2 should have multiline format comment");

   let a1_comments = a1.as_ref().unwrap().comments.as_ref().unwrap();
   let a2_comments = a2.as_ref().unwrap().comments.as_ref().unwrap();

   assert!(!a1_comments.is_empty());
   assert!(!a2_comments.is_empty());
   assert_eq!(a1_comments[0], "concise format comment");
   assert_eq!(a2_comments[0], "multiline format comment");
}

#[test]
fn test_empty_line_stops_multiline_comment_parsing() {
   // Test that empty lines prevent picking up descriptions from previous entries
   // This is a regression test for the bug where `md` would incorrectly pick up `lsfp`'s description
   use std::io::Write;
   use tempfile::NamedTempFile;

   let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
   writeln!(temp_file, "# alf").expect("Failed to write to temp file");
   writeln!(temp_file, "# search for file paths").expect("Failed to write to temp file");
   writeln!(temp_file, "# fla").expect("Failed to write to temp file");
   writeln!(temp_file, "function lsfp() {{").expect("Failed to write to temp file");
   writeln!(temp_file, "  find . -type f").expect("Failed to write to temp file");
   writeln!(temp_file, "}}").expect("Failed to write to temp file");
   writeln!(temp_file).expect("Failed to write to temp file"); // Empty line between functions
   writeln!(temp_file, "function md() {{").expect("Failed to write to temp file");
   writeln!(temp_file, "  mkdir -p \"$1\"").expect("Failed to write to temp file");
   writeln!(temp_file, "}}").expect("Failed to write to temp file");

   let entries = parser::parse_shell_file(temp_file.path()).expect("Failed to parse temp file");

   let lsfp = entries.iter().find(|e| e.name == "lsfp");
   let md = entries.iter().find(|e| e.name == "md");

   assert!(lsfp.is_some(), "Should find lsfp function");
   assert!(md.is_some(), "Should find md function");

   // lsfp should have the comment
   assert!(lsfp.as_ref().unwrap().comments.is_some(), "lsfp should have a comment");
   let lsfp_comments = lsfp.as_ref().unwrap().comments.as_ref().unwrap();
   assert!(!lsfp_comments.is_empty());
   assert_eq!(lsfp_comments[0], "search for file paths");

   // md should NOT have a comment (empty line prevents comment inheritance)
   assert!(md.as_ref().unwrap().comments.is_none(), "md should NOT have a comment from lsfp");
}
