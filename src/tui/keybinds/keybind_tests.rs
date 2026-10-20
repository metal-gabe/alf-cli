//! Tests for the keybind dispatch layer

use super::handle_key_event;
use crate::tui::app::app_tests::{make_app, make_empty_app};
use crate::tui::app::{App, ExitAction};
use crate::tui::state::{EntryFilter, InputMode, Panel, SortOrder};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

fn key(code: KeyCode) -> KeyEvent {
   KeyEvent::new(code, KeyModifiers::NONE)
}

fn ctrl(c: char) -> KeyEvent {
   KeyEvent::new(KeyCode::Char(c), KeyModifiers::CONTROL)
}

fn with(
   code: KeyCode,
   modifiers: KeyModifiers,
) -> KeyEvent {
   KeyEvent::new(code, modifiers)
}

fn search_app(marked: &str) -> App {
   let (before, after) = marked.split_once('|').expect("marked query must contain a '|' cursor");
   let mut app = make_app();
   app.enter_search_mode();
   app.set_search_query(format!("{before}{after}"));
   for _ in 0..after.chars().count() {
      app.search_cursor_left();
   }
   app
}

fn marked_query(app: &App) -> String {
   let chars: Vec<char> = app.search_query().chars().collect();
   let before: String = chars[..app.cursor_position()].iter().collect();
   let after: String = chars[app.cursor_position()..].iter().collect();
   format!("{before}|{after}")
}

// ===== Normal mode — Enter/Tab (enter-tab logic) =====

#[test]
fn enter_in_normal_mode_sets_execute_action() {
   let mut app = make_app();
   handle_key_event(&mut app, key(KeyCode::Enter));
   assert!(app.should_quit);
   assert!(matches!(app.exit_action, Some(ExitAction::Execute)));
}

#[test]
fn tab_in_normal_mode_sets_populate_action() {
   let mut app = make_app();
   handle_key_event(&mut app, key(KeyCode::Tab));
   assert!(app.should_quit);
   assert!(matches!(app.exit_action, Some(ExitAction::Populate)));
}

#[test]
fn enter_in_normal_mode_on_empty_app_quits_with_no_action() {
   let mut app = make_empty_app();
   handle_key_event(&mut app, key(KeyCode::Enter));
   assert!(app.should_quit);
   assert!(app.exit_action.is_none());
}

#[test]
fn tab_in_normal_mode_on_empty_app_quits_with_no_action() {
   let mut app = make_empty_app();
   handle_key_event(&mut app, key(KeyCode::Tab));
   assert!(app.should_quit);
   assert!(app.exit_action.is_none());
}

// ===== Normal mode — navigation =====

#[test]
fn j_in_normal_mode_moves_selection_down() {
   let mut app = make_app();
   let before = app.selected_index();
   handle_key_event(&mut app, key(KeyCode::Char('j')));
   assert!(app.selected_index() > before);
}

#[test]
fn down_arrow_moves_selection_down() {
   let mut app = make_app();
   let before = app.selected_index();
   handle_key_event(&mut app, key(KeyCode::Down));
   assert!(app.selected_index() > before);
}

#[test]
fn k_in_normal_mode_moves_selection_up() {
   let mut app = make_app();
   app.move_down();
   handle_key_event(&mut app, key(KeyCode::Char('k')));
   assert_eq!(app.selected_index(), 0);
}

#[test]
fn up_arrow_moves_selection_up() {
   let mut app = make_app();
   app.move_down();
   handle_key_event(&mut app, key(KeyCode::Up));
   assert_eq!(app.selected_index(), 0);
}

#[test]
fn capital_g_moves_to_last_entry() {
   let mut app = make_app();
   handle_key_event(&mut app, key(KeyCode::Char('G')));
   assert_eq!(app.selected_index(), 3);
}

#[test]
fn g_in_normal_mode_moves_to_top() {
   let mut app = make_app();
   app.move_bottom();
   assert_eq!(app.selected_index(), 3);
   handle_key_event(&mut app, key(KeyCode::Char('g')));
   assert_eq!(app.selected_index(), 0);
}

