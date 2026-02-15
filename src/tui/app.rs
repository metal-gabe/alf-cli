//! Application state management for the TUI.

use crate::models::{AliasEntry, EntryType};
use std::time::Instant;

/// Input mode for the application
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
   /// Normal vim navigation mode
   Normal,
   /// Typing in the search bar
   Search,
}

/// Active panel for scroll context
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Panel {
   /// Left panel: alias/function list
   List,
   /// Right-top panel: comments/description
   Description,
   /// Right-bottom panel: script/function body
   Script,
}

/// Filter for entry types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryFilter {
   /// Show all entries (globe icon)
   All,
   /// Show only aliases (& icon)
   Aliases,
   /// Show only functions (f icon)
   Functions,
}

/// Grouping mode for entries
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroupMode {
   /// All entries mixed together
   None,
   /// Aliases first, then functions
   Aliases,
   /// Functions first, then aliases
   Functions,
}

/// Sorting order for entries
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortOrder {
   /// A-Z by name
   Ascending,
   /// Z-A by name
   Descending,
}

/// Main application state
pub struct App {
   /// All loaded alias/function entries
   pub entries: Vec<AliasEntry>,
   /// Filtered and searched entries (indexes into `entries`)
   pub visible_indices: Vec<usize>,
   /// Current search query
   pub search_query: String,
   /// Cursor position within the search query
   pub cursor_position: usize,
   /// Index of selected item within `visible_indices`
   pub selected_index: usize,
   /// Current input mode
   pub input_mode: InputMode,
   /// Currently active panel (receives scroll commands)
   pub active_panel: Panel,
   /// Current entry type filter
   pub filter: EntryFilter,
   /// Scroll offset for the list panel
   pub list_scroll_offset: usize,
   /// Scroll offset for the description panel
   pub description_scroll_offset: usize,
   /// Scroll offset for the script panel
   pub script_scroll_offset: usize,
   /// Pending key for multi-key sequences (e.g. 'g' for 'gg')
   pub pending_key: Option<char>,
   /// Timestamp when pending key was set (for timeout handling)
   pub pending_key_time: Option<Instant>,
   /// Flag to signal application should quit
   pub should_quit: bool,
   /// Flag to show/hide the help modal
   pub show_help: bool,
   /// Current grouping mode
   pub group_mode: GroupMode,
   /// Current sort order
   pub sort_order: SortOrder,
}

impl App {
   /// Create a new App instance with the given entries
   pub fn new(entries: Vec<AliasEntry>) -> Self {
      let visible_indices: Vec<usize> = (0..entries.len()).collect();
      Self {
         entries,
         visible_indices,
         search_query: String::new(),
         cursor_position: 0,
         selected_index: 0,
         input_mode: InputMode::Normal,
         active_panel: Panel::List,
         filter: EntryFilter::All,
         list_scroll_offset: 0,
         description_scroll_offset: 0,
         script_scroll_offset: 0,
         pending_key: None,
         pending_key_time: None,
         should_quit: false,
         show_help: false,
         group_mode: GroupMode::Aliases,   // Default: aliases first
         sort_order: SortOrder::Ascending, // Default: A-Z
      }
   }

   /// Get the currently selected entry, if any
   pub fn selected_entry(&self) -> Option<&AliasEntry> {
      self.visible_indices.get(self.selected_index).and_then(|&idx| self.entries.get(idx))
   }

   /// Update visible entries based on current filter and search query
   pub fn update_visible_entries(&mut self) {
      let query = self.search_query.to_lowercase();

      self.visible_indices = self
         .entries
         .iter()
         .enumerate()
         .filter(|(_, entry)| match self.filter {
            EntryFilter::All => true,
            EntryFilter::Aliases => entry.entry_type == EntryType::Alias,
            EntryFilter::Functions => entry.entry_type == EntryType::Function,
         })
         .filter(|(_, entry)| {
            if query.is_empty() {
               return true;
            }
            // Simple substring matching on name, value, and comments
            let name_match = entry.name.to_lowercase().contains(&query);
            let value_match = entry.value.to_lowercase().contains(&query);
            let comment_match = entry
               .comments
               .as_ref()
               .map(|comments| comments.iter().any(|c| c.to_lowercase().contains(&query)))
               .unwrap_or(false);
            name_match || value_match || comment_match
         })
         .map(|(idx, _)| idx)
         .collect();

      // Apply grouping and sorting
      self.apply_grouping_and_sorting();

      // Clamp selected index to valid range
      if self.visible_indices.is_empty() {
         self.selected_index = 0;
      } else if self.selected_index >= self.visible_indices.len() {
         self.selected_index = self.visible_indices.len() - 1;
      }

      // Reset detail panel scroll when results change
      self.description_scroll_offset = 0;
      self.script_scroll_offset = 0;
   }

