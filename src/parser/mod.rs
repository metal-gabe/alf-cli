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
   let bytes = std::fs::read(path)?;
   let content = String::from_utf8_lossy(&bytes);
   let mut entries = Vec::new();

   entries.extend(extract_aliases(&content, path));

   entries.extend(extract_functions(&content, path));

   Ok(entries)
}

/// Extract alias definitions from shell file content
fn extract_aliases(
   content: &str,
   source: &Path,
) -> Vec<AliasEntry> {
   let mut entries = Vec::new();
   let lines: Vec<&str> = content.lines().collect();

   let alias_pattern =
      Regex::new(r#"^\s*alias\s+([a-zA-Z_][a-zA-Z0-9._-]*)=(?:'([^']*)'|"([^"]*)"|([^\s]+))"#).unwrap();

   for (line_num, line) in lines.iter().enumerate() {
      if let Some(captures) = alias_pattern.captures(line) {
         let name = captures.get(1).unwrap().as_str().to_string();
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
fn extract_functions(
   content: &str,
   source: &Path,
) -> Vec<AliasEntry> {
   let mut entries = Vec::new();
   let lines: Vec<&str> = content.lines().collect();

   let func_pattern = Regex::new(r#"^\s*(?:function\s+)?([a-zA-Z_][a-zA-Z0-9._-]*)\s*\(\)\s*\{"#).unwrap();

   for (line_num, line) in lines.iter().enumerate() {
      if let Some(captures) = func_pattern.captures(line) {
         let name = captures.get(1).unwrap().as_str().to_string();

         let mut body_lines = vec![line.to_string()];
         let mut brace_count = count_brace_delta(line);

         if brace_count > 0 {
            for next_line in lines.iter().skip(line_num + 1) {
               body_lines.push(next_line.to_string());
               brace_count += count_brace_delta(next_line);

               if brace_count == 0 {
                  break;
               }
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

/// Count the braces a line opens minus the ones it closes, as shell syntax
///
/// Braces inside single or double quotes, escaped with a backslash, or after an unquoted `#` that
/// starts a comment are ignored. Quote state does not carry over to the next line, so an apostrophe
/// in a heredoc or comment-like text only affects its own line.
fn count_brace_delta(line: &str) -> i32 {
   let mut delta = 0;
   let mut quote: Option<char> = None;
   let mut previous: Option<char> = None;
   let mut chars = line.chars();

   while let Some(c) = chars.next() {
      match (quote, c) {
         (Some('\''), '\'') | (Some('"'), '"') => quote = None,
         (Some('"'), '\\') | (None, '\\') => {
            chars.next();
         },
         (Some(_), _) => {},
         (None, '\'' | '"') => quote = Some(c),
         (None, '#') if previous.map_or(true, char::is_whitespace) => break,
         (None, '{') => delta += 1,
         (None, '}') => delta -= 1,
         (None, _) => {},
      }

      previous = Some(c);
   }

   delta
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
fn extract_comments(
   lines: &[&str],
   line_number: usize,
) -> Option<Vec<String>> {
   if line_number > 0 {
      if let Some(description) = extract_concise_comment(lines[line_number - 1]) {
         return Some(vec![description]);
      }
   }

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
fn extract_multiline_comment(
   lines: &[&str],
   mut line_number: usize,
) -> Option<Vec<String>> {
   if line_number == 0 {
      return None;
   }

   line_number = line_number.saturating_sub(1);

   let mut is_closing_found = false;
   let mut description_lines = Vec::new();
   let mut temp_line_num = line_number;

   loop {
      let line = lines[temp_line_num].trim();

      if line == "# fla" {
         is_closing_found = true;
      } else if line == "# alf" && is_closing_found {
         return if description_lines.is_empty() {
            None
         } else {
            description_lines.reverse();
            Some(description_lines)
         };
      } else if is_closing_found && line.starts_with('#') && line != "# fla" && line != "# alf" {
         let text = line.trim_start_matches('#').trim();
         if !text.is_empty() {
            description_lines.push(text.to_string());
         }
      } else if !is_closing_found || !line.starts_with('#') {
         return None;
      }

      if temp_line_num == 0 {
         break;
      }
      temp_line_num -= 1;
   }

   None
}

#[cfg(test)]
mod parser_tests;
