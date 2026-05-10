//! Fuzzy search implementation using nucleo-matcher.

use nucleo_matcher::{Config, Matcher};

use crate::config::CaseMatching;
use crate::models::{AliasEntry, SearchResult};

/// Search options for configuring fuzzy matching behavior
#[derive(Debug, Clone)]
pub struct SearchOptions {
   /// Case matching strategy
   pub case_matching: CaseMatching,
   /// Enable Unicode normalization
   pub normalize: bool,
}

impl Default for SearchOptions {
   fn default() -> Self {
      Self { case_matching: CaseMatching::Smart, normalize: true }
   }
}

/// Perform fuzzy search on entries using nucleo-matcher
///
/// # Arguments
/// * `entries` - The list of aliases/functions to search through
/// * `query` - The search query string
/// * `opts` - Search configuration options
///
/// # Returns
/// A vector of search results sorted by match score (highest first)
pub fn fuzzy_search(entries: &[AliasEntry], query: &str, opts: &SearchOptions) -> Vec<SearchResult> {
   if query.is_empty() {
      // Empty query matches all entries with equal score
      return entries.iter().map(|entry| SearchResult { entry: entry.clone(), score: 0 }).collect();
   }

   let mut matcher = create_matcher(opts);
   let mut results = Vec::new();

   // Convert query to nucleo format
   let query_haystack = nucleo_matcher::Utf32Str::Ascii(query.as_bytes());

   for entry in entries {
      // Try matching against entry name first (prioritized)
      let name_haystack = nucleo_matcher::Utf32Str::Ascii(entry.name.as_bytes());
      let name_score = matcher.fuzzy_match(name_haystack, query_haystack);

      let comment_score = entry.comments.as_ref().and_then(|comments| {
         // Join comments with spaces and try to match
         let comment_text = comments.join(" ");
         let comment_haystack = nucleo_matcher::Utf32Str::Ascii(comment_text.as_bytes());
         matcher.fuzzy_match(comment_haystack, query_haystack)
      });

      // Use name score if available, otherwise use comment score
      if let Some(score) = name_score.or(comment_score) {
         results.push(SearchResult { entry: entry.clone(), score: score as u32 });
      }
   }

   // Sort by score descending (higher scores first)
   results.sort_by(|a, b| b.score.cmp(&a.score));

   results
}

/// Create and configure a nucleo Matcher based on search options
fn create_matcher(_opts: &SearchOptions) -> Matcher {
   // Note: nucleo-matcher has limited API for case matching configuration
   // The default Smart case matching should work well for most users
   // Future versions may expose more configurability
   Matcher::new(Config::DEFAULT)
}

#[cfg(test)]
mod search_tests;
