//! `which`-like lookup that combines alf's parsed info with `which`/`type` output.

use crate::models::{AliasEntry, EntryType};
use crate::tui::syntax::highlight_shell_script_ansi;
use anyhow::Result;
use std::io::IsTerminal;
use std::path::Path;
use std::process::{Command, Stdio};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

const ALF_COLOR: (u8, u8, u8) = (255, 150, 50);
const ANSI_RESET: &str = "\x1b[0m";
const CONTENT_COLOR: (u8, u8, u8) = (204, 204, 204);
const CONTINUATION_MARKER: &str = "↪ ";
const LABEL_WIDTH: usize = 13;
const TAB_SPACES: &str = "    ";
const TYPE_COLOR: (u8, u8, u8) = (175, 110, 235);
const WHICH_COLOR: (u8, u8, u8) = (80, 150, 255);

/// A run of text drawn with a single ANSI style, where an empty `style` means unstyled
#[derive(Clone)]
struct StyledSpan {
   style: String,
   text: String,
}

/// A single line of table content, keeping its visible width apart from any ANSI styling
#[derive(Clone)]
struct TableLine {
   continuation: String,
   spans: Vec<StyledSpan>,
   width: usize,
}

impl TableLine {
   /// Create a line from `spans`, starting any wrapped continuation lines with `continuation`
   fn new(
      spans: Vec<StyledSpan>,
      continuation: &str,
   ) -> Self {
      let width = spans.iter().map(|span| span.text.width()).sum();

      Self {
         continuation: continuation.to_string(),
         spans,
         width,
      }
   }

   /// Create a line of unstyled text
   fn plain(text: &str) -> Self {
      Self::new(
         vec![StyledSpan {
            style: String::new(),
            text: text.to_string(),
         }],
         "",
      )
   }

   /// Create a line from text carrying ANSI styling, splitting it into styled spans
   fn styled(rendered: &str) -> Self {
      let mut spans = Vec::new();
      let mut style = String::new();
      let mut text = String::new();
      let mut chars = rendered.chars();

      while let Some(ch) = chars.next() {
         if ch != '\x1b' {
            text.push(ch);
            continue;
         }

         let mut sequence = String::from(ch);

         for next in chars.by_ref() {
            sequence.push(next);

            if next == 'm' {
               break;
            }
         }

         if !text.is_empty() {
            spans.push(StyledSpan {
               style: style.clone(),
               text: std::mem::take(&mut text),
            });
         }

         if sequence == ANSI_RESET {
            style.clear();
         } else {
            style = sequence;
         }
      }

      if !text.is_empty() {
         spans.push(StyledSpan {
            style,
            text,
         });
      }

      Self::new(spans, "")
   }

   /// Start any wrapped continuation lines with `continuation`
   fn with_continuation(
      mut self,
      continuation: &str,
   ) -> Self {
      self.continuation = continuation.to_string();
      self
   }

   /// Render the line with its ANSI styling, padded with spaces to `width` visible columns
   fn padded(
      &self,
      width: usize,
   ) -> String {
      let mut rendered = String::new();
      let mut current_style = "";

      for span in &self.spans {
         if span.style != current_style {
            if !current_style.is_empty() {
               rendered.push_str(ANSI_RESET);
            }

            rendered.push_str(&span.style);
            current_style = &span.style;
         }

         rendered.push_str(&span.text);
      }

      if !current_style.is_empty() {
         rendered.push_str(ANSI_RESET);
      }

      format!("{}{}", rendered, " ".repeat(width.saturating_sub(self.width)))
   }