   /// Apply grouping and sorting to visible_indices
   fn apply_grouping_and_sorting(&mut self) {
      match self.group_mode {
         GroupMode::None => {
            // All entries mixed together, sort by name
            self.visible_indices.sort_by(|&a, &b| {
               let name_a = &self.entries[a].name;
               let name_b = &self.entries[b].name;
               match self.sort_order {
                  SortOrder::Ascending => name_a.cmp(name_b),
                  SortOrder::Descending => name_b.cmp(name_a),
               }
            });
         }
         GroupMode::Aliases => {
            // Aliases first, then functions, each group sorted by name
            self.visible_indices.sort_by(|&a, &b| {
               let entry_a = &self.entries[a];
               let entry_b = &self.entries[b];

               // First, group by type (aliases before functions)
               match (entry_a.entry_type, entry_b.entry_type) {
                  (EntryType::Alias, EntryType::Function) => std::cmp::Ordering::Less,
                  (EntryType::Function, EntryType::Alias) => std::cmp::Ordering::Greater,
                  _ => {
                     // Within same group, sort by name
                     match self.sort_order {
                        SortOrder::Ascending => entry_a.name.cmp(&entry_b.name),
                        SortOrder::Descending => entry_b.name.cmp(&entry_a.name),
                     }
                  }
               }
            });
         }
         GroupMode::Functions => {
            // Functions first, then aliases, each group sorted by name
            self.visible_indices.sort_by(|&a, &b| {
               let entry_a = &self.entries[a];
               let entry_b = &self.entries[b];

               // First, group by type (functions before aliases)
               match (entry_a.entry_type, entry_b.entry_type) {
                  (EntryType::Function, EntryType::Alias) => std::cmp::Ordering::Less,
                  (EntryType::Alias, EntryType::Function) => std::cmp::Ordering::Greater,
                  _ => {
                     // Within same group, sort by name
                     match self.sort_order {
                        SortOrder::Ascending => entry_a.name.cmp(&entry_b.name),
                        SortOrder::Descending => entry_b.name.cmp(&entry_a.name),
                     }
                  }
               }
            });
         }
      }
   }

   /// Move selection up by one
   pub fn move_up(&mut self) {
      if self.selected_index > 0 {
         self.selected_index -= 1;
         self.description_scroll_offset = 0;
         self.script_scroll_offset = 0;
      }
   }

   /// Move selection down by one
   pub fn move_down(&mut self) {
      if !self.visible_indices.is_empty() && self.selected_index < self.visible_indices.len() - 1 {
         self.selected_index += 1;
         self.description_scroll_offset = 0;
         self.script_scroll_offset = 0;
      }
   }

   /// Jump to the top of the list
   pub fn move_top(&mut self) {
      self.selected_index = 0;
      self.list_scroll_offset = 0;
      self.description_scroll_offset = 0;
      self.script_scroll_offset = 0;
   }

   /// Jump to the bottom of the list
   pub fn move_bottom(&mut self) {
      if !self.visible_indices.is_empty() {
         self.selected_index = self.visible_indices.len() - 1;
         self.description_scroll_offset = 0;
         self.script_scroll_offset = 0;
      }
   }

   /// Scroll the active panel up
   pub fn scroll_up(&mut self, amount: usize) {
      match self.active_panel {
         Panel::List => {
            self.selected_index = self.selected_index.saturating_sub(amount);
            self.description_scroll_offset = 0;
            self.script_scroll_offset = 0;
         }
         Panel::Description => {
            self.description_scroll_offset = self.description_scroll_offset.saturating_sub(amount);
         }
         Panel::Script => {
            self.script_scroll_offset = self.script_scroll_offset.saturating_sub(amount);
         }
      }
   }

   /// Scroll the active panel down
   pub fn scroll_down(&mut self, amount: usize) {
      match self.active_panel {
         Panel::List => {
            if !self.visible_indices.is_empty() {
               self.selected_index = (self.selected_index + amount).min(self.visible_indices.len() - 1);
               self.description_scroll_offset = 0;
               self.script_scroll_offset = 0;
            }
         }
         Panel::Description => {
            self.description_scroll_offset += amount;
         }
         Panel::Script => {
            self.script_scroll_offset += amount;
         }
      }
   }

   /// Cycle to the next panel (forward)
   pub fn cycle_panel(&mut self) {
      self.active_panel = match self.active_panel {
         Panel::List => Panel::Description,
         Panel::Description => Panel::Script,
         Panel::Script => Panel::List,
      };
   }

   /// Cycle to the previous panel (backward)
   pub fn cycle_panel_backward(&mut self) {
      self.active_panel = match self.active_panel {
         Panel::List => Panel::Script,
         Panel::Script => Panel::Description,
         Panel::Description => Panel::List,
      };
   }

