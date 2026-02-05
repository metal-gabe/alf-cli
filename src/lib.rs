//! # alf - Alias & Function CLI Searching Tool
//!
//! A CLI tool to search & rediscover your custom shell aliases & functions.

pub mod cli;
pub mod config;
pub mod models;
pub mod parser;
pub mod search;
pub mod tui;

// Re-export commonly used types
pub use config::Config;
pub use models::{AliasEntry, EntryType};
