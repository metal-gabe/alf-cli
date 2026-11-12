//! Application state management for the TUI.

use super::themes::Theme;
use crate::config::ParseComments;
use crate::models::AliasEntry;

/// Action to take when the user selects an entry
#[derive(Debug, Clone, Copy)]
pub enum ExitAction {
   /// Populate shell with entry and execute it immediately
   Execute,
   /// Populate shell with entry but do not execute
   Populate,
}

/// Whether the application is still running, and how it ended if not
#[derive(Debug, Clone, Copy)]
enum RunState {
   /// The event loop should keep going
   Running,
   /// Exit without selecting an entry
   Quit,
   /// Exit after selecting an entry with the given action
   Selected(ExitAction),
}

use super::state::{
   EntryData, EntryFilter, FilterState, GroupMode, InputMode, InputState, NavigationState, Panel, ScrollManager,
   SearchState, SortOrder, UiState,
};

/// Main application state
pub struct App {
   /// Entry data storage
   data: EntryData,
   /// Search state
   search: SearchState,
   /// UI state (panels, scroll offsets, help modal)
   ui: UiState,
   /// Input state (mode, pending keys)
   input: InputState,
   /// Navigation state (selection)
   navigation: NavigationState,
   /// Filter/grouping/sorting state
   filter: FilterState,
   /// Color theme
   theme: Theme,
   /// Whether the application is running or how it exited
   run_state: RunState,
   /// When the description panel is shown
   parse_comments: ParseComments,
}

impl App {
   /// Create a new App instance with the given entries and theme
   pub fn new(
      entries: Vec<AliasEntry>,
      theme: Theme,
   ) -> Self {
      let mut app = Self {
         data: EntryData::new(entries),
         search: SearchState::new(),
         ui: UiState::default(),
         input: InputState::default(),
         navigation: NavigationState::default(),
         filter: FilterState::default(),
         theme,
         run_state: RunState::Running,
         parse_comments: ParseComments::default(),
      };

      app.update_visible_entries();

      app
   }

   /// Get the currently selected entry, if any
   pub fn selected_entry(&self) -> Option<&AliasEntry> {
      self.data.get_visible_entry(self.navigation.selected_index())
   }

   /// Update visible entries based on current filter and search query
   pub fn update_visible_entries(&mut self) {
      self.filter.update_visible_entries(&mut self.data, self.search.query());
      self.navigation.clamp(self.data.visible_count());
      self.ui.reset_detail_scroll();
   }

   /// Get reference to all entries
   pub fn entries(&self) -> &[AliasEntry] {
      self.data.entries()
   }

   /// Get reference to visible indices
   pub fn visible_indices(&self) -> &[usize] {
      self.data.visible_indices()
   }

   /// Get the search query
   pub fn search_query(&self) -> &str {
      self.search.query()
   }

   /// Get the cursor position
   pub fn cursor_position(&self) -> usize {
      self.search.cursor_position()
   }

   /// Get the selected index
   pub fn selected_index(&self) -> usize {
      self.navigation.selected_index()
   }

   /// Get the input mode
   pub fn input_mode(&self) -> InputMode {
      self.input.mode()
   }

   /// Get the active panel
   pub fn active_panel(&self) -> Panel {
      self.ui.active_panel()
   }

   /// Whether the description panel is shown for the currently selected entry
   pub fn is_description_visible(&self) -> bool {
      let does_have_comments = self.selected_entry().is_some_and(|entry| entry.comments.is_some());
      self.parse_comments.is_description_visible(does_have_comments)
   }

   /// Get when the description panel is shown
   pub fn parse_comments(&self) -> ParseComments {
      self.parse_comments
   }

   /// Cycle when the description panel is shown for this session only: `"dynamic"` → `true` → `false`
   pub fn cycle_parse_comments(&mut self) {
      self.parse_comments = self.parse_comments.next();
   }

   /// Set when the description panel is shown
   pub fn set_parse_comments(
      &mut self,
      parse_comments: ParseComments,
   ) {
      self.parse_comments = parse_comments;
   }

   /// Move focus off the description panel while it is hidden, onto the script panel that takes its place
   pub fn sync_active_panel(&mut self) {
      if self.ui.active_panel() == Panel::Description && !self.is_description_visible() {
         self.ui.set_active_panel(Panel::Script);
      }
   }

   /// Get the fullscreen flag
   pub fn is_fullscreen(&self) -> bool {
      self.ui.is_fullscreen()
   }

   /// Get the current filter
   pub fn filter(&self) -> EntryFilter {
      self.filter.filter()
   }

   /// Get description scroll offset
   pub fn description_scroll_offset(&self) -> usize {
      self.ui.description_scroll_offset()
   }

   /// Get description max scroll
   pub fn description_max_scroll(&self) -> usize {
      self.ui.description_max_scroll()
   }

   /// Get script scroll offset
   pub fn script_scroll_offset(&self) -> usize {
      self.ui.script_scroll_offset()
   }

   /// Get script max scroll
   pub fn script_max_scroll(&self) -> usize {
      self.ui.script_max_scroll()
   }

