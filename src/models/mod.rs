//! Data models for representing shell aliases and functions.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Type of shell entry
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntryType {
    /// Shell alias
    Alias,
    /// Shell function
    Function,
}

/// Represents a parsed alias or function from a shell file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AliasEntry {
    /// Name of the alias or function
    pub name: String,
    /// Whether this is an alias or function
    pub entry_type: EntryType,
    /// The command or function body
    pub value: String,
    /// Optional comments providing description and context
    pub comments: Option<Vec<String>>,
    /// Source file where this entry was found
    pub source_file: PathBuf,
}

/// Search result with match score
#[derive(Debug, Clone)]
pub struct SearchResult {
    /// The matched entry
    pub entry: AliasEntry,
    /// Match score from fuzzy matcher (higher is better)
    pub score: u32,
}
