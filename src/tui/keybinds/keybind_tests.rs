//! Tests for the keybind dispatch layer

use super::handle_key_event;
use crate::tui::app::app_tests::{make_app, make_empty_app};
use crate::tui::app::ExitAction;
use crate::tui::state::{EntryFilter, InputMode, Panel, SortOrder};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

fn key(code: KeyCode) -> KeyEvent {
   KeyEvent::new(code, KeyModifiers::NONE)
}

fn ctrl(c: char) -> KeyEvent {
   KeyEvent::new(KeyCode::Char(c), KeyModifiers::CONTROL)
}

// ===== Normal mode — Enter/Tab (enter-tab logic) =====

#[test]
fn test_enter_in_normal_mode_sets_execute_action() {
   let mut app = make_app();
   handle_key_event(&mut app, key(KeyCode::Enter));
   assert!(app.should_quit);
   assert!(matches!(app.exit_action, Some(ExitAction::Execute)));
}

#[test]
fn test_tab_in_normal_mode_sets_populate_action() {
   let mut app = make_app();
   handle_key_event(&mut app, key(KeyCode::Tab));
   assert!(app.should_quit);
   assert!(matches!(app.exit_action, Some(ExitAction::Populate)));
}

#[test]
fn test_enter_in_normal_mode_on_empty_app_quits_with_no_action() {
   let mut app = make_empty_app();
   handle_key_event(&mut app, key(KeyCode::Enter));
   assert!(app.should_quit);
   assert!(app.exit_action.is_none());
}

#[test]
fn test_tab_in_normal_mode_on_empty_app_quits_with_no_action() {
   let mut app = make_empty_app();
   handle_key_event(&mut app, key(KeyCode::Tab));
   assert!(app.should_quit);
   assert!(app.exit_action.is_none());
}

// ===== Normal mode — navigation =====

#[test]
fn test_j_in_normal_mode_moves_selection_down() {
   let mut app = make_app();
   let before = app.selected_index();
   handle_key_event(&mut app, key(KeyCode::Char('j')));
   assert!(app.selected_index() > before);
}

#[test]
fn test_down_arrow_moves_selection_down() {
   let mut app = make_app();
   let before = app.selected_index();
   handle_key_event(&mut app, key(KeyCode::Down));
   assert!(app.selected_index() > before);
}

#[test]
fn test_k_in_normal_mode_moves_selection_up() {
   let mut app = make_app();
   app.move_down();
   handle_key_event(&mut app, key(KeyCode::Char('k')));
   assert_eq!(app.selected_index(), 0);
}

#[test]
fn test_up_arrow_moves_selection_up() {
   let mut app = make_app();
   app.move_down();
   handle_key_event(&mut app, key(KeyCode::Up));
   assert_eq!(app.selected_index(), 0);
}

#[test]
fn test_capital_g_moves_to_last_entry() {
   let mut app = make_app();
   handle_key_event(&mut app, key(KeyCode::Char('G')));
   assert_eq!(app.selected_index(), 3);
}

#[test]
fn test_g_in_normal_mode_moves_to_top() {
   let mut app = make_app();
   app.move_bottom();
   assert_eq!(app.selected_index(), 3);
   handle_key_event(&mut app, key(KeyCode::Char('g')));
   assert_eq!(app.selected_index(), 0);
}

// ===== Normal mode — panel cycling =====

#[test]
fn test_n_cycles_panel_forward() {
   let mut app = make_app();
   assert_eq!(app.active_panel(), Panel::List);
   handle_key_event(&mut app, key(KeyCode::Char('n')));
   assert_eq!(app.active_panel(), Panel::Description);
}

#[test]
fn test_p_cycles_panel_backward() {
   let mut app = make_app();
   handle_key_event(&mut app, key(KeyCode::Char('p')));
   assert_eq!(app.active_panel(), Panel::Script);
}

// ===== Normal mode — filter =====

#[test]
fn test_l_cycles_filter_forward() {
   let mut app = make_app();
   assert_eq!(app.filter(), EntryFilter::All);
   handle_key_event(&mut app, key(KeyCode::Char('l')));
   assert_eq!(app.filter(), EntryFilter::Aliases);
}