   /// Split the line into lines no wider than `max_width`, keeping each character's styling
   ///
   /// Lines break at the last space that fits, or mid-word when a word is wider than the space left.
   /// Every line after the first starts with the unstyled `continuation` text, shortened when the next
   /// character would not fit beside it.
   fn wrap(
      &self,
      max_width: usize,
   ) -> Vec<TableLine> {
      let continuation_width = self.continuation.width();

      if self.width <= max_width || max_width <= continuation_width {
         return vec![self.clone()];
      }

      let cells: Vec<(char, usize)> =
         self.spans.iter().enumerate().flat_map(|(idx, span)| span.text.chars().map(move |ch| (ch, idx))).collect();
      let mut rows: Vec<(String, &[(char, usize)])> = Vec::new();
      let mut start = 0;

      while start < cells.len() {
         let next_width = cells[start].0.width().unwrap_or_default();
         let mut prefix = if rows.is_empty() { String::new() } else { self.continuation.clone() };

         if prefix.width() + next_width > max_width {
            let limit = max_width.saturating_sub(next_width);
            let mut prefix_width = 0;
            prefix = prefix
               .chars()
               .take_while(|ch| {
                  prefix_width += ch.width().unwrap_or_default();
                  prefix_width <= limit
               })
               .collect();
         }

         let available = max_width - prefix.width();
         let mut end = start;
         let mut used = 0;

         while let Some((ch, _)) = cells.get(end) {
            let char_width = ch.width().unwrap_or_default();

            if used + char_width > available {
               break;
            }

            used += char_width;
            end += 1;
         }

         if end == cells.len() {
            rows.push((prefix, &cells[start..]));
            break;
         }

         let space = cells[start..end].iter().rposition(|(ch, _)| *ch == ' ').filter(|pos| *pos > 0);
         let split = space.map_or(end.max(start + 1), |pos| start + pos);
         rows.push((prefix, &cells[start..split]));
         start = if space.is_some() { split + 1 } else { split };
      }

      rows.iter().map(|(prefix, row)| self.rebuild_row(row, prefix)).collect()
   }

   /// Rebuild a line from `cells` of characters and their span indexes, starting with the unstyled `prefix`
   ///
   /// The prefix is the continuation text, shortened when a wide character would not otherwise fit, or empty
   /// for the first row.
   fn rebuild_row(
      &self,
      cells: &[(char, usize)],
      prefix: &str,
   ) -> TableLine {
      let mut spans: Vec<StyledSpan> = Vec::new();

      if !prefix.is_empty() {
         spans.push(StyledSpan {
            style: String::new(),
            text: prefix.to_string(),
         });
      }

      for (ch, idx) in cells {
         let style = &self.spans[*idx].style;

         match spans.last_mut() {
            Some(span) if span.style == *style => span.text.push(*ch),
            _ => spans.push(StyledSpan {
               style: style.clone(),
               text: ch.to_string(),
            }),
         }
      }

      TableLine::new(spans, &self.continuation)
   }
}

/// A titled table box: its header, the header's underline, and its content lines
struct TableSection {
   header: TableLine,
   lines: Vec<TableLine>,
   underline: TableLine,
   width: usize,
}

/// Print alf's info for `name` together with the `which` and `type` lookup results as a table
///
/// A lookup box is left out when the matching `[which]` config setting hides it.
///
/// # Arguments
/// * `name` - Name of the alias, function or command to look up
/// * `will_show_script` - Whether to print the full script contents of each alf match
pub fn run_which(
   name: &str,
   will_show_script: bool,
) -> Result<()> {
   let config = crate::config::load_config().ok();
   let entries = match crate::tui::load_entries(config.as_ref()) {
      Ok(entries) => Some(entries),
      Err(error) => {
         eprintln!("Warning: Failed to load shell files: {}", error);
         None
      },
   };
   let are_entries_loaded = entries.is_some();
   let matches = entries.as_deref().map(|entries| find_entries(entries, name)).unwrap_or_default();
   let is_terminal = std::io::stdout().is_terminal();
   let which_config = config.map(|cfg| cfg.which).unwrap_or_default();
   let which_result = resolve_lookup(name, "ALF_WHICH_RESULT", || which_lookup(name));
   let type_result = resolve_lookup(name, "ALF_TYPE_RESULT", || type_lookup(name));
   let is_known_to_shell = which_result.is_some() || type_result.is_some();
   let alf_lines = build_alf_lines(&matches, will_show_script, are_entries_loaded, is_known_to_shell, is_terminal);
   let alf_section = build_section("Alf Result", ALF_COLOR, alf_lines, is_terminal);
   let mut lookup_sections = Vec::new();

   if !which_config.hide_which.is_hidden(which_result.is_some()) {
      let which_lines = match which_result.as_deref() {
         Some(result) => build_script_lines(result, is_terminal),
         None => build_lookup_lines(None, is_terminal),
      };
      lookup_sections.push(build_section("Which Result", WHICH_COLOR, which_lines, is_terminal));
   }

   if !which_config.hide_type.is_hidden(type_result.is_some()) {
      let type_lines = build_lookup_lines(type_result.as_deref(), is_terminal);
      lookup_sections.push(build_section("Type Result", TYPE_COLOR, type_lines, is_terminal));
   }

   print_table(&lookup_sections, &alf_section);
   Ok(())
}

