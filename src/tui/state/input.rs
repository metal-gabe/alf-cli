//! Input mode and pending key state

use std::time::Instant;

/// Input mode for the application
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum InputMode {
   /// Normal vim navigation mode
   #[default]
   Normal,
   /// Typing in the search bar
   Search,
}

/// Input state management
#[derive(Debug, Clone, Default)]
pub struct InputState {
   /// Current input mode
   mode: InputMode,
   /// Pending key for multi-key sequences (e.g. 'o' for 'og') and when it was set, for timeout handling
   pending: Option<(char, Instant)>,
}

impl InputState {
   /// Create a new InputState
   pub fn new() -> Self {
      Self::default()
   }

   /// Get the current input mode
   pub fn mode(&self) -> InputMode {
      self.mode
   }

   /// Get the pending key
   pub fn pending_key(&self) -> Option<char> {
      self.pending.map(|(key, _)| key)
   }

   /// Get the pending key timestamp
   pub fn pending_key_time(&self) -> Option<Instant> {
      self.pending.map(|(_, time)| time)
   }

   /// Enter search mode
   pub fn enter_search(&mut self) {
      self.mode = InputMode::Search;
   }

   /// Exit search mode and return to normal mode
   pub fn exit_search(&mut self) {
      self.mode = InputMode::Normal;
   }

   /// Check if currently in search mode
   pub fn is_searching(&self) -> bool {
      self.mode == InputMode::Search
   }

   /// Set pending key with timestamp
   pub fn set_pending_key(
      &mut self,
      key: char,
   ) {
      self.pending = Some((key, Instant::now()));
   }

   /// Clear pending key state
   pub fn clear_pending_key(&mut self) {
      self.pending = None;
   }

   /// Check if pending key has timed out (2 seconds)
   pub fn is_pending_key_expired(&self) -> bool {
      self.pending.is_some_and(|(_, time)| time.elapsed() > std::time::Duration::from_secs(2))
   }
}

#[cfg(test)]
#[path = "input_tests.rs"]
mod input_tests;
