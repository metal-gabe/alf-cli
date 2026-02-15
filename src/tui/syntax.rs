//! Syntax highlighting for shell scripts using syntect.

#[cfg(feature = "syntax-highlighting")]
use syntect::easy::HighlightLines;
#[cfg(feature = "syntax-highlighting")]
use syntect::highlighting::ThemeSet;
#[cfg(feature = "syntax-highlighting")]
use syntect::parsing::SyntaxSet;
#[cfg(feature = "syntax-highlighting")]
use syntect::util::LinesWithEndings;

use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span, Text};

/// Convert syntect color to ratatui color
#[cfg(feature = "syntax-highlighting")]
fn syntect_color_to_ratatui(color: syntect::highlighting::Color) -> Color {
   Color::Rgb(color.r, color.g, color.b)
}

/// Highlight shell script code with syntax highlighting
#[cfg(feature = "syntax-highlighting")]
pub fn highlight_shell_script(code: &str) -> Text<'static> {
   let ps = SyntaxSet::load_defaults_newlines();
   let ts = ThemeSet::load_defaults();

   // Use a dark theme that works well with our color scheme
   let theme = &ts.themes["base16-ocean.dark"];

   // Try to find the best syntax for shell scripts
   let syntax = ps
      .find_syntax_by_extension("sh")
      .or_else(|| ps.find_syntax_by_extension("bash"))
      .or_else(|| ps.find_syntax_by_extension("zsh"))
      .unwrap_or_else(|| ps.find_syntax_plain_text());

   let mut highlighter = HighlightLines::new(syntax, theme);
   let mut lines = Vec::new();

   for line in LinesWithEndings::from(code) {
      let ranges = highlighter.highlight_line(line, &ps).unwrap_or_default();

      let mut spans = Vec::new();
      for (style, text) in ranges {
         let fg = syntect_color_to_ratatui(style.foreground);
         let ratatui_style = Style::default().fg(fg);
         spans.push(Span::styled(text.to_string(), ratatui_style));
      }

      lines.push(Line::from(spans));
   }

   Text::from(lines)
}

/// Highlight shell script code (fallback when feature is disabled)
#[cfg(not(feature = "syntax-highlighting"))]
pub fn highlight_shell_script(code: &str) -> Text<'static> {
   Text::from(code.to_string())
}

/// Highlight shell script code with line numbers and optional dimming for inactive panels
pub fn highlight_shell_script_with_style(code: &str, dim: bool) -> Text<'static> {
   let text = highlight_shell_script(code);

   // Add line numbers to each line
   let mut numbered_lines = Vec::new();
   let line_count = text.lines.len();
   let line_num_width = line_count.to_string().len().max(2); // At least 2 digits width

   for (idx, line) in text.lines.into_iter().enumerate() {
      let line_num = idx + 1;
      let line_num_str = format!("{:>width$} ", line_num, width = line_num_width);

      // Create line number span with dimmed style
      let line_num_span = Span::styled(
         line_num_str,
         Style::default()
            .fg(Color::Rgb(100, 100, 100)) // Dark gray color for line numbers
            .add_modifier(if dim { ratatui::style::Modifier::DIM } else { ratatui::style::Modifier::empty() }),
      );

      // Combine line number with the existing line spans
      let mut new_spans = vec![line_num_span];
      new_spans.extend(line.spans.into_iter().map(|span| {
         if dim {
            Span::styled(span.content, span.style.add_modifier(ratatui::style::Modifier::DIM))
         } else {
            span
         }
      }));

      numbered_lines.push(Line::from(new_spans));
   }

   Text::from(numbered_lines)
}
