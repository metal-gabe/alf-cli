//! Keybinding definitions and handling.

use crossterm::event::KeyEvent;

/// Available actions in the application
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// Quit the application
    Quit,
    /// Move selection up
    MoveUp,
    /// Move selection down
    MoveDown,
    /// Jump to top of list
    MoveTop,
    /// Jump to bottom of list
    MoveBottom,
    /// Scroll content up
    ScrollUp,
    /// Scroll content down
    ScrollDown,
    /// Page up
    PageUp,
    /// Page down
    PageDown,
    /// Focus search input
    Search,
    /// Clear search query
    ClearSearch,
}

/// Key mapping for vim-style navigation
pub struct KeyMap {
    // TODO: Store key -> action mappings
}

impl KeyMap {
    /// Create vim-style keybindings
    pub fn vim() -> Self {
        // TODO: Define vim keybindings
        // j/k - MoveDown/MoveUp
        // gg - MoveTop
        // G - MoveBottom
        // Ctrl-u/d - ScrollUp/ScrollDown
        // Ctrl-b/f - PageUp/PageDown
        // / - Search
        // Esc - ClearSearch
        // q - Quit
        Self {}
    }

    /// Get action for a key event
    pub fn get_action(&self, _key: KeyEvent) -> Option<Action> {
        // TODO: Lookup action for key
        None
    }
}