// ===== Normal mode — panel cycling =====

#[test]
fn n_cycles_panel_forward() {
   let mut app = make_app();
   assert_eq!(app.active_panel(), Panel::List);
   handle_key_event(&mut app, key(KeyCode::Char('n')));
   assert_eq!(app.active_panel(), Panel::Description);
}

#[test]
fn p_cycles_panel_backward() {
   let mut app = make_app();
   handle_key_event(&mut app, key(KeyCode::Char('p')));
   assert_eq!(app.active_panel(), Panel::Script);
}

// ===== Search mode — panel cycling =====

#[test]
fn shift_n_and_shift_p_cycle_panels_in_search_mode() {
   let cases = [('N', Panel::Description), ('P', Panel::Script)];
   for (c, expected_panel) in cases {
      let mut app = make_app();
      app.enter_search_mode();
      handle_key_event(&mut app, key(KeyCode::Char(c)));
      assert_eq!(app.active_panel(), expected_panel, "Key '{c}'");
      assert!(app.search_query().is_empty(), "Key '{c}' should not be inserted into the query");
   }
}

// ===== Fullscreen =====

#[test]
fn fullscreen_key_dispatch_per_mode() {
   let cases =
      [(InputMode::Normal, 'f', true, ""), (InputMode::Search, 'F', true, ""), (InputMode::Search, 'f', false, "f")];
   for (mode, c, is_expected_fullscreen, expected_query) in cases {
      let mut app = make_app();
      if mode == InputMode::Search {
         app.enter_search_mode();
      }
      handle_key_event(&mut app, key(KeyCode::Char(c)));
      assert_eq!(app.is_fullscreen(), is_expected_fullscreen, "{mode:?} mode, key '{c}'");
      assert_eq!(app.search_query(), expected_query, "{mode:?} mode, key '{c}'");
   }
}

#[test]
fn f_twice_in_normal_mode_toggles_fullscreen_off() {
   let mut app = make_app();
   handle_key_event(&mut app, key(KeyCode::Char('f')));
   assert!(app.is_fullscreen());
   handle_key_event(&mut app, key(KeyCode::Char('f')));
   assert!(!app.is_fullscreen());
}

#[test]
fn ctrl_f_scrolls_instead_of_toggling_fullscreen() {
   let mut app = make_app();
   handle_key_event(&mut app, ctrl('f'));
   assert!(!app.is_fullscreen());
   assert_eq!(app.selected_index(), 3);
}

#[test]
fn f_with_help_open_does_not_toggle_fullscreen() {
   let mut app = make_app();
   app.toggle_help();
   handle_key_event(&mut app, key(KeyCode::Char('f')));
   assert!(!app.is_fullscreen());
   assert!(app.show_help());
}

#[test]
fn f_with_pending_key_clears_pending_without_toggling_fullscreen() {
   for pending in ['o', 't'] {
      let mut app = make_app();
      handle_key_event(&mut app, key(KeyCode::Char(pending)));
      handle_key_event(&mut app, key(KeyCode::Char('f')));
      assert!(app.pending_key().is_none(), "Pending '{pending}'");
      assert!(!app.is_fullscreen(), "Pending '{pending}'");
   }
}

// ===== Normal mode — filter =====

#[test]
fn l_cycles_filter_forward() {
   let mut app = make_app();
   assert_eq!(app.filter(), EntryFilter::All);
   handle_key_event(&mut app, key(KeyCode::Char('l')));
   assert_eq!(app.filter(), EntryFilter::Aliases);
}

#[test]
fn h_cycles_filter_backward() {
   let mut app = make_app();
   handle_key_event(&mut app, key(KeyCode::Char('h')));
   assert_eq!(app.filter(), EntryFilter::Functions);
}

#[test]
fn key_1_sets_aliases_filter() {
   let mut app = make_app();
   handle_key_event(&mut app, key(KeyCode::Char('1')));
   assert_eq!(app.filter(), EntryFilter::Aliases);
}

