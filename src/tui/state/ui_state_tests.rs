//! Tests for UiState and ScrollManager

use super::{Panel, UiState};
use crate::tui::state::{NavigationState, ScrollManager};

type CycleCase = (fn(&mut UiState), [Panel; 3]);
type ScrollArea = (&'static str, fn(&mut UiState, usize, usize), fn(&UiState) -> usize);

/// The scroll accessors of a detail panel, so one table can drive both panels
struct DetailPanel {
   max: fn(&UiState) -> usize,
   offset: fn(&UiState) -> usize,
   panel: Panel,
   set_offset: fn(&mut UiState, usize),
   update_max: fn(&mut UiState, usize, usize),
}

const DESCRIPTION: DetailPanel = DetailPanel {
   max: UiState::description_max_scroll,
   offset: UiState::description_scroll_offset,
   panel: Panel::Description,
   set_offset: UiState::set_description_scroll_offset,
   update_max: UiState::update_description_max_scroll,
};

const SCRIPT: DetailPanel = DetailPanel {
   max: UiState::script_max_scroll,
   offset: UiState::script_scroll_offset,
   panel: Panel::Script,
   set_offset: UiState::set_script_scroll_offset,
   update_max: UiState::update_script_max_scroll,
};

fn ui_on(panel: Panel) -> UiState {
   let mut ui = UiState::new();
   while ui.active_panel() != panel {
      ui.cycle_panel();
   }
   ui
}

// ===== UiState defaults =====

#[test]
fn ui_state_default_panel_is_list() {
   let ui = UiState::new();
   assert_eq!(ui.active_panel(), Panel::List);
}

#[test]
fn ui_state_default_all_offsets_zero() {
   let ui = UiState::new();
   assert_eq!(ui.description_scroll_offset(), 0);
   assert_eq!(ui.script_scroll_offset(), 0);
   assert_eq!(ui.help_scroll_offset(), 0);
}

#[test]
fn ui_state_default_help_hidden() {
   let ui = UiState::new();
   assert!(!ui.is_help_visible());
}

#[test]
fn ui_state_default_fullscreen_off() {
   let ui = UiState::new();
   assert!(!ui.is_fullscreen());
}

// ===== Panel cycling =====

#[test]
fn cycle_panel_visits_panels_in_order_and_wraps() {
   let cases: [CycleCase; 2] = [
      (UiState::cycle_panel, [Panel::Description, Panel::Script, Panel::List]),
      (UiState::cycle_panel_backward, [Panel::Script, Panel::Description, Panel::List]),
   ];
   for (cycle, expected_order) in cases {
      let mut ui = UiState::new();
      for expected in expected_order {
         cycle(&mut ui);
         assert_eq!(ui.active_panel(), expected);
      }
   }
}

// ===== Fullscreen =====

#[test]
fn toggle_fullscreen_alternates_on_each_call() {
   let mut ui = UiState::new();
   for toggle_count in 1..=4 {
      ui.toggle_fullscreen();
      assert_eq!(ui.is_fullscreen(), toggle_count % 2 == 1, "After {toggle_count} toggle(s)");
   }
}

#[test]
fn toggle_fullscreen_does_not_change_active_panel() {
   let mut ui = UiState::new();
   ui.cycle_panel();
   ui.toggle_fullscreen();
   assert_eq!(ui.active_panel(), Panel::Description);
}

// ===== Help modal =====

#[test]
fn toggle_help_opens_modal() {
   let mut ui = UiState::new();
   ui.toggle_help();
   assert!(ui.is_help_visible());
}

#[test]
fn toggle_help_closes_modal() {
   let mut ui = UiState::new();
   ui.toggle_help();
   ui.toggle_help();
   assert!(!ui.is_help_visible());
}

#[test]
fn toggle_help_resets_scroll_when_opening() {
   let mut ui = UiState::new();
   ui.update_help_max_scroll(20, 5);
   ui.set_help_scroll_offset(3);
   ui.toggle_help();
   assert_eq!(ui.help_scroll_offset(), 0, "Opening help should reset scroll");
}

#[test]
fn toggle_help_does_not_reset_scroll_when_closing() {
   let mut ui = UiState::new();
   ui.toggle_help();
   ui.update_help_max_scroll(20, 5);
   ui.set_help_scroll_offset(3);
   ui.toggle_help();
   assert_eq!(ui.help_scroll_offset(), 3, "Closing help should preserve scroll offset");
}

// ===== reset_detail_scroll =====

#[test]
fn reset_detail_scroll_zeroes_detail_offsets() {
   for detail in [DESCRIPTION, SCRIPT] {
      let mut ui = UiState::new();
      (detail.set_offset)(&mut ui, 5);
      ui.reset_detail_scroll();
      assert_eq!((detail.offset)(&ui), 0, "{:?}", detail.panel);
   }
}

// ===== Max scroll updates =====

#[test]
fn update_max_scroll_is_the_overflow_for_every_scrollable_area() {
   let areas: [ScrollArea; 3] = [
      ("description", UiState::update_description_max_scroll, UiState::description_max_scroll),
      ("script", UiState::update_script_max_scroll, UiState::script_max_scroll),
      ("help", UiState::update_help_max_scroll, UiState::help_max_scroll),
   ];
   for (label, update, max) in areas {
      for (total, visible, expected) in [(20, 10, 10), (10, 10, 0), (5, 10, 0)] {
         let mut ui = UiState::new();
         update(&mut ui, total, visible);
         assert_eq!(max(&ui), expected, "{label} with {total} total and {visible} visible");
      }
   }
}

#[test]
fn update_help_max_scroll_clamps_the_offset() {
   let mut ui = UiState::new();
   ui.update_help_max_scroll(30, 10);
   ui.set_help_scroll_offset(15);
   ui.update_help_max_scroll(20, 10);
   assert_eq!(ui.help_scroll_offset(), 10);
   ui.update_help_max_scroll(5, 10);
   assert_eq!(ui.help_scroll_offset(), 0);
}

// ===== ScrollManager: scroll_up on panels =====

#[test]
fn scroll_up_on_list_panel_moves_navigation() {
   let mut ui = UiState::new();
   let mut nav = NavigationState::new();
   nav.scroll_down(5, 10);
   assert_eq!(nav.selected_index(), 5);
   ScrollManager::scroll_up(&mut ui, &mut nav, 2);
   assert_eq!(nav.selected_index(), 3);
}

#[test]
fn scroll_up_on_detail_panels_decrements_and_saturates() {
   for (detail, start, amount, expected) in [(DESCRIPTION, 5, 2, 3), (DESCRIPTION, 1, 10, 0), (SCRIPT, 8, 3, 5)] {
      let mut ui = ui_on(detail.panel);
      let mut nav = NavigationState::new();
      (detail.set_offset)(&mut ui, start);
      ScrollManager::scroll_up(&mut ui, &mut nav, amount);
      assert_eq!((detail.offset)(&ui), expected, "{:?} from {start} by {amount}", detail.panel);
   }
}

// ===== ScrollManager: scroll_down on panels =====

#[test]
fn scroll_down_on_list_panel_moves_navigation() {
   let mut ui = UiState::new();
   let mut nav = NavigationState::new();
   ScrollManager::scroll_down(&mut ui, &mut nav, 3, 10);
   assert_eq!(nav.selected_index(), 3);
}

#[test]
fn scroll_down_on_detail_panels_increments_and_clamps() {
   for (detail, total, visible, amount, expected) in
      [(DESCRIPTION, 20, 5, 3, 3), (DESCRIPTION, 20, 15, 100, 5), (SCRIPT, 20, 5, 4, 4)]
   {
      let mut ui = ui_on(detail.panel);
      let mut nav = NavigationState::new();
      (detail.update_max)(&mut ui, total, visible);
      ScrollManager::scroll_down(&mut ui, &mut nav, amount, 10);
      assert_eq!((detail.offset)(&ui), expected, "{:?} by {amount}", detail.panel);
   }
}

// ===== ScrollManager: move_top / move_bottom =====

#[test]
fn move_top_on_list_resets_navigation() {
   let mut ui = UiState::new();
   let mut nav = NavigationState::new();
   nav.move_bottom(10);
   ScrollManager::move_top(&mut ui, &mut nav);
   assert_eq!(nav.selected_index(), 0);
}

#[test]
fn move_top_on_detail_panels_zeroes_offset() {
   for detail in [DESCRIPTION, SCRIPT] {
      let mut ui = ui_on(detail.panel);
      let mut nav = NavigationState::new();
      (detail.set_offset)(&mut ui, 7);
      ScrollManager::move_top(&mut ui, &mut nav);
      assert_eq!((detail.offset)(&ui), 0, "{:?}", detail.panel);
   }
}

#[test]
fn move_bottom_on_list_moves_navigation_to_last() {
   let mut ui = UiState::new();
   let mut nav = NavigationState::new();
   ScrollManager::move_bottom(&mut ui, &mut nav, 10);
   assert_eq!(nav.selected_index(), 9);
}

#[test]
fn move_bottom_on_detail_panels_sets_max() {
   for (detail, total, visible, expected) in [(DESCRIPTION, 20, 5, 15), (SCRIPT, 30, 10, 20)] {
      let mut ui = ui_on(detail.panel);
      let mut nav = NavigationState::new();
      (detail.update_max)(&mut ui, total, visible);
      ScrollManager::move_bottom(&mut ui, &mut nav, 10);
      assert_eq!((detail.max)(&ui), expected, "{:?}", detail.panel);
      assert_eq!((detail.offset)(&ui), expected, "{:?}", detail.panel);
   }
}

// ===== ScrollManager: help modal =====

#[test]
fn help_scroll_up_decrements_offset() {
   let mut ui = UiState::new();
   ui.update_help_max_scroll(20, 5);
   ui.set_help_scroll_offset(5);
   ScrollManager::help_scroll_up(&mut ui);
   assert_eq!(ui.help_scroll_offset(), 4);
}

#[test]
fn help_scroll_up_saturates_at_zero() {
   let mut ui = UiState::new();
   ScrollManager::help_scroll_up(&mut ui);
   assert_eq!(ui.help_scroll_offset(), 0);
}

#[test]
fn help_scroll_down_increments_offset() {
   let mut ui = UiState::new();
   ui.update_help_max_scroll(20, 5);
   ScrollManager::help_scroll_down(&mut ui);
   assert_eq!(ui.help_scroll_offset(), 1);
}

#[test]
fn help_scroll_down_clamped_to_max() {
   let mut ui = UiState::new();
   ui.update_help_max_scroll(20, 15);
   ui.set_help_scroll_offset(5);
   ScrollManager::help_scroll_down(&mut ui);
   assert_eq!(ui.help_scroll_offset(), 5, "Should not exceed max");
}

#[test]
fn help_jump_top_sets_offset_to_zero() {
   let mut ui = UiState::new();
   ui.set_help_scroll_offset(10);
   ScrollManager::help_jump_top(&mut ui);
   assert_eq!(ui.help_scroll_offset(), 0);
}

#[test]
fn help_jump_bottom_sets_offset_to_max() {
   let mut ui = UiState::new();
   ui.update_help_max_scroll(30, 10);
   ScrollManager::help_jump_bottom(&mut ui);
   assert_eq!(ui.help_scroll_offset(), 20);
}
