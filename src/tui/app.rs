//! Application state management for the TUI.

use crate::models::{AliasEntry, SearchResult};
use crate::tui::{Action, Theme};

/// Main application state
#[derive(Debug)]
pub struct App {
    /// All loaded alias/function entries
    pub entries: Vec<AliasEntry>,
    /// Filtered search results
    pub filtered_results: Vec<SearchResult>,
    /// Current search query
    pub search_query: String,
    /// Index of selected result
    pub selected_index: usize,
    /// Scroll offset for result list
    pub scroll_offset: usize,
    /// Current theme
    pub theme: Theme,
    /// Flag to signal application should quit
    pub should_quit: bool,
}

impl App {
    /// Create a new App instance
    pub fn new(entries: Vec<AliasEntry>, theme: Theme) -> Self {
        Self {
            entries,
            filtered_results: Vec::new(),
            search_query: String::new(),
            selected_index: 0,
            scroll_offset: 0,
            theme,
            should_quit: false,
        }
    }

    /// Update application state (called each tick)
    pub fn tick(&mut self) {
        // TODO: Handle periodic updates (animations, etc.)
    }

    /// Handle a user action
    pub fn handle_action(&mut self, _action: Action) {
        // TODO: Process keybind actions
        // - Update selected_index for navigation
        // - Update scroll_offset for scrolling
        // - Update search_query for search
        // - Set should_quit for Quit action
    }
}
