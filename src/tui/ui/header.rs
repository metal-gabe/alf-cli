//! Header bar rendering.
//!
//! Draws the top bar with filter badges, mode indicator, and shell indicator.

use ratatui::layout::Rect;
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::tui::app::App;
use crate::tui::state::{EntryFilter, InputMode};
use crate::tui::themes::Theme;

/// Draw the header bar with filter badges, mode indicator, and shell indicator
pub fn draw_header(
   frame: &mut Frame,
   app: &App,
   theme: &Theme,
   area: Rect,
) {
   let filter_color = match app.filter() {
      EntryFilter::Aliases => theme.alias_color,
      EntryFilter::All => theme.foreground,
      EntryFilter::Functions => theme.function_color,
   };

   let badge_style = Style::default().fg(theme.background).bg(filter_color).bold();

   // Build the left side: filter badges
   let badges = vec![
      Span::styled("FILTERS: ", Style::default().bold()),
      Span::styled(" @ ", if matches!(app.filter(), EntryFilter::Aliases) { badge_style } else { Style::default() }),
      Span::raw(" "),
      Span::styled(" ƒ ", if matches!(app.filter(), EntryFilter::Functions) { badge_style } else { Style::default() }),
      Span::raw(" "),
      Span::styled(" * ", if matches!(app.filter(), EntryFilter::All) { badge_style } else { Style::default() }),
   ];

   // Build the middle: mode indicator
   let (mode_text, mode_color) = match app.input_mode() {
      InputMode::Normal => ("NORMAL", theme.mode_normal_color),
      InputMode::Search => ("-- SEARCH --", theme.highlight),
   };
   let mode_span = Span::styled(mode_text, Style::default().fg(mode_color));

   // Build the right side: shell indicator
   let shell_label_prefix = " $SHELL: ";

   // Detect shell name from environment or entries
   let shell_name = detect_shell_name(app);

   let shell_label_suffix = " ";

   // Calculate widths
   let badges_width: usize = badges.iter().map(|s| s.width()).sum();
   let mode_width = mode_text.len();
   let shell_width = shell_label_prefix.len() + shell_name.len() + shell_label_suffix.len();

   // Calculate padding to center the mode and right-align the shell
   let total_width = area.width as usize;
   let left_padding = if total_width > badges_width + mode_width + shell_width {
      (total_width - badges_width - mode_width - shell_width) / 2
   } else {
      1
   };
   let right_padding = if total_width > badges_width + left_padding + mode_width + shell_width {
      total_width - badges_width - left_padding - mode_width - shell_width
   } else {
      1
   };

   let mut spans = badges;
   spans.push(Span::raw(" ".repeat(left_padding)));
   spans.push(mode_span);
   spans.push(Span::raw(" ".repeat(right_padding)));
   spans.push(Span::styled(shell_label_prefix, Style::default().bold()));
   spans.push(Span::styled(shell_name, Style::default().fg(filter_color)));
   spans.push(Span::raw(shell_label_suffix));

   let header = Paragraph::new(Line::from(spans));
   frame.render_widget(header, area);
}

/// Detect the shell name from $0 or SHELL environment variable
fn detect_shell_name(_app: &App) -> &'static str {
   if let Ok(shell_path) = std::env::var("SHELL") {
      if !shell_path.is_empty() {
         // Extract basename from path (e.g., "/bin/zsh" -> "zsh")
         if let Some(basename) = shell_path.split('/').next_back() {
            if !basename.is_empty() {
               return match basename {
                  "bash" => "bash",
                  "csh" => "csh",
                  "fish" => "fish",
                  "ksh" => "ksh",
                  "sh" => "sh",
                  "tcsh" => "tcsh",
                  "zsh" => "zsh",
                  _ => "unknown",
               };
            }
         }
      }
   }

   // Last resort
   "unknown"
}