/// Build the alf cell lines for each matching entry, including the script only when `will_show_script` is set
///
/// With no matches, the message says when the shell files failed to load (`are_entries_loaded` unset), or
/// when `is_known_to_shell` notes that `which`/`type` found the name but no configured `shell_files` entry
/// defines it.
fn build_alf_lines(
   matches: &[&AliasEntry],
   will_show_script: bool,
   are_entries_loaded: bool,
   is_known_to_shell: bool,
   is_terminal: bool,
) -> Vec<TableLine> {
   let mut lines = Vec::new();

   if matches.is_empty() {
      let message = match (are_entries_loaded, is_known_to_shell) {
         (false, _) => "Unavailable (shell files failed to load)",
         (true, true) => "None (not in shell_files)",
         (true, false) => "None",
      };
      lines.push(build_content_line("", message, is_terminal));
   }

   for (idx, entry) in matches.iter().enumerate() {
      let kind = match entry.entry_type {
         EntryType::Alias => "alias",
         EntryType::Function => "function",
      };

      if idx > 0 {
         lines.push(TableLine::plain(""));
      }

      lines.extend(build_field_lines("Name", &[format!("{} ({})", entry.name, kind)], is_terminal));
      lines.extend(build_field_lines("Source", &[display_path(&entry.source_file)], is_terminal));
      lines.extend(build_field_lines("Description", entry.comments.as_deref().unwrap_or_default(), is_terminal));

      if will_show_script {
         lines.push(TableLine::plain(""));
         lines.push(TableLine::plain("Script:"));
         lines.extend(build_script_lines(&entry.value, is_terminal));
      }
   }

   lines
}

/// Build a line whose `value` is colored as content after an unstyled `prefix` when `is_terminal` is set
///
/// Tabs in `value` are expanded to spaces so the measured width matches what the terminal draws.
fn build_content_line(
   prefix: &str,
   value: &str,
   is_terminal: bool,
) -> TableLine {
   let continuation = if prefix.is_empty() { CONTINUATION_MARKER.to_string() } else { " ".repeat(prefix.width()) };
   let value = value.replace('\t', TAB_SPACES);

   if !is_terminal {
      return TableLine::plain(&format!("{}{}", prefix, value)).with_continuation(&continuation);
   }

   TableLine::styled(&format!("{}{}", prefix, paint(&value, CONTENT_COLOR))).with_continuation(&continuation)
}

/// Build a labelled field, indenting any continuation lines under the first
fn build_field_lines(
   label: &str,
   values: &[String],
   is_terminal: bool,
) -> Vec<TableLine> {
   let label = format!("{:<width$}", format!("{}:", label), width = LABEL_WIDTH);
   let indent = " ".repeat(LABEL_WIDTH);

   match values.split_first() {
      Some((first, rest)) => {
         let mut lines = vec![build_content_line(&label, first, is_terminal)];
         lines.extend(rest.iter().map(|value| build_content_line(&indent, value, is_terminal)));
         lines
      },
      None => vec![build_content_line(&label, "None", is_terminal)],
   }
}

/// Build the cell lines for a `which`/`type` lookup result
fn build_lookup_lines(
   result: Option<&str>,
   is_terminal: bool,
) -> Vec<TableLine> {
   result
      .unwrap_or("None")
      .replace('\t', TAB_SPACES)
      .lines()
      .map(|line| build_content_line("", line, is_terminal))
      .collect()
}

