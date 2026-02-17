//! UI state management

/// Active panel for scroll context
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Panel {
   /// Left panel: alias/function list
   List,
   /// Right-top panel: comments/description
   Description,
   /// Right-bottom panel: script/function body
   Script,
}

/// UI state (panels, scroll offsets, help modal)
#[derive(Debug, Clone)]
pub struct UiState {
   /// Currently active panel (receives scroll commands)
   active_panel: Panel,
   /// Scroll offset for the list panel
   list_scroll_offset: usize,
   /// Scroll offset for the description panel
   description_scroll_offset: usize,
   /// Maximum scroll offset for description panel (updated during rendering)
   description_max_scroll: usize,
   /// Scroll offset for the script panel
   script_scroll_offset: usize,
   /// Maximum scroll offset for script panel (updated during rendering)
   script_max_scroll: usize,
   /// Flag to show/hide the help modal
   show_help: bool,
   /// Scroll offset for the help modal
   help_scroll_offset: usize,
   /// Maximum scroll offset for help modal (updated during rendering)
   help_max_scroll: usize,
}

impl Default for UiState {
   fn default() -> Self {
      Self {
         active_panel: Panel::List,
         list_scroll_offset: 0,
         description_scroll_offset: 0,
         description_max_scroll: 0,
         script_scroll_offset: 0,
         script_max_scroll: 0,
         show_help: false,
         help_scroll_offset: 0,
         help_max_scroll: 0,
      }
   }
}

impl UiState {
   /// Create a new UiState
   pub fn new() -> Self {
      Self::default()
   }

   /// Get the active panel
   pub fn active_panel(&self) -> Panel {
      self.active_panel
   }

   /// Get list scroll offset
   pub fn list_scroll_offset(&self) -> usize {
      self.list_scroll_offset
   }

   /// Get description scroll offset
   pub fn description_scroll_offset(&self) -> usize {
      self.description_scroll_offset
   }

   /// Get description max scroll
   pub fn description_max_scroll(&self) -> usize {
      self.description_max_scroll
   }

   /// Get script scroll offset
   pub fn script_scroll_offset(&self) -> usize {
      self.script_scroll_offset
   }

   /// Get script max scroll
   pub fn script_max_scroll(&self) -> usize {
      self.script_max_scroll
   }

   /// Get show help flag
   pub fn show_help(&self) -> bool {
      self.show_help
   }

   /// Get help scroll offset
   pub fn help_scroll_offset(&self) -> usize {
      self.help_scroll_offset
   }

   /// Get help max scroll
   pub fn help_max_scroll(&self) -> usize {
      self.help_max_scroll
   }

   /// Set list scroll offset
   pub fn set_list_scroll_offset(&mut self, offset: usize) {
      self.list_scroll_offset = offset;
   }

   /// Set description scroll offset
   pub fn set_description_scroll_offset(&mut self, offset: usize) {
      self.description_scroll_offset = offset;
   }

   /// Set script scroll offset
   pub fn set_script_scroll_offset(&mut self, offset: usize) {
      self.script_scroll_offset = offset;
   }

   /// Set help scroll offset
   pub fn set_help_scroll_offset(&mut self, offset: usize) {
      self.help_scroll_offset = offset;
   }

   /// Cycle to the next panel (forward)
   pub fn cycle_panel(&mut self) {
      self.active_panel = match self.active_panel {
         Panel::List => Panel::Description,
         Panel::Description => Panel::Script,
         Panel::Script => Panel::List,
      };
   }

   /// Cycle to the previous panel (backward)
   pub fn cycle_panel_backward(&mut self) {
      self.active_panel = match self.active_panel {
         Panel::List => Panel::Script,
         Panel::Script => Panel::Description,
         Panel::Description => Panel::List,
      };
   }

   /// Toggle the help modal
   pub fn toggle_help(&mut self) {
      self.show_help = !self.show_help;
      // Reset scroll position when opening help
      if self.show_help {
         self.help_scroll_offset = 0;
      }
   }

   /// Reset detail panel scroll offsets (when selection changes)
   pub fn reset_detail_scroll(&mut self) {
      self.description_scroll_offset = 0;
      self.script_scroll_offset = 0;
   }

   /// Update the maximum scroll offset for description panel based on content and visible area
   pub fn update_description_max_scroll(&mut self, total_lines: usize, visible_lines: usize) {
      self.description_max_scroll =
         if total_lines > visible_lines { total_lines.saturating_sub(visible_lines) } else { 0 };
   }

   /// Update the maximum scroll offset for script panel based on content and visible area
   pub fn update_script_max_scroll(&mut self, total_lines: usize, visible_lines: usize) {
      self.script_max_scroll = if total_lines > visible_lines { total_lines.saturating_sub(visible_lines) } else { 0 };
   }

   /// Update the maximum scroll offset for help modal based on content and visible area
   pub fn update_help_max_scroll(&mut self, total_lines: usize, visible_lines: usize) {
      self.help_max_scroll = if total_lines > visible_lines { total_lines.saturating_sub(visible_lines) } else { 0 };
   }
}
