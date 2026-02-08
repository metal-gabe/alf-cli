//! Fuzzy search implementation using nucleo-matcher.

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
pub fn fuzzy_search(
    _entries: &[AliasEntry],
    _query: &str,
    _opts: &SearchOptions,
) -> Vec<SearchResult> {
    // TODO: Implement fuzzy search
    // - Create nucleo Matcher with appropriate config
    // - Iterate through entries
    // - Match query against entry name and comments
    // - Collect results with scores
    // - Sort by score (descending)
    Vec::new()
}

/// Create and configure a nucleo Matcher based on search options
fn _create_matcher(_opts: &SearchOptions) -> nucleo_matcher::Matcher {
    // TODO: Initialize nucleo Matcher
    // - Configure case sensitivity based on opts.case_matching
    // - Set normalization based on opts.normalize
    // - Return configured matcher
    todo!("Implement matcher creation")
}