   /// Get pending key
   pub fn pending_key(&self) -> Option<char> {
      self.input.pending_key()
   }

   /// Get pending key time
   pub fn pending_key_time(&self) -> Option<std::time::Instant> {
      self.input.pending_key_time()
   }

   /// Get show help flag
   pub fn is_help_visible(&self) -> bool {
      self.ui.is_help_visible()
   }

   /// Get help scroll offset
   pub fn help_scroll_offset(&self) -> usize {
      self.ui.help_scroll_offset()
   }

   /// Get help max scroll
   pub fn help_max_scroll(&self) -> usize {
      self.ui.help_max_scroll()
   }

   /// Get group mode
   pub fn group_mode(&self) -> GroupMode {
      self.filter.group_mode()
   }

   /// Get sort order
   pub fn sort_order(&self) -> SortOrder {
      self.filter.sort_order()
   }

   /// Move selection up by one
   pub fn move_up(&mut self) {
      self.navigation.move_up();
      self.ui.reset_detail_scroll();
   }

   /// Move selection down by one
   pub fn move_down(&mut self) {
      self.navigation.move_down(self.data.visible_count());
      self.ui.reset_detail_scroll();
   }

   /// Jump to the top of the active panel
   pub fn move_top(&mut self) {
      ScrollManager::move_top(&mut self.ui, &mut self.navigation);
   }

   /// Jump to the bottom of the active panel
   pub fn move_bottom(&mut self) {
      ScrollManager::move_bottom(&mut self.ui, &mut self.navigation, self.data.visible_count());
   }

   /// Scroll the active panel up
   pub fn scroll_up(
      &mut self,
      amount: usize,
   ) {
      ScrollManager::scroll_up(&mut self.ui, &mut self.navigation, amount);
   }

   /// Scroll the active panel down
   pub fn scroll_down(
      &mut self,
      amount: usize,
   ) {
      ScrollManager::scroll_down(&mut self.ui, &mut self.navigation, amount, self.data.visible_count());
   }

   /// Cycle to the next panel (forward)
   pub fn cycle_panel(&mut self) {
      self.ui.cycle_panel();
      if self.ui.active_panel() == Panel::Description && !self.is_description_visible() {
         self.ui.cycle_panel();
      }
   }

   /// Cycle to the previous panel (backward)
   pub fn cycle_panel_backward(&mut self) {
      self.ui.cycle_panel_backward();
      if self.ui.active_panel() == Panel::Description && !self.is_description_visible() {
         self.ui.cycle_panel_backward();
      }
   }

   /// Toggle fullscreen for the active panel
   pub fn toggle_fullscreen(&mut self) {
      self.ui.toggle_fullscreen();
   }

   /// Cycle the entry type filter (forward)
   pub fn cycle_filter(&mut self) {
      self.filter.cycle_filter();
      self.update_visible_entries();
   }

   /// Cycle the entry type filter (backward)
   pub fn cycle_filter_backward(&mut self) {
      self.filter.cycle_filter_backward();
      self.update_visible_entries();
   }

   /// Set a specific filter
   pub fn set_filter(
      &mut self,
      filter: EntryFilter,
   ) {
      self.filter.set_filter(filter);
      self.update_visible_entries();
   }

   /// Enter search mode
   pub fn enter_search_mode(&mut self) {
      self.input.enter_search();
   }

   /// Exit search mode, keeping the current query
   pub fn exit_search_keep_query(&mut self) {
      self.input.exit_search();
   }

   /// Exit search mode, clearing the query
   pub fn exit_search_clear_query(&mut self) {
      self.input.exit_search();
      self.search.clear();
      self.update_visible_entries();
   }

   /// Clear the search query without changing mode
   pub fn clear_search(&mut self) {
      self.search.clear();
      self.update_visible_entries();
   }

   /// Set the search query directly and update visible entries
   pub fn set_search_query(
      &mut self,
      query: String,
   ) {
      self.search.set_query(query);
      self.update_visible_entries();
   }

   /// Insert a character at the cursor position in the search query
   pub fn search_insert_char(
      &mut self,
      c: char,
   ) {
      self.search.insert_char(c);
      self.update_visible_entries();
   }

   /// Delete the character before the cursor in the search query
   pub fn search_delete_char(&mut self) {
      self.search.delete_char();
      self.update_visible_entries();
   }

   /// Move search cursor left
   pub fn search_cursor_left(&mut self) {
      self.search.move_cursor_left();
   }

   /// Move search cursor right
   pub fn search_cursor_right(&mut self) {
      self.search.move_cursor_right();
   }

   /// Delete the character under the cursor in the search query
   pub fn search_delete_char_forward(&mut self) {
      self.search.delete_char_forward();
      self.update_visible_entries();
   }

   /// Delete the word before the cursor in the search query
   pub fn search_delete_word_backward(&mut self) {
      self.search.delete_word_backward();
      self.update_visible_entries();
   }

   /// Delete the word after the cursor in the search query
   pub fn search_delete_word_forward(&mut self) {
      self.search.delete_word_forward();
      self.update_visible_entries();
   }