#[test]
fn key_2_sets_functions_filter() {
   let mut app = make_app();
   handle_key_event(&mut app, key(KeyCode::Char('2')));
   assert_eq!(app.filter(), EntryFilter::Functions);
}

#[test]
fn key_3_sets_all_filter() {
   let mut app = make_app();
   app.set_filter(EntryFilter::Aliases);
   handle_key_event(&mut app, key(KeyCode::Char('3')));
   assert_eq!(app.filter(), EntryFilter::All);
}

// ===== Normal mode — sort order and group mode =====

#[test]
fn ctrl_s_toggles_sort_order() {
   let mut app = make_app();
   assert_eq!(app.sort_order(), SortOrder::Ascending);
   handle_key_event(&mut app, ctrl('s'));
   assert_eq!(app.sort_order(), SortOrder::Descending);
}

#[test]
fn ctrl_g_cycles_group_mode() {
   let mut app = make_app();
   let before = app.group_mode();
   handle_key_event(&mut app, ctrl('g'));
   assert_ne!(std::mem::discriminant(&app.group_mode()), std::mem::discriminant(&before));
}

#[test]
fn os_sequence_toggles_sort_order() {
   let mut app = make_app();
   assert_eq!(app.sort_order(), SortOrder::Ascending);
   handle_key_event(&mut app, key(KeyCode::Char('o')));
   handle_key_event(&mut app, key(KeyCode::Char('s')));
   assert_eq!(app.sort_order(), SortOrder::Descending);
}

// ===== Normal mode — theme cycling =====

#[test]
fn tj_sequence_cycles_theme_next() {
   let mut app = make_app();
   let before = app.theme().name.clone();
   handle_key_event(&mut app, key(KeyCode::Char('t')));
   handle_key_event(&mut app, key(KeyCode::Char('j')));
   assert_ne!(app.theme().name, before);
}

#[test]
fn tk_sequence_cycles_theme_prev() {
   let mut app = make_app();
   let before = app.theme().name.clone();
   handle_key_event(&mut app, key(KeyCode::Char('t')));
   handle_key_event(&mut app, key(KeyCode::Char('k')));
   assert_ne!(app.theme().name, before);
}

// ===== Normal mode — scrolling =====

#[test]
fn ctrl_f_page_scrolls_to_last() {
   let mut app = make_app();
   handle_key_event(&mut app, ctrl('f'));
   assert_eq!(app.selected_index(), 3);
}

#[test]
fn ctrl_b_page_scrolls_to_first() {
   let mut app = make_app();
   app.move_bottom();
   handle_key_event(&mut app, ctrl('b'));
   assert_eq!(app.selected_index(), 0);
}

#[test]
fn half_page_scroll_keys_in_normal_mode() {
   let cases = [('j', false, 3), ('d', false, 3), ('k', true, 0), ('u', true, 0)];
   for (c, is_start_at_bottom, expected_index) in cases {
      let mut app = make_app();
      if is_start_at_bottom {
         app.move_bottom();
      }
      handle_key_event(&mut app, ctrl(c));
      assert_eq!(app.selected_index(), expected_index, "ctrl-{c}");
   }
}

#[test]
fn ctrl_u_in_normal_mode_keeps_search_query() {
   let mut app = make_app();
   app.set_search_query("a".to_string());
   handle_key_event(&mut app, ctrl('u'));
   assert_eq!(app.search_query(), "a");
}

// ===== Normal mode — mode switching =====

#[test]
fn q_in_normal_mode_quits() {
   let mut app = make_app();
   handle_key_event(&mut app, key(KeyCode::Char('q')));
   assert!(app.should_quit);
}

#[test]
fn slash_in_normal_mode_enters_search() {
   let mut app = make_app();
   handle_key_event(&mut app, key(KeyCode::Char('/')));
   assert_eq!(app.input_mode(), InputMode::Search);
}

#[test]
fn i_in_normal_mode_enters_search() {
   let mut app = make_app();
   handle_key_event(&mut app, key(KeyCode::Char('i')));
   assert_eq!(app.input_mode(), InputMode::Search);
}