   /// Cycle the entry type filter (forward)
   pub fn cycle_filter(&mut self) {
      self.filter = match self.filter {
         EntryFilter::All => EntryFilter::Aliases,
         EntryFilter::Aliases => EntryFilter::Functions,
         EntryFilter::Functions => EntryFilter::All,
      };
      self.update_visible_entries();
   }

   /// Cycle the entry type filter (backward)
   pub fn cycle_filter_backward(&mut self) {
      self.filter = match self.filter {
         EntryFilter::All => EntryFilter::Functions,
         EntryFilter::Functions => EntryFilter::Aliases,
         EntryFilter::Aliases => EntryFilter::All,
      };
      self.update_visible_entries();
   }

   /// Set a specific filter
   pub fn set_filter(&mut self, filter: EntryFilter) {
      if self.filter != filter {
         self.filter = filter;
         self.update_visible_entries();
      }
   }

   /// Enter search mode
   pub fn enter_search_mode(&mut self) {
      self.input_mode = InputMode::Search;
   }

   /// Exit search mode, keeping the current query
   pub fn exit_search_keep_query(&mut self) {
      self.input_mode = InputMode::Normal;
   }

   /// Exit search mode, clearing the query
   pub fn exit_search_clear_query(&mut self) {
      self.input_mode = InputMode::Normal;
      self.search_query.clear();
      self.cursor_position = 0;
      self.update_visible_entries();
   }

   /// Clear the search query without changing mode
   pub fn clear_search(&mut self) {
      self.search_query.clear();
      self.cursor_position = 0;
      self.update_visible_entries();
   }

   /// Insert a character at the cursor position in the search query
   /// Uppercase letters are automatically converted to lowercase for case-insensitive search
   pub fn search_insert_char(&mut self, c: char) {
      // Convert uppercase letters to lowercase, leave special chars/symbols unchanged
      let char_to_insert = if c.is_ascii_uppercase() { c.to_ascii_lowercase() } else { c };

      self.search_query.insert(self.cursor_position, char_to_insert);
      self.cursor_position += 1;
      self.update_visible_entries();
   }

   /// Delete the character before the cursor in the search query
   pub fn search_delete_char(&mut self) {
      if self.cursor_position > 0 {
         self.cursor_position -= 1;
         self.search_query.remove(self.cursor_position);
         self.update_visible_entries();
      }
   }

   /// Move search cursor left
   pub fn search_cursor_left(&mut self) {
      if self.cursor_position > 0 {
         self.cursor_position -= 1;
      }
   }

   /// Move search cursor right
   pub fn search_cursor_right(&mut self) {
      if self.cursor_position < self.search_query.len() {
         self.cursor_position += 1;
      }
   }

   /// Toggle the help modal
   pub fn toggle_help(&mut self) {
      self.show_help = !self.show_help;
   }

   /// Cycle to the next group mode
   pub fn cycle_group_mode(&mut self) {
      self.group_mode = match self.group_mode {
         GroupMode::None => GroupMode::Aliases,
         GroupMode::Aliases => GroupMode::Functions,
         GroupMode::Functions => GroupMode::None,
      };
      self.update_visible_entries();
   }

   /// Cycle to the previous group mode
   pub fn cycle_group_mode_backward(&mut self) {
      self.group_mode = match self.group_mode {
         GroupMode::None => GroupMode::Functions,
         GroupMode::Functions => GroupMode::Aliases,
         GroupMode::Aliases => GroupMode::None,
      };
      self.update_visible_entries();
   }

   /// Toggle sort order
   pub fn toggle_sort_order(&mut self) {
      self.sort_order = match self.sort_order {
         SortOrder::Ascending => SortOrder::Descending,
         SortOrder::Descending => SortOrder::Ascending,
      };
      self.update_visible_entries();
   }

   /// Check if pending key has timed out (2 seconds)
   pub fn is_pending_key_expired(&self) -> bool {
      if let (Some(_), Some(time)) = (self.pending_key, self.pending_key_time) {
         time.elapsed() > std::time::Duration::from_secs(2)
      } else {
         false
      }
   }

   /// Clear pending key state
   pub fn clear_pending_key(&mut self) {
      self.pending_key = None;
      self.pending_key_time = None;
   }

   /// Set pending key with timestamp
   pub fn set_pending_key(&mut self, key: char) {
      self.pending_key = Some(key);
      self.pending_key_time = Some(Instant::now());
   }

   /// Update application state (called each tick)
   pub fn tick(&mut self) {
      // Check and clear expired pending keys
      if self.is_pending_key_expired() {
         self.clear_pending_key();
      }
   }
}