   /// Move search cursor to the end of the query
   pub fn search_cursor_end(&mut self) {
      self.search.move_cursor_end();
   }

   /// Move search cursor to the start of the query
   pub fn search_cursor_start(&mut self) {
      self.search.move_cursor_start();
   }

   /// Move search cursor one word left
   pub fn search_cursor_word_left(&mut self) {
      self.search.move_word_backward();
   }

   /// Move search cursor one word right
   pub fn search_cursor_word_right(&mut self) {
      self.search.move_word_forward();
   }

   /// Toggle the help modal
   pub fn toggle_help(&mut self) {
      self.ui.toggle_help();
   }

   /// Scroll help modal down by one line
   pub fn help_scroll_down(&mut self) {
      ScrollManager::help_scroll_down(&mut self.ui);
   }

   /// Scroll help modal up by one line
   pub fn help_scroll_up(&mut self) {
      ScrollManager::help_scroll_up(&mut self.ui);
   }

   /// Jump to the top of the help modal
   pub fn help_jump_top(&mut self) {
      ScrollManager::help_jump_top(&mut self.ui);
   }

   /// Jump to the bottom of the help modal
   pub fn help_jump_bottom(&mut self) {
      ScrollManager::help_jump_bottom(&mut self.ui);
   }

   /// Update the maximum scroll offset for help modal based on content and visible area
   pub fn update_help_max_scroll(
      &mut self,
      total_lines: usize,
      visible_lines: usize,
   ) {
      self.ui.update_help_max_scroll(total_lines, visible_lines);
   }

   /// Update the maximum scroll offset for description panel based on content and visible area
   pub fn update_description_max_scroll(
      &mut self,
      total_lines: usize,
      visible_lines: usize,
   ) {
      self.ui.update_description_max_scroll(total_lines, visible_lines);
   }

   /// Update the maximum scroll offset for script panel based on content and visible area
   pub fn update_script_max_scroll(
      &mut self,
      total_lines: usize,
      visible_lines: usize,
   ) {
      self.ui.update_script_max_scroll(total_lines, visible_lines);
   }

   /// Cycle to the next group mode
   pub fn cycle_group_mode(&mut self) {
      self.filter.cycle_group_mode();
      self.update_visible_entries();
   }

   /// Cycle to the previous group mode
   pub fn cycle_group_mode_backward(&mut self) {
      self.filter.cycle_group_mode_backward();
      self.update_visible_entries();
   }

   /// Toggle sort order
   pub fn toggle_sort_order(&mut self) {
      self.filter.toggle_sort_order();
      self.update_visible_entries();
   }

   /// Check if pending key has timed out (2 seconds)
   pub fn is_pending_key_expired(&self) -> bool {
      self.input.is_pending_key_expired()
   }

   /// Clear pending key state
   pub fn clear_pending_key(&mut self) {
      self.input.clear_pending_key();
   }

   /// Set pending key with timestamp
   pub fn set_pending_key(
      &mut self,
      key: char,
   ) {
      self.input.set_pending_key(key);
   }

   /// Select the current entry with the given action and exit gracefully
   pub fn select_entry(
      &mut self,
      action: ExitAction,
   ) {
      self.run_state = if self.selected_entry().is_some() { RunState::Selected(action) } else { RunState::Quit };
   }

   /// Exit without selecting an entry
   pub fn quit(&mut self) {
      self.run_state = RunState::Quit;
   }

   /// Whether the event loop should keep going
   pub fn is_running(&self) -> bool {
      matches!(self.run_state, RunState::Running)
   }

   /// The action chosen when an entry was selected, if any
   pub fn exit_action(&self) -> Option<ExitAction> {
      match self.run_state {
         RunState::Selected(action) => Some(action),
         RunState::Quit | RunState::Running => None,
      }
   }

   /// Update application state (called each tick)
   pub fn tick(&mut self) {
      if self.input.is_pending_key_expired() {
         self.input.clear_pending_key();
      }
   }

   /// Cycle to the next theme in alphabetical order (wraps around)
   pub fn cycle_theme_next(&mut self) {
      let themes = Theme::available_themes();
      let current = themes.iter().position(|n| n == &self.theme.name).unwrap_or(0);
      let next = (current + 1) % themes.len();
      if let Some(t) = Theme::from_name(&themes[next]) {
         self.theme = t;
      }
   }

   /// Cycle to the previous theme in alphabetical order (wraps around)
   pub fn cycle_theme_prev(&mut self) {
      let themes = Theme::available_themes();
      let current = themes.iter().position(|n| n == &self.theme.name).unwrap_or(0);
      let prev = (current + themes.len() - 1) % themes.len();
      if let Some(t) = Theme::from_name(&themes[prev]) {
         self.theme = t;
      }
   }

   /// Get reference to the current theme
   pub fn theme(&self) -> &Theme {
      &self.theme
   }
}

#[cfg(test)]
#[path = "app_tests.rs"]
pub(crate) mod app_tests;