// ===== Search mode — Enter/Tab (enter-tab logic) =====

#[test]
fn enter_in_search_mode_sets_execute_action() {
   let mut app = make_app();
   app.enter_search_mode();
   handle_key_event(&mut app, key(KeyCode::Enter));
   assert!(app.should_quit);
   assert!(matches!(app.exit_action, Some(ExitAction::Execute)));
}

#[test]
fn tab_in_search_mode_sets_populate_action() {
   let mut app = make_app();
   app.enter_search_mode();
   handle_key_event(&mut app, key(KeyCode::Tab));
   assert!(app.should_quit);
   assert!(matches!(app.exit_action, Some(ExitAction::Populate)));
}

// ===== Search mode — text input & navigation =====

#[test]
fn char_in_search_mode_appends_to_query() {
   let mut app = make_app();
   app.enter_search_mode();
   handle_key_event(&mut app, key(KeyCode::Char('a')));
   assert_eq!(app.search_query(), "a");
}

#[test]
fn backspace_in_search_mode_removes_last_char() {
   let mut app = make_app();
   app.enter_search_mode();
   handle_key_event(&mut app, key(KeyCode::Char('a')));
   handle_key_event(&mut app, key(KeyCode::Backspace));
   assert!(app.search_query().is_empty());
}

#[test]
fn esc_in_search_mode_returns_to_normal() {
   let mut app = make_app();
   app.enter_search_mode();
   handle_key_event(&mut app, key(KeyCode::Esc));
   assert_eq!(app.input_mode(), InputMode::Normal);
}

// ===== Global keybinds =====

#[test]
fn ctrl_c_always_quits() {
   let cases = [(InputMode::Normal, false), (InputMode::Search, false), (InputMode::Normal, true)];
   for (mode, is_help_open) in cases {
      let mut app = make_app();
      if mode == InputMode::Search {
         app.enter_search_mode();
      }
      if is_help_open {
         app.toggle_help();
      }
      handle_key_event(&mut app, ctrl('c'));
      assert!(app.should_quit, "{mode:?} mode, help open: {is_help_open}");
   }
}

#[test]
fn ctrl_d_never_quits() {
   let cases = [(InputMode::Normal, false), (InputMode::Search, false), (InputMode::Normal, true)];
   for (mode, is_help_open) in cases {
      let mut app = make_app();
      if mode == InputMode::Search {
         app.enter_search_mode();
      }
      if is_help_open {
         app.toggle_help();
      }
      handle_key_event(&mut app, ctrl('d'));
      assert!(!app.should_quit, "{mode:?} mode, help open: {is_help_open}");
   }
}

#[test]
fn ctrl_u_clears_search_query_in_search_mode() {
   let mut app = make_app();
   app.enter_search_mode();
   app.set_search_query("hello".to_string());
   handle_key_event(&mut app, ctrl('u'));
   assert!(app.search_query().is_empty());
}

// ===== Multi-key sequences =====

#[test]
fn og_sequence_cycles_group_mode_forward() {
   let mut app = make_app();
   let before = app.group_mode();
   handle_key_event(&mut app, key(KeyCode::Char('o')));
   handle_key_event(&mut app, key(KeyCode::Char('g')));
   assert_ne!(std::mem::discriminant(&app.group_mode()), std::mem::discriminant(&before));
}

#[test]
fn og_upper_sequence_cycles_group_mode_backward() {
   let mut app = make_app();
   let before = app.group_mode();
   handle_key_event(&mut app, key(KeyCode::Char('o')));
   handle_key_event(&mut app, key(KeyCode::Char('G')));
   assert_ne!(std::mem::discriminant(&app.group_mode()), std::mem::discriminant(&before));
}

#[test]
fn pending_key_cleared_on_esc() {
   let mut app = make_app();
   handle_key_event(&mut app, key(KeyCode::Char('o')));
   assert!(app.pending_key().is_some());
   handle_key_event(&mut app, key(KeyCode::Esc));
   assert!(app.pending_key().is_none());
}