#[test]
fn test_h_cycles_filter_backward() {
   let mut app = make_app();
   handle_key_event(&mut app, key(KeyCode::Char('h')));
   assert_eq!(app.filter(), EntryFilter::Functions);
}

#[test]
fn test_1_sets_aliases_filter() {
   let mut app = make_app();
   handle_key_event(&mut app, key(KeyCode::Char('1')));
   assert_eq!(app.filter(), EntryFilter::Aliases);
}

#[test]
fn test_2_sets_functions_filter() {
   let mut app = make_app();
   handle_key_event(&mut app, key(KeyCode::Char('2')));
   assert_eq!(app.filter(), EntryFilter::Functions);
}

#[test]
fn test_3_sets_all_filter() {
   let mut app = make_app();
   app.set_filter(EntryFilter::Aliases);
   handle_key_event(&mut app, key(KeyCode::Char('3')));
   assert_eq!(app.filter(), EntryFilter::All);
}

// ===== Normal mode — sort order and group mode =====

#[test]
fn test_ctrl_s_toggles_sort_order() {
   let mut app = make_app();
   assert_eq!(app.sort_order(), SortOrder::Ascending);
   handle_key_event(&mut app, ctrl('s'));
   assert_eq!(app.sort_order(), SortOrder::Descending);
}

#[test]
fn test_ctrl_g_cycles_group_mode() {
   let mut app = make_app();
   let before = app.group_mode();
   handle_key_event(&mut app, ctrl('g'));
   assert_ne!(std::mem::discriminant(&app.group_mode()), std::mem::discriminant(&before));
}

#[test]
fn test_os_sequence_toggles_sort_order() {
   let mut app = make_app();
   assert_eq!(app.sort_order(), SortOrder::Ascending);
   handle_key_event(&mut app, key(KeyCode::Char('o')));
   handle_key_event(&mut app, key(KeyCode::Char('s')));
   assert_eq!(app.sort_order(), SortOrder::Descending);
}

// ===== Normal mode — theme cycling =====

#[test]
fn test_tj_sequence_cycles_theme_next() {
   let mut app = make_app();
   let before = app.theme().name.clone();
   handle_key_event(&mut app, key(KeyCode::Char('t')));
   handle_key_event(&mut app, key(KeyCode::Char('j')));
   assert_ne!(app.theme().name, before);
}

#[test]
fn test_tk_sequence_cycles_theme_prev() {
   let mut app = make_app();
   let before = app.theme().name.clone();
   handle_key_event(&mut app, key(KeyCode::Char('t')));
   handle_key_event(&mut app, key(KeyCode::Char('k')));
   assert_ne!(app.theme().name, before);
}

// ===== Normal mode — scrolling =====

#[test]
fn test_ctrl_f_page_scrolls_to_last() {
   let mut app = make_app();
   handle_key_event(&mut app, ctrl('f'));
   assert_eq!(app.selected_index(), 3);
}

#[test]
fn test_ctrl_b_page_scrolls_to_first() {
   let mut app = make_app();
   app.move_bottom();
   handle_key_event(&mut app, ctrl('b'));
   assert_eq!(app.selected_index(), 0);
}

#[test]
fn test_ctrl_j_half_page_scrolls_down() {
   let mut app = make_app();
   handle_key_event(&mut app, ctrl('j'));
   assert_eq!(app.selected_index(), 3);
}

#[test]
fn test_ctrl_k_half_page_scrolls_up() {
   let mut app = make_app();
   app.move_bottom();
   handle_key_event(&mut app, ctrl('k'));
   assert_eq!(app.selected_index(), 0);
}

// ===== Normal mode — mode switching =====

#[test]
fn test_q_in_normal_mode_quits() {
   let mut app = make_app();
   handle_key_event(&mut app, key(KeyCode::Char('q')));
   assert!(app.should_quit);
}

#[test]
fn test_slash_in_normal_mode_enters_search() {
   let mut app = make_app();
   handle_key_event(&mut app, key(KeyCode::Char('/')));
   assert_eq!(app.input_mode(), InputMode::Search);
}

#[test]
fn test_i_in_normal_mode_enters_search() {
   let mut app = make_app();
   handle_key_event(&mut app, key(KeyCode::Char('i')));
   assert_eq!(app.input_mode(), InputMode::Search);
}

// ===== Search mode — Enter/Tab (enter-tab logic) =====

