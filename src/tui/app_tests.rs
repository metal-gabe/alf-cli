//! Tests for the App state manager

pub use crate::models::{AliasEntry, EntryType};
pub use super::{App, ExitAction};
pub use crate::tui::state::{EntryFilter, GroupMode, InputMode, Panel, SortOrder};
pub use crate::tui::themes::Theme;
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

#[path = "app_tests/filter.rs"]
mod filter;
#[path = "app_tests/group_mode_and_sort_order.rs"]
mod group_mode_and_sort_order;
#[path = "app_tests/help_modal.rs"]
mod help_modal;
#[path = "app_tests/initial_state.rs"]
mod initial_state;
#[path = "app_tests/navigation.rs"]
mod navigation;
#[path = "app_tests/panel_cycling.rs"]
mod panel_cycling;
#[path = "app_tests/pending_key.rs"]
mod pending_key;
#[path = "app_tests/scroll.rs"]
mod scroll;
#[path = "app_tests/search_mode.rs"]
mod search_mode;
#[path = "app_tests/select_entry.rs"]
mod select_entry;
#[path = "app_tests/selected_entry.rs"]
mod selected_entry;
#[path = "app_tests/theme_cycling.rs"]
mod theme_cycling;
#[path = "app_tests/tick.rs"]
mod tick;