#[test]
fn invalid_sequence_clears_pending_key() {
   let mut app = make_app();
   handle_key_event(&mut app, key(KeyCode::Char('o')));
   assert!(app.pending_key().is_some());
   handle_key_event(&mut app, key(KeyCode::Char('z')));
   assert!(app.pending_key().is_none());
}

// ===== Help modal =====

#[test]
fn question_mark_toggles_help_open() {
   let mut app = make_app();
   handle_key_event(&mut app, key(KeyCode::Char('?')));
   assert!(app.show_help());
}

#[test]
fn esc_closes_help_modal() {
   let mut app = make_app();
   app.toggle_help();
   handle_key_event(&mut app, key(KeyCode::Esc));
   assert!(!app.show_help());
}

#[test]
fn q_closes_help_modal() {
   let mut app = make_app();
   app.toggle_help();
   handle_key_event(&mut app, key(KeyCode::Char('q')));
   assert!(!app.show_help());
}

// ===== Search mode — word & line editing =====

#[test]
fn search_mode_text_editing_keys() {
   let ctrl_mod = KeyModifiers::CONTROL;
   let alt_mod = KeyModifiers::ALT;
   let cases = [
      ("ctrl-w", ctrl('w'), "git-co|", "git-|"),
      ("alt-backspace", with(KeyCode::Backspace, alt_mod), "git-co|", "git-|"),
      ("ctrl-backspace", with(KeyCode::Backspace, ctrl_mod), "git-co|", "git-|"),
      ("ctrl-h", ctrl('h'), "git-co|", "git-c|"),
      ("alt-d", with(KeyCode::Char('d'), alt_mod), "|git-co", "|-co"),
      ("ctrl-delete", with(KeyCode::Delete, ctrl_mod), "|git-co", "|-co"),
      ("delete", key(KeyCode::Delete), "|git-co", "|it-co"),
      ("ctrl-d", ctrl('d'), "|git-co", "|it-co"),
      ("ctrl-left", with(KeyCode::Left, ctrl_mod), "git-co|", "git-|co"),
      ("alt-left", with(KeyCode::Left, alt_mod), "git-co|", "git-|co"),
      ("alt-b", with(KeyCode::Char('b'), alt_mod), "git-co|", "git-|co"),
      ("ctrl-right", with(KeyCode::Right, ctrl_mod), "|git-co", "git|-co"),
      ("alt-right", with(KeyCode::Right, alt_mod), "|git-co", "git|-co"),
      ("alt-f", with(KeyCode::Char('f'), alt_mod), "|git-co", "git|-co"),
      ("ctrl-a", ctrl('a'), "git-co|", "|git-co"),
      ("home", key(KeyCode::Home), "git-co|", "|git-co"),
      ("ctrl-e", ctrl('e'), "|git-co", "git-co|"),
      ("end", key(KeyCode::End), "|git-co", "git-co|"),
   ];
   for (label, event, start, expected) in cases {
      let mut app = search_app(start);
      handle_key_event(&mut app, event);
      assert_eq!(marked_query(&app), expected, "{label} on {start:?}");
      assert!(!app.should_quit, "{label} should not quit");
   }
}

#[test]
fn search_mode_modifier_chords_only_insert_plain_or_altgr_chars() {
   let cases = [
      ("ctrl-x", ctrl('x'), "|"),
      ("alt-x", with(KeyCode::Char('x'), KeyModifiers::ALT), "|"),
      ("ctrl-alt-@ (AltGr)", with(KeyCode::Char('@'), KeyModifiers::CONTROL | KeyModifiers::ALT), "@|"),
      ("shift-a", with(KeyCode::Char('A'), KeyModifiers::SHIFT), "a|"),
      ("x", key(KeyCode::Char('x')), "x|"),
   ];
   for (label, event, expected) in cases {
      let mut app = search_app("|");
      handle_key_event(&mut app, event);
      assert_eq!(marked_query(&app), expected, "{label}");
   }
}
