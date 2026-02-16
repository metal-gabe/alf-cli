//! State management modules

pub mod data;
pub mod filter;
pub mod input;
pub mod navigation;
pub mod scroll;
pub mod search;
pub mod ui;

// Re-export commonly used types for convenience
pub use data::EntryData;
pub use filter::{EntryFilter, FilterState, GroupMode, SortOrder};
pub use input::{InputMode, InputState};
pub use navigation::NavigationState;
pub use scroll::ScrollManager;
pub use search::SearchState;
pub use ui::{Panel, UiState};
