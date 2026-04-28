//! Integration tests for the App state manager

pub use alf::models::{AliasEntry, EntryType};
pub use alf::tui::app::{App, ExitAction};
pub use alf::tui::state::{EntryFilter, GroupMode, InputMode, Panel, SortOrder};
pub use alf::tui::Theme;
use std::path::PathBuf;

pub fn make_entry(name: &str, entry_type: EntryType, value: &str) -> AliasEntry {
   AliasEntry {
      name: name.to_string(),
      entry_type,
      value: value.to_string(),
      comments: None,
      source_file: PathBuf::from("test.sh"),
   }
}

/// Creates an App with 2 aliases and 2 functions.
/// Default ordering (GroupMode::Aliases first, A-Z):
///   alpha (Alias), beta (Alias), gamma (Function), delta (Function)
/// Wait - sorted A-Z within groups: alpha, beta for aliases; delta, gamma for functions
pub fn make_app() -> App {
   let entries = vec![
      make_entry("beta", EntryType::Alias, "cmd_beta"),
      make_entry("alpha", EntryType::Alias, "cmd_alpha"),
      make_entry("gamma", EntryType::Function, "{ body_gamma }"),
      make_entry("delta", EntryType::Function, "{ body_delta }"),
   ];
   App::new(entries, Theme::default())
}

pub fn make_empty_app() -> App {
   App::new(vec![], Theme::default())
}

mod filter;
mod group_mode_and_sort_order;
mod help_modal;
mod initial_state;
mod navigation;
mod panel_cycling;
mod pending_key;
mod scroll;
mod search_mode;
mod select_entry;
mod selected_entry;
mod theme_cycling;
mod tick;
