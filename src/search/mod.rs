//! Fuzzy search implementation using nucleo-matcher.

use nucleo_matcher::pattern::{self, Atom, AtomKind, Normalization};
use nucleo_matcher::{Config, Matcher, Utf32String};

use crate::config::CaseMatching;
use crate::models::{AliasEntry, SearchResult};

/// Search options for configuring fuzzy matching behavior
#[derive(Debug, Clone)]
pub struct SearchOptions {
   /// Case matching strategy
   pub case_matching: CaseMatching,
   /// Enable Unicode normalization
   pub should_normalize: bool,
}

impl Default for SearchOptions {
   fn default() -> Self {
      Self {
         case_matching: CaseMatching::Smart,
         should_normalize: true,
      }
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
pub fn fuzzy_search(
   entries: &[AliasEntry],
   query: &str,
   opts: &SearchOptions,
) -> Vec<SearchResult> {
   if query.is_empty() {
      return entries
         .iter()
         .map(|entry| SearchResult {
            entry: entry.clone(),
            score: 0,
         })
         .collect();
   }

   let mut matcher = Matcher::new(Config::DEFAULT);
   let mut results = Vec::new();
   let atom = create_atom(query, opts);

   for entry in entries {
      let name_score = atom.score(Utf32String::from(entry.name.as_str()).slice(..), &mut matcher);
      let comment_score = entry
         .comments
         .as_ref()
         .and_then(|comments| atom.score(Utf32String::from(comments.join(" ")).slice(..), &mut matcher));

      if let Some(score) = name_score.or(comment_score) {
         results.push(SearchResult {
            entry: entry.clone(),
            score: u32::from(score),
         });
      }
   }

   results.sort_by_key(|r| std::cmp::Reverse(r.score));

   results
}

/// Create the fuzzy pattern for `query` from the search options
fn create_atom(
   query: &str,
   opts: &SearchOptions,
) -> Atom {
   let case_matching = match opts.case_matching {
      CaseMatching::Ignore => pattern::CaseMatching::Ignore,
      CaseMatching::Respect => pattern::CaseMatching::Respect,
      CaseMatching::Smart => pattern::CaseMatching::Smart,
   };
   let normalization = if opts.should_normalize { Normalization::Smart } else { Normalization::Never };

   Atom::new(query, case_matching, normalization, AtomKind::Fuzzy, false)
}

#[cfg(test)]
mod search_tests;
