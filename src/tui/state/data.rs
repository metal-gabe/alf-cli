//! Entry data storage

use crate::models::AliasEntry;

/// Entry data storage
#[derive(Debug, Clone)]
pub struct EntryData {
   /// All loaded alias/function entries
   entries: Vec<AliasEntry>,
   /// Filtered and searched entries (indexes into `entries`)
   visible_indices: Vec<usize>,
}

impl EntryData {
   /// Create a new EntryData instance with the given entries
   pub fn new(entries: Vec<AliasEntry>) -> Self {
      Self { entries, visible_indices: Vec::new() }
   }

   /// Get reference to all entries
   pub fn entries(&self) -> &[AliasEntry] {
      &self.entries
   }

   /// Get reference to visible indices
   pub fn visible_indices(&self) -> &[usize] {
      &self.visible_indices
   }

   /// Get mutable reference to visible indices (for filtering operations)
   pub fn visible_indices_mut(&mut self) -> &mut Vec<usize> {
      &mut self.visible_indices
   }

   /// Get the entry at the given visible index
   pub fn get_visible_entry(&self, selected_index: usize) -> Option<&AliasEntry> {
      self.visible_indices.get(selected_index).and_then(|&idx| self.entries.get(idx))
   }

   /// Get total number of visible entries
   pub fn visible_count(&self) -> usize {
      self.visible_indices.len()
   }

   /// Check if there are no visible entries
   pub fn is_empty(&self) -> bool {
      self.visible_indices.is_empty()
   }

   /// Sort visible indices with a comparison function
   /// This method safely handles the borrow checker by splitting the data and indices
   pub fn sort_visible_indices<F>(&mut self, mut compare: F)
   where
      F: FnMut(&crate::models::AliasEntry, &crate::models::AliasEntry) -> std::cmp::Ordering,
   {
      self.visible_indices.sort_by(|&a, &b| compare(&self.entries[a], &self.entries[b]));
   }
}
