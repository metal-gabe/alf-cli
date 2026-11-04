//! Tests for `alf which` lookups and table line wrapping

use super::{resolve_lookup, TableLine};
use crate::test_support::TempHome;

const ENV_KEY: &str = "ALF_WHICH_RESULT";

fn row_text(line: &TableLine) -> String {
   line.spans.iter().map(|span| span.text.as_str()).collect()
}

fn wrapped_text(
   line: &TableLine,
   max_width: usize,
) -> Vec<String> {
   line.wrap(max_width).iter().map(row_text).collect()
}

// ===== resolve_lookup =====

#[test]
fn resolve_lookup_trusts_the_wrapper_only_for_a_matching_name() {
   let cases = [
      (Some("foo"), Some("/bin/foo"), Some("/bin/foo")),
      (Some("foo"), Some("/bin/foo  \n"), Some("/bin/foo")),
      (Some("foo"), Some(""), None),
      (Some("foo"), None, None),
      (Some("bar"), Some("/bin/bar"), Some("fallback")),
      (None, Some("/bin/foo"), Some("fallback")),
   ];

   for (lookup_name, env_value, expected) in cases {
      let mut home = TempHome::new();

      match lookup_name {
         Some(lookup_name) => home.set_env("ALF_LOOKUP_NAME", lookup_name),
         None => home.unset_env("ALF_LOOKUP_NAME"),
      }

      match env_value {
         Some(env_value) => home.set_env(ENV_KEY, env_value),
         None => home.unset_env(ENV_KEY),
      }

      let result = resolve_lookup("foo", ENV_KEY, || Some("fallback".to_string()));
      assert_eq!(result.as_deref(), expected, "lookup name {lookup_name:?}, env value {env_value:?}");
   }
}

// ===== TableLine::wrap =====

#[test]
fn wrap_splits_lines_to_fit_the_width() {
   let cases: [(&str, &str, usize, &[&str]); 7] = [
      ("short", "", 10, &["short"]),
      ("exact", "", 5, &["exact"]),
      ("aa bb cc", "", 5, &["aa", "bb cc"]),
      ("abcdefgh", "↪ ", 5, &["abcde", "↪ fgh"]),
      ("abcdef", "  ", 2, &["abcdef"]),
      ("日本語", "", 4, &["日本", "語"]),
      ("日本", "", 1, &["日", "本"]),
   ];

   for (text, continuation, max_width, expected) in cases {
      let line = TableLine::plain(text).with_continuation(continuation);
      assert_eq!(wrapped_text(&line, max_width), expected, "{text:?} at width {max_width}");
   }
}

#[test]
fn wrap_keeps_styling_on_every_row() {
   let style = "\x1b[31m";
   let rows = TableLine::styled(&format!("{style}abcd")).wrap(2);
   assert_eq!(rows.len(), 2);

   for row in &rows {
      assert_eq!(row.spans[0].style, style);
   }
}
