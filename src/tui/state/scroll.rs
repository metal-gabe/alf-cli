//! Scroll management for all panels

use super::navigation::NavigationState;
use super::ui::{Panel, UiState};

/// Scroll manager - handles scroll operations across all panels
pub struct ScrollManager;

impl ScrollManager {
   /// Scroll the active panel up
   pub fn scroll_up(ui: &mut UiState, navigation: &mut NavigationState, amount: usize) {
      match ui.active_panel() {
         Panel::List => {
            navigation.scroll_up(amount);
            ui.reset_detail_scroll();
         }
         Panel::Description => {
            let new_offset = ui.description_scroll_offset().saturating_sub(amount);
            ui.set_description_scroll_offset(new_offset);
         }
         Panel::Script => {
            let new_offset = ui.script_scroll_offset().saturating_sub(amount);
            ui.set_script_scroll_offset(new_offset);
         }
      }
   }

   /// Scroll the active panel down
   pub fn scroll_down(ui: &mut UiState, navigation: &mut NavigationState, amount: usize, visible_count: usize) {
      match ui.active_panel() {
         Panel::List => {
            navigation.scroll_down(amount, visible_count);
            ui.reset_detail_scroll();
         }
         Panel::Description => {
            let new_offset = ui.description_scroll_offset() + amount;
            let clamped = new_offset.min(ui.description_max_scroll());
            ui.set_description_scroll_offset(clamped);
         }
         Panel::Script => {
            let new_offset = ui.script_scroll_offset() + amount;
            let clamped = new_offset.min(ui.script_max_scroll());
            ui.set_script_scroll_offset(clamped);
         }
      }
   }

   /// Jump to top of active panel
   pub fn move_top(ui: &mut UiState, navigation: &mut NavigationState) {
      match ui.active_panel() {
         Panel::List => {
            navigation.move_top();
            ui.set_list_scroll_offset(0);
            ui.reset_detail_scroll();
         }
         Panel::Description => {
            ui.set_description_scroll_offset(0);
         }
         Panel::Script => {
            ui.set_script_scroll_offset(0);
         }
      }
   }

   /// Jump to bottom of active panel
   pub fn move_bottom(ui: &mut UiState, navigation: &mut NavigationState, visible_count: usize) {
      match ui.active_panel() {
         Panel::List => {
            navigation.move_bottom(visible_count);
            ui.reset_detail_scroll();
         }
         Panel::Description => {
            ui.set_description_scroll_offset(ui.description_max_scroll());
         }
         Panel::Script => {
            ui.set_script_scroll_offset(ui.script_max_scroll());
         }
      }
   }

   /// Scroll help modal up
   pub fn help_scroll_up(ui: &mut UiState) {
      let new_offset = ui.help_scroll_offset().saturating_sub(1);
      ui.set_help_scroll_offset(new_offset);
   }

   /// Scroll help modal down
   pub fn help_scroll_down(ui: &mut UiState) {
      let current = ui.help_scroll_offset();
      let max = ui.help_max_scroll();
      if current < max {
         ui.set_help_scroll_offset(current + 1);
      }
   }

   /// Jump to top of help modal
   pub fn help_jump_top(ui: &mut UiState) {
      ui.set_help_scroll_offset(0);
   }

   /// Jump to bottom of help modal
   pub fn help_jump_bottom(ui: &mut UiState) {
      ui.set_help_scroll_offset(ui.help_max_scroll());
   }
}
