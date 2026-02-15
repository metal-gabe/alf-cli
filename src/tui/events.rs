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
   tick_rate: Duration,
}

impl EventHandler {
   /// Create a new event handler with the given tick rate
   pub fn new(tick_rate: Duration) -> Self {
      Self { tick_rate }
   }

   /// Poll for the next event.
   ///
   /// Blocks until either a terminal event occurs or the tick rate elapses.
   /// Returns `Event::Tick` on timeout, or the appropriate event variant.
   pub fn next(&self) -> Result<Event> {
      if event::poll(self.tick_rate)? {
         match event::read()? {
            CrosstermEvent::Key(key) => Ok(Event::Key(key)),
            CrosstermEvent::Resize(w, h) => Ok(Event::Resize(w, h)),
            // Ignore mouse, focus, and paste events
            _ => Ok(Event::Tick),
         }
      } else {
         Ok(Event::Tick)
      }
   }
}
