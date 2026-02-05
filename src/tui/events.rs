//! Event handling for the TUI.

use anyhow::Result;
use crossterm::event::{self, Event as CrosstermEvent, KeyEvent};
use std::time::Duration;

/// Application events
#[derive(Debug, Clone)]
pub enum Event {
   /// Keyboard input event
   Key(KeyEvent),
   /// Terminal resize event
   Resize(u16, u16),
   /// Periodic tick for updates
   Tick,
}

/// Event handler for polling terminal events
pub struct EventHandler {
   _tick_rate: Duration,
}

impl EventHandler {
    /// Create a new event handler
   pub fn new(tick_rate: Duration) -> Self {
      Self { _tick_rate: tick_rate }
   }

    /// Poll for the next event
   pub fn next(&self) -> Result<Event> {
      // TODO: Implement event polling
      // - Use crossterm::event::poll() to check for events
      // - Return Event::Tick after tick_rate timeout
      // - Convert crossterm events to our Event enum
      // - Handle KeyEvent, Resize, etc.
      todo!("Implement event polling")
   }
}