/// Build the script cell lines, syntax highlighted when `is_terminal` is set
fn build_script_lines(
   script: &str,
   is_terminal: bool,
) -> Vec<TableLine> {
   let script = script.replace('\t', TAB_SPACES);
   let script = script.trim_end_matches('\n');

   if !is_terminal {
      return script.lines().map(|line| TableLine::plain(line).with_continuation(CONTINUATION_MARKER)).collect();
   }

   let highlighted = highlight_shell_script_ansi(script);

   highlighted
      .split('\n')
      .take(script.lines().count())
      .map(|rendered| TableLine::styled(rendered).with_continuation(CONTINUATION_MARKER))
      .collect()
}

/// Build a titled table section, colored with `color` when `is_terminal` is set
fn build_section(
   text: &str,
   color: (u8, u8, u8),
   lines: Vec<TableLine>,
   is_terminal: bool,
) -> TableSection {
   let header = title(text, color, is_terminal);
   let underline = underline(&header, color, is_terminal);
   let width = lines.iter().map(|line| line.width).chain([header.width]).max().unwrap_or_default();

   TableSection {
      header,
      lines,
      underline,
      width,
   }
}

/// Build a horizontal border spanning cells of the given content `widths`
fn border(
   left: &str,
   joint: &str,
   right: &str,
   widths: &[usize],
) -> String {
   let segments: Vec<String> = widths.iter().map(|width| "─".repeat(width + 2)).collect();

   format!("{}{}{}", left, segments.join(joint), right)
}

/// Collect every entry whose name matches `name` exactly
fn find_entries<'a>(
   entries: &'a [AliasEntry],
   name: &str,
) -> Vec<&'a AliasEntry> {
   entries.iter().filter(|entry| entry.name == name).collect()
}

/// Shorten `path` with a leading `~` when it sits under `$HOME`
fn display_path(path: &Path) -> String {
   let home = std::env::var("HOME").ok().filter(|home| !home.is_empty());

   if let Some(relative) = home.as_deref().and_then(|home| path.strip_prefix(home).ok()) {
      return format!("~/{}", relative.display());
   }

   path.display().to_string()
}

/// Print a full-width alf row above the lookup sections, which sit beside each other
///
/// Each lookup section gets its own full-width row instead when there are not exactly two of them, or when
/// placing both side by side would be wider than the terminal. Every row is sized to the same total width.
///
/// When printing to a terminal, rows are capped at the terminal width and longer lines wrap.
fn print_table(
   lookup_sections: &[TableSection],
   alf_section: &TableSection,
) {
   let terminal_width = crossterm::terminal::size().ok().map(|(columns, _)| usize::from(columns));
   let max_width = terminal_width.filter(|_| std::io::stdout().is_terminal()).map(|columns| columns.saturating_sub(4));
   let side_by_side = match lookup_sections {
      [left, right] if !terminal_width.is_some_and(|columns| left.width + right.width + 7 > columns) => {
         Some((left, right))
      },
      _ => None,
   };

   println!();

   if let Some((left, right)) = side_by_side {
      let right_width = right.width.max(alf_section.width.saturating_sub(left.width + 3));
      let right_width =
         max_width.map_or(right_width, |max| right_width.min(max.saturating_sub(left.width + 3)).max(right.width));
      let full_width = left.width + right_width + 3;
      let empty = TableLine::plain("");
      let row_count = left.lines.len().max(right.lines.len());
      println!("{}", border("┌", "┬", "┐", &[full_width]));
      print_full_row(alf_section, full_width);
      println!("{}", border("├", "┬", "┤", &[left.width, right_width]));
      println!("│ {} │ {} │", left.header.padded(left.width), right.header.padded(right_width));
      println!("│ {} │ {} │", left.underline.padded(left.width), right.underline.padded(right_width));

      for idx in 0..row_count {
         let left_line = left.lines.get(idx).unwrap_or(&empty);
         let right_line = right.lines.get(idx).unwrap_or(&empty);
         println!("│ {} │ {} │", left_line.padded(left.width), right_line.padded(right_width));
      }

      println!("{}", border("└", "┴", "┘", &[left.width, right_width]));
   } else {
      let full_width = lookup_sections
         .iter()
         .chain(std::iter::once(alf_section))
         .map(|section| section.width)
         .max()
         .unwrap_or_default();
      let full_width = max_width.map_or(full_width, |max| full_width.min(max));
      println!("{}", border("┌", "┬", "┐", &[full_width]));
      print_full_row(alf_section, full_width);

      for section in lookup_sections {
         println!("{}", border("├", "┬", "┤", &[full_width]));
         print_full_row(section, full_width);
      }

      println!("{}", border("└", "┬", "┘", &[full_width]));
   }

   println!();
}

