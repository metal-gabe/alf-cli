//! Search query state and manipulation

/// Search query state
#[derive(Debug, Clone, Default)]
pub struct SearchState {
   /// Current search query
   query: String,
   /// Cursor position within the search query (character index, not byte index)
   cursor_position: usize,
}

impl SearchState {
   /// Create a new SearchState
   pub fn new() -> Self {
      Self::default()
   }

   /// Get the search query
   pub fn query(&self) -> &str {
      &self.query
   }

   /// Get the cursor position
   pub fn cursor_position(&self) -> usize {
      self.cursor_position
   }

   /// Check if search query is empty
   pub fn is_empty(&self) -> bool {
      self.query.is_empty()
   }

   /// Set the search query directly (with lowercase conversion for case-insensitive search)
   pub fn set_query(&mut self, query: String) {
      self.query = query.to_ascii_lowercase();
      self.cursor_position = self.query.chars().count();
   }

   /// Clear the search query and reset cursor
   pub fn clear(&mut self) {
      self.query.clear();
      self.cursor_position = 0;
   }

   /// Insert a character at the cursor position
   /// Uppercase letters are automatically converted to lowercase for case-insensitive search
   pub fn insert_char(&mut self, c: char) {
      // Convert uppercase letters to lowercase, leave special chars/symbols unchanged
      let char_to_insert = if c.is_ascii_uppercase() { c.to_ascii_lowercase() } else { c };

      // Map character index to byte index for safe UTF-8 insertion
      let byte_index =
         self.query.char_indices().nth(self.cursor_position).map(|(idx, _)| idx).unwrap_or(self.query.len()); // If at end, use total byte length

      self.query.insert(byte_index, char_to_insert);
      self.cursor_position += 1;
   }

   /// Delete the character before the cursor in the search query
   pub fn delete_char(&mut self) {
      if self.cursor_position > 0 {
         // Map character index to byte index for the character to delete
         // We need to find the byte range of the character at position (cursor_position - 1)
         if let Some((byte_idx, ch)) = self.query.char_indices().nth(self.cursor_position - 1) {
            // Calculate the byte length of the character to remove
            let char_len = ch.len_utf8();
            // Remove the byte range for this character
            self.query.drain(byte_idx..byte_idx + char_len);
            self.cursor_position -= 1;
         }
      }
   }

   /// Move search cursor left
   pub fn move_cursor_left(&mut self) {
      if self.cursor_position > 0 {
         self.cursor_position -= 1;
      }
   }

   /// Move search cursor right
   pub fn move_cursor_right(&mut self) {
      // Use character count, not byte length
      let char_count = self.query.chars().count();
      if self.cursor_position < char_count {
         self.cursor_position += 1;
      }
   }
}
