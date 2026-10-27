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

/// Print alf's info for `name` together with the `which` and `type` lookup results as a table
///
/// # Arguments
/// * `name` - Name of the alias, function or command to look up
/// * `will_show_script` - Whether to print the full script contents of each alf match
///
/// # Errors
/// Returns an error when neither alf, `which` nor `type` know about `name`
pub fn run_which(
   name: &str,
   will_show_script: bool,
) -> Result<()> {
   let config = crate::config::load_config().ok();
   let entries = crate::tui::load_entries(config.as_ref()).unwrap_or_default();
   let matches = find_entries(&entries, name);
   let is_terminal = std::io::stdout().is_terminal();
   let which_result = which_lookup(name);
   let type_result = type_lookup(name);
   let alf_lines = build_alf_lines(&matches, will_show_script, is_terminal);
   let which_lines = build_lookup_lines(which_result.as_deref(), is_terminal);
   let type_lines = build_lookup_lines(type_result.as_deref(), is_terminal);
   print_table(&which_lines, &type_lines, &alf_lines, is_terminal);

   if matches.is_empty() && which_result.is_none() && type_result.is_none() {
      anyhow::bail!("`{}` was not found by alf, `which` or `type`", name);
   }

   Ok(())
}

/// Build the alf cell lines for each matching entry, including the script only when `will_show_script` is set
fn build_alf_lines(
   matches: &[&AliasEntry],
   will_show_script: bool,
   is_terminal: bool,
) -> Vec<TableLine> {
   let mut lines = Vec::new();

   if matches.is_empty() {
      lines.push(build_content_line("", "None", is_terminal));
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

/// Return the widest visible width among `header` and `lines`
fn max_width(
   header: &TableLine,
   lines: &[TableLine],
) -> usize {
   lines.iter().map(|line| line.width).chain([header.width]).max().unwrap_or_default()
}

/// Print the `which`/`type` lookups beside each other above a full-width alf row
///
/// The lookups are stacked into their own full-width rows instead when placing them side by side would
/// be wider than the terminal. Every row is sized to the same total width either way.
fn print_table(
   which_lines: &[TableLine],
   type_lines: &[TableLine],
   alf_lines: &[TableLine],
   is_terminal: bool,
) {
   let which_header = title("Which Result", WHICH_COLOR, is_terminal);
   let type_header = title("Type Result", TYPE_COLOR, is_terminal);
   let alf_header = title("Alf Result", ALF_COLOR, is_terminal);
   let which_underline = underline(&which_header, WHICH_COLOR, is_terminal);
   let type_underline = underline(&type_header, TYPE_COLOR, is_terminal);
   let alf_underline = underline(&alf_header, ALF_COLOR, is_terminal);
   let which_width = max_width(&which_header, which_lines);
   let type_width = max_width(&type_header, type_lines);
   let alf_width = max_width(&alf_header, alf_lines);
   let terminal_width = crossterm::terminal::size().ok().map(|(columns, _)| usize::from(columns));
   let will_stack = terminal_width.is_some_and(|columns| which_width + type_width + 7 > columns);

   println!();

   if will_stack {
      let full_width = which_width.max(type_width).max(alf_width);
      println!("{}", border("┌", "┬", "┐", &[full_width]));
      print_full_row(&which_header, &which_underline, which_lines, full_width);
      println!("{}", border("├", "┬", "┤", &[full_width]));
      print_full_row(&type_header, &type_underline, type_lines, full_width);
      println!("{}", border("├", "┬", "┤", &[full_width]));
      print_full_row(&alf_header, &alf_underline, alf_lines, full_width);
      println!("{}", border("└", "┬", "┘", &[full_width]));
   } else {
      let type_width = type_width.max(alf_width.saturating_sub(which_width + 3));
      let full_width = which_width + type_width + 3;
      let empty = TableLine::plain("");
      let row_count = which_lines.len().max(type_lines.len());
      println!("{}", border("┌", "┬", "┐", &[which_width, type_width]));
      println!("│ {} │ {} │", which_header.padded(which_width), type_header.padded(type_width));
      println!("│ {} │ {} │", which_underline.padded(which_width), type_underline.padded(type_width));

      for idx in 0..row_count {
         let which_line = which_lines.get(idx).unwrap_or(&empty);
         let type_line = type_lines.get(idx).unwrap_or(&empty);
         println!("│ {} │ {} │", which_line.padded(which_width), type_line.padded(type_width));
      }

      println!("{}", border("├", "┴", "┤", &[which_width, type_width]));
      print_full_row(&alf_header, &alf_underline, alf_lines, full_width);
      println!("{}", border("└", "┬", "┘", &[full_width]));
   }

   println!();
}

/// Print a full-width row holding `header`, its `underline` and `lines`, padded to `width`
fn print_full_row(
   header: &TableLine,
   underline: &TableLine,
   lines: &[TableLine],
   width: usize,
) {
   println!("│ {} │", header.padded(width));
   println!("│ {} │", underline.padded(width));

   for line in lines {
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
