//! Entry filtering, grouping, and sorting

use super::data::EntryData;
use crate::models::EntryType;

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

/// Filter state management
#[derive(Debug, Clone)]
pub struct FilterState {
   /// Current entry type filter
   filter: EntryFilter,
   /// Current grouping mode
   group_mode: GroupMode,
   /// Current sort order
   sort_order: SortOrder,
}

impl Default for FilterState {
   fn default() -> Self {
      Self {
         filter: EntryFilter::All,
         group_mode: GroupMode::Aliases,   // Default: aliases first
         sort_order: SortOrder::Ascending, // Default: A-Z
      }
   }
}

impl FilterState {
   /// Create a new FilterState
   pub fn new() -> Self {
      Self::default()
   }

   /// Get the current filter
   pub fn filter(&self) -> EntryFilter {
      self.filter
   }

   /// Get the current group mode
   pub fn group_mode(&self) -> GroupMode {
      self.group_mode
   }

   /// Get the current sort order
   pub fn sort_order(&self) -> SortOrder {
      self.sort_order
   }

   /// Cycle the entry type filter (forward)
   pub fn cycle_filter(&mut self) {
      self.filter = match self.filter {
         EntryFilter::All => EntryFilter::Aliases,
         EntryFilter::Aliases => EntryFilter::Functions,
         EntryFilter::Functions => EntryFilter::All,
      };
   }

   /// Cycle the entry type filter (backward)
   pub fn cycle_filter_backward(&mut self) {
      self.filter = match self.filter {
         EntryFilter::All => EntryFilter::Functions,
         EntryFilter::Functions => EntryFilter::Aliases,
         EntryFilter::Aliases => EntryFilter::All,
      };
   }

   /// Set a specific filter
   pub fn set_filter(&mut self, filter: EntryFilter) {
      self.filter = filter;
   }

   /// Cycle to the next group mode
   pub fn cycle_group_mode(&mut self) {
      self.group_mode = match self.group_mode {
         GroupMode::None => GroupMode::Aliases,
         GroupMode::Aliases => GroupMode::Functions,
         GroupMode::Functions => GroupMode::None,
      };
   }

   /// Cycle to the previous group mode
   pub fn cycle_group_mode_backward(&mut self) {
      self.group_mode = match self.group_mode {
         GroupMode::None => GroupMode::Functions,
         GroupMode::Functions => GroupMode::Aliases,
         GroupMode::Aliases => GroupMode::None,
      };
   }

   /// Toggle sort order
   pub fn toggle_sort_order(&mut self) {
      self.sort_order = match self.sort_order {
         SortOrder::Ascending => SortOrder::Descending,
         SortOrder::Descending => SortOrder::Ascending,
      };
   }

   /// Update visible entries based on current filter, search query, grouping, and sorting
   pub fn update_visible_entries(&self, data: &mut EntryData, search_query: &str) {
      let query = search_query.to_lowercase();

      // Filter by type and search query
      *data.visible_indices_mut() = data
         .entries()
         .iter()
         .enumerate()
         .filter(|(_, entry)| self.matches_filter(entry))
         .filter(|(_, entry)| self.matches_search(entry, &query))
         .map(|(idx, _)| idx)
         .collect();

      // Apply grouping and sorting
      self.apply_grouping_and_sorting(data);
   }

   /// Check if entry matches current filter
   fn matches_filter(&self, entry: &crate::models::AliasEntry) -> bool {
      match self.filter {
         EntryFilter::All => true,
         EntryFilter::Aliases => entry.entry_type == EntryType::Alias,
         EntryFilter::Functions => entry.entry_type == EntryType::Function,
      }
   }

   /// Check if entry matches search query
   fn matches_search(&self, entry: &crate::models::AliasEntry, query: &str) -> bool {
      if query.is_empty() {
         return true;
      }
      // Simple substring matching on name, value, and comments
      let name_match = entry.name.to_lowercase().contains(query);
      let value_match = entry.value.to_lowercase().contains(query);
      let comment_match = entry
         .comments
         .as_ref()
         .map(|comments| comments.iter().any(|c| c.to_lowercase().contains(query)))
         .unwrap_or(false);
      name_match || value_match || comment_match
   }

   /// Apply grouping and sorting to visible_indices
   fn apply_grouping_and_sorting(&self, data: &mut EntryData) {
      let sort_order = self.sort_order;

      match self.group_mode {
         GroupMode::None => {
            // All entries mixed together, sort by name
            data.sort_visible_indices(|entry_a, entry_b| match sort_order {
               SortOrder::Ascending => entry_a.name.cmp(&entry_b.name),
               SortOrder::Descending => entry_b.name.cmp(&entry_a.name),
            });
         }
         GroupMode::Aliases => {
            // Aliases first, then functions, each group sorted by name
            data.sort_visible_indices(|entry_a, entry_b| {
               // First, group by type (aliases before functions)
               match (entry_a.entry_type, entry_b.entry_type) {
                  (EntryType::Alias, EntryType::Function) => std::cmp::Ordering::Less,
                  (EntryType::Function, EntryType::Alias) => std::cmp::Ordering::Greater,
                  _ => {
                     // Within same group, sort by name
                     match sort_order {
                        SortOrder::Ascending => entry_a.name.cmp(&entry_b.name),
                        SortOrder::Descending => entry_b.name.cmp(&entry_a.name),
                     }
                  }
               }
            });
         }
         GroupMode::Functions => {
            // Functions first, then aliases, each group sorted by name
            data.sort_visible_indices(|entry_a, entry_b| {
               // First, group by type (functions before aliases)
               match (entry_a.entry_type, entry_b.entry_type) {
                  (EntryType::Function, EntryType::Alias) => std::cmp::Ordering::Less,
                  (EntryType::Alias, EntryType::Function) => std::cmp::Ordering::Greater,
                  _ => {
                     // Within same group, sort by name
                     match sort_order {
                        SortOrder::Ascending => entry_a.name.cmp(&entry_b.name),
                        SortOrder::Descending => entry_b.name.cmp(&entry_a.name),
                     }
                  }
               }
            });
         }
      }
   }
}
