//! List navigation state and methods

/// Navigation state for list selection
#[derive(Debug, Clone)]
pub struct NavigationState {
   /// Index of selected item within visible entries
   selected_index: usize,
}

impl Default for NavigationState {
   fn default() -> Self {
      Self { selected_index: 0 }
   }
}

impl NavigationState {
   /// Create a new NavigationState
   pub fn new() -> Self {
      Self::default()
   }

   /// Get the selected index
   pub fn selected_index(&self) -> usize {
      self.selected_index
   }

   /// Move selection up by one
   pub fn move_up(&mut self) {
      if self.selected_index > 0 {
         self.selected_index -= 1;
      }
   }

   /// Move selection down by one
   pub fn move_down(&mut self, visible_count: usize) {
      if visible_count > 0 && self.selected_index < visible_count - 1 {
         self.selected_index += 1;
      }
   }

   /// Jump to the top
   pub fn move_top(&mut self) {
      self.selected_index = 0;
   }

   /// Jump to the bottom
   pub fn move_bottom(&mut self, visible_count: usize) {
      if visible_count > 0 {
         self.selected_index = visible_count - 1;
      }
   }

   /// Scroll up by amount
   pub fn scroll_up(&mut self, amount: usize) {
      self.selected_index = self.selected_index.saturating_sub(amount);
   }

   /// Scroll down by amount
   pub fn scroll_down(&mut self, amount: usize, visible_count: usize) {
      if visible_count > 0 {
         self.selected_index = (self.selected_index + amount).min(visible_count - 1);
      }
   }

   /// Clamp selection to valid range
   pub fn clamp(&mut self, visible_count: usize) {
      if visible_count == 0 {
         self.selected_index = 0;
      } else if self.selected_index >= visible_count {
         self.selected_index = visible_count - 1;
      }
   }
}
