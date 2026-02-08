//! Terminal User Interface (TUI) implementation.

pub mod app;
pub mod events;
pub mod keybinds;
pub mod themes;
pub mod ui;

pub use app::App;
pub use events::{Event, EventHandler};
pub use keybinds::{Action, KeyMap};
pub use themes::Theme;

use anyhow::Result;

/// Initialize and run the TUI application
pub fn run() -> Result<()> {
    // TODO: Implement TUI initialization and event loop
    // - Setup terminal (enter alternate screen, enable raw mode)
    // - Create App state
    // - Create EventHandler
    // - Run main event loop
    // - Cleanup terminal on exit
    todo!("Implement TUI run loop")
}
