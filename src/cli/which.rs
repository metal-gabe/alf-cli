//! `which`-like lookup that combines alf's parsed info with `which`/`type` output.

use crate::models::{AliasEntry, EntryType};
use crate::tui::syntax::highlight_shell_script_ansi;
use anyhow::Result;
use std::io::IsTerminal;
use std::path::Path;
use std::process::{Command, Stdio};

const ALF_COLOR: (u8, u8, u8) = (255, 150, 50);
const ANSI_RESET: &str = "\x1b[0m";
const CONTENT_COLOR: (u8, u8, u8) = (204, 204, 204);
const LABEL_WIDTH: usize = 13;
const TAB_SPACES: &str = "    ";
const TYPE_COLOR: (u8, u8, u8) = (175, 110, 235);
const WHICH_COLOR: (u8, u8, u8) = (80, 150, 255);

/// A single line of table content, keeping its visible width apart from any ANSI styling
struct TableLine {
   rendered: String,
   width: usize,
}

impl TableLine {
   /// Create a line of unstyled text
   fn plain(text: &str) -> Self {
      Self {
         rendered: text.to_string(),
         width: text.chars().count(),
      }
   }

   /// Create a line whose `rendered` text carries ANSI styling over the visible `plain` text
   fn styled(
      plain: &str,
      rendered: &str,
   ) -> Self {
      Self {
         rendered: format!("{}{}", rendered, ANSI_RESET),
         width: plain.chars().count(),
      }
   }

   /// Render the line padded with spaces to `width` visible columns
   fn padded(
      &self,
      width: usize,
   ) -> String {
      format!("{}{}", self.rendered, " ".repeat(width.saturating_sub(self.width)))
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
   let entries = crate::tui::load_entries(config.as_ref()).unwrap_or_default();
   let matches = find_entries(&entries, name);
   let is_terminal = std::io::stdout().is_terminal();
   let which_config = config.map(|cfg| cfg.which).unwrap_or_default();
   let which_result = resolve_lookup(name, "ALF_WHICH_RESULT", || which_lookup(name));
   let type_result = resolve_lookup(name, "ALF_TYPE_RESULT", || type_lookup(name));
   let is_known_to_shell = which_result.is_some() || type_result.is_some();
   let alf_lines = build_alf_lines(&matches, will_show_script, is_known_to_shell, is_terminal);
   let alf_section = build_section("Alf Result", ALF_COLOR, alf_lines, is_terminal);
   let mut lookup_sections = Vec::new();

   if !which_config.hide_which.is_hidden(which_result.is_some()) {
      let which_lines = build_lookup_lines(which_result.as_deref(), is_terminal);
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
/// With no matches, `is_known_to_shell` notes that `which`/`type` found the name, so it exists but is not
/// defined in any configured `shell_files` entry.
fn build_alf_lines(
   matches: &[&AliasEntry],
   will_show_script: bool,
   is_known_to_shell: bool,
   is_terminal: bool,
) -> Vec<TableLine> {
   let mut lines = Vec::new();

   if matches.is_empty() {
      let message = if is_known_to_shell { "None (not in shell_files)" } else { "None" };
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
fn build_content_line(
   prefix: &str,
   value: &str,
   is_terminal: bool,
) -> TableLine {
   if !is_terminal {
      return TableLine::plain(&format!("{}{}", prefix, value));
   }

   TableLine::styled(&format!("{}{}", prefix, value), &format!("{}{}", prefix, paint(value, CONTENT_COLOR)))
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
      return script.lines().map(TableLine::plain).collect();
   }

   let highlighted = highlight_shell_script_ansi(script);

   script.lines().zip(highlighted.split('\n')).map(|(plain, rendered)| TableLine::styled(plain, rendered)).collect()
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

/// Print the lookup sections beside each other above a full-width alf row
///
/// Each lookup section gets its own full-width row instead when there are not exactly two of them, or when
/// placing both side by side would be wider than the terminal. Every row is sized to the same total width.
fn print_table(
   lookup_sections: &[TableSection],
   alf_section: &TableSection,
) {
   let terminal_width = crossterm::terminal::size().ok().map(|(columns, _)| usize::from(columns));
   let side_by_side = match lookup_sections {
      [left, right] if !terminal_width.is_some_and(|columns| left.width + right.width + 7 > columns) => {
         Some((left, right))
      },
      _ => None,
   };

   println!();

   if let Some((left, right)) = side_by_side {
      let right_width = right.width.max(alf_section.width.saturating_sub(left.width + 3));
      let full_width = left.width + right_width + 3;
      let empty = TableLine::plain("");
      let row_count = left.lines.len().max(right.lines.len());
      println!("{}", border("┌", "┬", "┐", &[left.width, right_width]));
      println!("│ {} │ {} │", left.header.padded(left.width), right.header.padded(right_width));
      println!("│ {} │ {} │", left.underline.padded(left.width), right.underline.padded(right_width));

      for idx in 0..row_count {
         let left_line = left.lines.get(idx).unwrap_or(&empty);
         let right_line = right.lines.get(idx).unwrap_or(&empty);
         println!("│ {} │ {} │", left_line.padded(left.width), right_line.padded(right_width));
      }

      println!("{}", border("├", "┴", "┤", &[left.width, right_width]));
      print_full_row(alf_section, full_width);
      println!("{}", border("└", "┬", "┘", &[full_width]));
   } else {
      let full_width = lookup_sections
         .iter()
         .chain(std::iter::once(alf_section))
         .map(|section| section.width)
         .max()
         .unwrap_or_default();
      println!("{}", border("┌", "┬", "┐", &[full_width]));

      for section in lookup_sections {
         print_full_row(section, full_width);
         println!("{}", border("├", "┬", "┤", &[full_width]));
      }

      print_full_row(alf_section, full_width);
      println!("{}", border("└", "┬", "┘", &[full_width]));
   }

   println!();
}

/// Print a full-width row holding a section's header, underline and lines, padded to `width`
fn print_full_row(
   section: &TableSection,
   width: usize,
) {
   println!("│ {} │", section.header.padded(width));
   println!("│ {} │", section.underline.padded(width));

   for line in &section.lines {
      println!("│ {} │", line.padded(width));
   }
}

/// Wrap `text` in a 24-bit ANSI foreground `(r, g, b)` color, leaving the reset to [`TableLine::styled`]
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

   TableLine::styled(text, &paint(text, color))
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

   TableLine::styled(&dashes, &paint(&dashes, color))
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
