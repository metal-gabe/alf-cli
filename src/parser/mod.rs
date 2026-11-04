//! Shell file parsing for aliases and functions.

use anyhow::Result;
use regex::Regex;
use std::iter::Peekable;
use std::path::Path;
use std::str::Chars;

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
         let mut scanner = BraceScanner::default();
         let mut brace_count = scanner.count_brace_delta(line);

         if brace_count > 0 {
            for next_line in lines.iter().skip(line_num + 1) {
               body_lines.push(next_line.to_string());
               brace_count += scanner.count_brace_delta(next_line);

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

/// The kind of quoted string a [`BraceScanner`] is inside
#[derive(Clone, Copy)]
enum Quote {
   AnsiC,
   Double,
   Single,
}

/// A heredoc whose body starts on the line after its `<<` operator
struct Heredoc {
   terminator: String,
   is_tab_stripped: bool,
}

/// Shell lexing state carried from one line of a function body to the next
#[derive(Default)]
struct BraceScanner {
   heredocs: Vec<Heredoc>,
   parameter_depth: usize,
   quote: Option<Quote>,
}

impl BraceScanner {
   /// Count the braces a line opens minus the ones it closes, as shell syntax
   ///
   /// Braces inside single, double or ANSI-C (`$'…'`) quotes, escaped with a backslash, inside a
   /// heredoc body, or after an unquoted `#` that starts a comment are ignored. Quote state carries
   /// over to the next line, so a quoted string spanning lines hides all of its braces. `${…}`
   /// parameter expansions are tracked so a `#` inside one is not taken for a comment and their braces
   /// leave the count unchanged.
   fn count_brace_delta(
      &mut self,
      line: &str,
   ) -> i32 {
      if let Some(heredoc) = self.heredocs.first() {
         let candidate = if heredoc.is_tab_stripped { line.trim_start_matches('\t') } else { line };

         if candidate == heredoc.terminator {
            self.heredocs.remove(0);
         }

         return 0;
      }

      let mut delta = 0;
      let mut previous: Option<char> = None;
      let mut chars = line.chars().peekable();

      while let Some(c) = chars.next() {
         match (self.quote, c) {
            (Some(Quote::AnsiC | Quote::Single), '\'') | (Some(Quote::Double), '"') => self.quote = None,
            (Some(Quote::AnsiC | Quote::Double) | None, '\\') => {
               chars.next();
            },
            (Some(_), _) => {},
            (None, '$') if chars.next_if_eq(&'\'').is_some() => self.quote = Some(Quote::AnsiC),
            (None, '$') if chars.next_if_eq(&'{').is_some() => self.parameter_depth += 1,
            (None, '\'') => self.quote = Some(Quote::Single),
            (None, '"') => self.quote = Some(Quote::Double),
            (None, '#') if self.parameter_depth == 0 && previous.map_or(true, char::is_whitespace) => break,
            (None, '<') if self.parameter_depth == 0 && chars.next_if_eq(&'<').is_some() => {
               self.heredocs.extend(read_heredoc(&mut chars));
            },
            (None, '{') if self.parameter_depth == 0 => delta += 1,
            (None, '}') if self.parameter_depth > 0 => self.parameter_depth -= 1,
            (None, '}') => delta -= 1,
            (None, _) => {},
         }

         previous = Some(c);
      }

      delta
   }
}

/// Read the heredoc delimiter that follows a `<<` operator
///
/// Returns `None` for a `<<<` here-string, or when the delimiter does not start with a letter, `_`,
/// a quote or a backslash, which keeps a shift such as `$(( 1 << 2 ))` from being read as a heredoc.
/// Quotes and backslashes are stripped from the delimiter, as the shell does when matching it.
fn read_heredoc(chars: &mut Peekable<Chars>) -> Option<Heredoc> {
   if chars.next_if_eq(&'<').is_some() {
      return None;
   }

   let is_tab_stripped = chars.next_if_eq(&'-').is_some();
   while chars.next_if(|c| *c == ' ' || *c == '\t').is_some() {}
   let first = *chars.peek()?;

   if !(first.is_alphabetic() || matches!(first, '_' | '\'' | '"' | '\\')) {
      return None;
   }

   let mut terminator = String::new();

   while let Some(c) = chars.next_if(|c| !c.is_whitespace() && !matches!(c, ';' | '|' | '&' | '<' | '>' | '(' | ')')) {
      if !matches!(c, '\'' | '"' | '\\') {
         terminator.push(c);
      }
   }

   if terminator.is_empty() {
      return None;
   }

   Some(Heredoc {
      terminator,
      is_tab_stripped,
   })
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