/// Print a full-width row holding a section's header, underline and lines, wrapped and padded to `width`
fn print_full_row(
   section: &TableSection,
   width: usize,
) {
   println!("│ {} │", section.header.padded(width));
   println!("│ {} │", section.underline.padded(width));

   for line in section.lines.iter().flat_map(|line| line.wrap(width)) {
      println!("│ {} │", line.padded(width));
   }
}

/// Wrap `text` in a 24-bit ANSI foreground `(r, g, b)` color, leaving the reset to [`TableLine::padded`]
fn paint(
   text: &str,
   color: (u8, u8, u8),
) -> String {
   let (r, g, b) = color;

   format!("\x1b[38;2;{};{};{}m{}", r, g, b, text)
}

/// Build a table title, colored with `color` when `is_terminal` is set
fn title(
   text: &str,
   color: (u8, u8, u8),
   is_terminal: bool,
) -> TableLine {
   if !is_terminal {
      return TableLine::plain(text);
   }

   TableLine::styled(&paint(text, color))
}

/// Build a line of dashes as wide as `title`, colored with `color` when `is_terminal` is set
fn underline(
   title: &TableLine,
   color: (u8, u8, u8),
   is_terminal: bool,
) -> TableLine {
   let dashes = "-".repeat(title.width);

   if !is_terminal {
      return TableLine::plain(&dashes);
   }

   TableLine::styled(&paint(&dashes, color))
}

/// Use the lookup result the shell wrapper passed in `env_key`, or run `fallback` when it did not
///
/// The wrapper runs `which`/`type` in the user's interactive shell, where their aliases and functions
/// are defined, and sets `ALF_LOOKUP_NAME` to the name it looked up. Its results are only trusted when
/// that name matches `name`; an empty result then means the shell found nothing.
fn resolve_lookup(
   name: &str,
   env_key: &str,
   fallback: impl FnOnce() -> Option<String>,
) -> Option<String> {
   let is_from_wrapper = std::env::var_os("ALF_LOOKUP_NAME").is_some_and(|lookup_name| lookup_name == name);

   if !is_from_wrapper {
      return fallback();
   }

   let value = std::env::var(env_key).unwrap_or_default().trim_end().to_string();

   (!value.is_empty()).then_some(value)
}

/// Run `command` and return its trimmed stdout when it succeeds with output
fn run_lookup(command: &mut Command) -> Option<String> {
   let output = command.stdin(Stdio::null()).stderr(Stdio::null()).output().ok()?;

   if !output.status.success() {
      return None;
   }

   let stdout = String::from_utf8_lossy(&output.stdout).trim_end().to_string();

   if stdout.is_empty() {
      return None;
   }

   Some(stdout)
}

/// Resolve `name` with the user's shell `type` builtin
fn type_lookup(name: &str) -> Option<String> {
   let shell = std::env::var("SHELL").ok().filter(|shell| !shell.is_empty()).unwrap_or_else(|| "sh".to_string());

   run_lookup(Command::new(shell).args(["-c", "type -- \"$1\"", "alf", name]))
}

/// Resolve `name` with the `which` binary
fn which_lookup(name: &str) -> Option<String> {
   run_lookup(Command::new("which").arg(name))
}
