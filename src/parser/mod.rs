//! Shell file parsing for aliases and functions.

use anyhow::Result;
use regex::Regex;
use std::path::Path;

use crate::models::{AliasEntry, EntryType};

/// Parse a shell file and extract aliases and functions
///
/// # Arguments
/// * `path` - Path to the shell file to parse
///
/// # Returns
/// A vector of parsed alias and function entries
pub fn parse_shell_file(path: &Path) -> Result<Vec<AliasEntry>> {
   let content = std::fs::read_to_string(path)?;
   let mut entries = Vec::new();

   // Extract aliases
   entries.extend(extract_aliases(&content, path));

   // Extract functions
   entries.extend(extract_functions(&content, path));

   Ok(entries)
}

/// Extract alias definitions from shell file content
fn extract_aliases(content: &str, source: &Path) -> Vec<AliasEntry> {
   let mut entries = Vec::new();
   let lines: Vec<&str> = content.lines().collect();

   // Regex patterns for alias definitions
   // Matches: alias name='command' or alias name="command"
   // Alias names can include dots, hyphens, and underscores (e.g., ll, my.alias)
   let alias_pattern =
      Regex::new(r#"^\s*alias\s+([a-zA-Z_][a-zA-Z0-9._-]*)=(?:'([^']*)'|"([^"]*)"|([^\s]+))"#).unwrap();

   for (line_num, line) in lines.iter().enumerate() {
      if let Some(captures) = alias_pattern.captures(line) {
         let name = captures.get(1).unwrap().as_str().to_string();
         // The value is in one of these three groups (whichever matched)
         let value = captures
            .get(2)
            .or_else(|| captures.get(3))
            .or_else(|| captures.get(4))
            .map(|m| m.as_str())
            .unwrap_or("")
            .to_string();

         let comments = extract_comments(&lines, line_num);

         entries.push(AliasEntry {
            name,
            entry_type: EntryType::Alias,
            value,
            comments,
            source_file: source.to_path_buf(),
         });
      }
   }

   entries
}

/// Extract function definitions from shell file content
fn extract_functions(content: &str, source: &Path) -> Vec<AliasEntry> {
   let mut entries = Vec::new();
   let lines: Vec<&str> = content.lines().collect();

   // Regex pattern for function definitions
   // Matches: function name() { or name() {
   // Function names can include dots, hyphens, and underscores (e.g., t.command, my-func)
   let func_pattern = Regex::new(r#"^\s*(?:function\s+)?([a-zA-Z_][a-zA-Z0-9._-]*)\s*\(\)\s*\{"#).unwrap();

   for (line_num, line) in lines.iter().enumerate() {
      if let Some(captures) = func_pattern.captures(line) {
         let name = captures.get(1).unwrap().as_str().to_string();

         // Extract function body (from opening { to closing })
         let mut body_lines = vec![line.to_string()];
         let mut brace_count = 1;

         for next_line in lines.iter().skip(line_num + 1) {
            body_lines.push(next_line.to_string());
            brace_count += next_line.matches('{').count() as i32;
            brace_count -= next_line.matches('}').count() as i32;

            if brace_count == 0 {
               break;
            }
         }

         let value = body_lines.join("\n");
         let comments = extract_comments(&lines, line_num);

         entries.push(AliasEntry {
            name,
            entry_type: EntryType::Function,
            value,
            comments,
            source_file: source.to_path_buf(),
         });
      }
   }

   entries
}

/// Extract alf-friendly comments preceding a definition
///
/// Supports two formats:
/// 1. Multi-line format:
///    # alf
///    # description line 1
///    # description line 2
///    # fla
///    alias name='value'
///
/// 2. Concise format (line before definition):
///    #@: description here :f#
///    alias name='value'
///
/// If neither format is found, returns None (ignores other comments)
fn extract_comments(lines: &[&str], line_number: usize) -> Option<Vec<String>> {
   // First check for concise format on the line BEFORE the definition
   if line_number > 0 {
      if let Some(description) = extract_concise_comment(lines[line_number - 1]) {
         return Some(vec![description]);
      }
   }

   // Then check for multi-line format above the definition
   extract_multiline_comment(lines, line_number)
}

/// Extract concise alf-friendly comment: #@: description :f#
/// Returns the description if found
fn extract_concise_comment(line: &str) -> Option<String> {
   let pattern = Regex::new(r#"#@:\s*(.+?)\s*:f#"#).ok()?;
   pattern.captures(line).and_then(|caps| caps.get(1).map(|m| m.as_str().trim().to_string()))
}

/// Extract multi-line alf-friendly comment block
/// Format:
/// # alf
/// # line 1
/// # line 2
/// # fla
fn extract_multiline_comment(lines: &[&str], mut line_number: usize) -> Option<Vec<String>> {
   if line_number == 0 {
      return None;
   }

   line_number = line_number.saturating_sub(1);

   // Look for closing marker "# fla" first
   let mut closing_found = false;
   let mut description_lines = Vec::new();
   let mut temp_line_num = line_number;

   // Scan backwards to find the markers
   loop {
      let line = lines[temp_line_num].trim();

      // Check for closing marker
      if line == "# fla" {
         closing_found = true;
      } else if line == "# alf" && closing_found {
         // Found opening marker, we have a valid block
         return if description_lines.is_empty() {
            None
         } else {
            description_lines.reverse();
            Some(description_lines)
         };
      } else if closing_found && line.starts_with('#') && line != "# fla" && line != "# alf" {
         // Collect description lines between markers
         let text = line.trim_start_matches('#').trim();
         if !text.is_empty() {
            description_lines.push(text.to_string());
         }
      } else if closing_found && !line.starts_with('#') && !line.is_empty() {
         // Hit non-comment before finding opening marker, invalid block
         return None;
      } else if closing_found && line.is_empty() {
         // Empty line after closing marker, block is incomplete
         return None;
      } else if !closing_found && line.is_empty() {
         // Empty line before finding closing marker, no alf block here
         return None;
      } else if !closing_found && line.starts_with('#') && line != "# alf" {
         // Found comment that's not an alf marker before closing marker
         // This is not a valid alf block
         return None;
      }

      if temp_line_num == 0 {
         break;
      }
      temp_line_num -= 1;
   }

   None
}