#[test]
fn test_enter_in_search_mode_sets_execute_action() {
   let mut app = make_app();
   app.enter_search_mode();
   handle_key_event(&mut app, key(KeyCode::Enter));
   assert!(app.should_quit);
   assert!(matches!(app.exit_action, Some(ExitAction::Execute)));
}

#[test]
fn test_tab_in_search_mode_sets_populate_action() {
   let mut app = make_app();
   app.enter_search_mode();
   handle_key_event(&mut app, key(KeyCode::Tab));
   assert!(app.should_quit);
   assert!(matches!(app.exit_action, Some(ExitAction::Populate)));
}

// ===== Search mode — text input & navigation =====

#[test]
fn test_char_in_search_mode_appends_to_query() {
   let mut app = make_app();
   app.enter_search_mode();
   handle_key_event(&mut app, key(KeyCode::Char('a')));
   assert_eq!(app.search_query(), "a");
}

#[test]
fn test_backspace_in_search_mode_removes_last_char() {
   let mut app = make_app();
   app.enter_search_mode();
   handle_key_event(&mut app, key(KeyCode::Char('a')));
   handle_key_event(&mut app, key(KeyCode::Backspace));
   assert!(app.search_query().is_empty());
}

#[test]
fn test_esc_in_search_mode_returns_to_normal() {
   let mut app = make_app();
   app.enter_search_mode();
   handle_key_event(&mut app, key(KeyCode::Esc));
   assert_eq!(app.input_mode(), InputMode::Normal);
}

// ===== Global keybinds =====

#[test]
fn test_ctrl_c_always_quits() {
   let mut app = make_app();
   handle_key_event(&mut app, ctrl('c'));
   assert!(app.should_quit);
}

#[test]
fn test_ctrl_d_always_quits() {
   let mut app = make_app();
   handle_key_event(&mut app, ctrl('d'));
   assert!(app.should_quit);
}

#[test]
fn test_ctrl_u_clears_search_query() {
   let mut app = make_app();
   app.set_search_query("hello".to_string());
   handle_key_event(&mut app, ctrl('u'));
   assert!(app.search_query().is_empty());
}

// ===== Multi-key sequences =====

#[test]
fn test_og_sequence_cycles_group_mode_forward() {
   let mut app = make_app();
   let before = app.group_mode();
   handle_key_event(&mut app, key(KeyCode::Char('o')));
   handle_key_event(&mut app, key(KeyCode::Char('g')));
   assert_ne!(std::mem::discriminant(&app.group_mode()), std::mem::discriminant(&before));
}

#[test]
fn test_og_upper_sequence_cycles_group_mode_backward() {
   let mut app = make_app();
   let before = app.group_mode();
   handle_key_event(&mut app, key(KeyCode::Char('o')));
   handle_key_event(&mut app, key(KeyCode::Char('G')));
   assert_ne!(std::mem::discriminant(&app.group_mode()), std::mem::discriminant(&before));
}

#[test]
fn test_pending_key_cleared_on_esc() {
   let mut app = make_app();
   handle_key_event(&mut app, key(KeyCode::Char('o')));
   assert!(app.pending_key().is_some());
   handle_key_event(&mut app, key(KeyCode::Esc));
   assert!(app.pending_key().is_none());
}

#[test]
fn test_invalid_sequence_clears_pending_key() {
   let mut app = make_app();
   handle_key_event(&mut app, key(KeyCode::Char('o')));
   assert!(app.pending_key().is_some());
   handle_key_event(&mut app, key(KeyCode::Char('z')));
   assert!(app.pending_key().is_none());
}

// ===== Help modal =====

#[test]
fn test_question_mark_toggles_help_open() {
   let mut app = make_app();
   handle_key_event(&mut app, key(KeyCode::Char('?')));
   assert!(app.show_help());
}

#[test]
fn test_esc_closes_help_modal() {
   let mut app = make_app();
   app.toggle_help();
   handle_key_event(&mut app, key(KeyCode::Esc));
   assert!(!app.show_help());
}

#[test]
fn test_q_closes_help_modal() {
   let mut app = make_app();
   app.toggle_help();
   handle_key_event(&mut app, key(KeyCode::Char('q')));
   assert!(!app.show_help());
}
