//! Tests for the CLI init module (shell file detection and shell hooks)

use super::{detect_shell_files, get_shell_hook, parse_theme_choice, print_shell_hook};
use std::fs;
use tempfile::TempDir;

fn make_home() -> TempDir {
   tempfile::tempdir().unwrap()
}

fn touch(
   dir: &TempDir,
   name: &str,
) {
   fs::File::create(dir.path().join(name)).unwrap();
}

// ===== detect_shell_files =====

#[test]
fn detect_shell_files_returns_empty_on_an_empty_dir() {
   let home = make_home();
   let result = detect_shell_files(home.path().to_str().unwrap());
   assert!(result.is_empty());
}

#[test]
fn detect_shell_files_finds_each_standard_file() {
   for name in [".bashrc", ".zshrc", ".kshrc", ".profile", ".zprofile"] {
      let home = make_home();
      touch(&home, name);
      let result = detect_shell_files(home.path().to_str().unwrap());
      assert_eq!(result.len(), 1, "{name}");
      assert!(result[0].ends_with(name), "{name}");
   }
}

#[test]
fn detect_shell_files_finds_multiple_standard_files() {
   let home = make_home();
   touch(&home, ".bashrc");
   touch(&home, ".zshrc");
   let result = detect_shell_files(home.path().to_str().unwrap());
   assert_eq!(result.len(), 2);
}

#[test]
fn detect_shell_files_ignores_nonstandard_files() {
   let home = make_home();
   touch(&home, ".myrc");
   let result = detect_shell_files(home.path().to_str().unwrap());
   assert!(result.is_empty());
}

#[test]
fn detect_shell_files_finds_fish_config() {
   let home = make_home();
   let fish_dir = home.path().join(".config").join("fish");
   fs::create_dir_all(&fish_dir).unwrap();
   fs::File::create(fish_dir.join("config.fish")).unwrap();
   let result = detect_shell_files(home.path().to_str().unwrap());
   assert!(result.iter().any(|p| p.ends_with("config.fish")));
}

// ===== print_shell_hook =====

#[test]
fn print_shell_hook_accepts_supported_shells_in_any_case() {
   for shell in ["zsh", "bash", "ZSH", "BASH"] {
      assert!(print_shell_hook(shell).is_ok(), "Expected ok for {shell}");
      assert_eq!(get_shell_hook(shell), get_shell_hook(&shell.to_lowercase()), "{shell}");
      assert!(!get_shell_hook(shell).is_empty(), "{shell}");
   }
}

#[test]
fn print_shell_hook_rejects_unsupported_shells_by_name() {
   let error = print_shell_hook("fish").expect_err("fish has no hook");
   assert!(error.to_string().contains("fish"));
}
// ===== get_shell_hook =====

#[test]
fn get_shell_hook_contains_the_wrapper_contract() {
   let needles =
      ["ALF_LOOKUP_NAME", "ALF_WHICH_RESULT", "ALF_TYPE_RESULT", "ALF_OUTPUT=\"$tmp\"", "command alf \"$@\""];

   for shell in ["zsh", "bash"] {
      let hook = get_shell_hook(shell);

      for needle in needles {
         assert!(hook.contains(needle), "{shell} hook is missing {needle}");
      }
   }
}

#[test]
fn get_shell_hook_never_emits_unquoted_help_flags() {
   for shell in ["zsh", "bash"] {
      let hook = get_shell_hook(shell);
      assert!(hook.contains("'-h'|'--help'"), "{shell} hook should quote its help flags");
      assert!(
         !hook.split_whitespace().any(|word| word == "-h" || word == "--help"),
         "{shell} hook has an unquoted help flag, which a global alias would expand"
      );
   }
}

#[test]
fn get_shell_hook_shares_the_which_block_between_shells() {
   let which_block = |shell| {
      let hook = get_shell_hook(shell);
      let start = hook.find("if [[ \"$1\" == \"which\" ]]").expect("Should have a which block");
      let end = hook.find("ALF_OUTPUT=").expect("Should run alf");
      hook[start..end].to_string()
   };
   assert_eq!(which_block("zsh"), which_block("bash"));
}

// ===== parse_theme_choice =====

#[test]
fn parse_theme_choice_resolves_menu_answers() {
   let themes = ["alpha".to_string(), "beta".to_string()];
   let cases = [
      ("", Some("default")),
      ("\n", Some("default")),
      ("1", Some("alpha")),
      ("2\n", Some("beta")),
      ("0", None),
      ("99", None),
      ("abc", None),
   ];
   for (choice, expected) in cases {
      assert_eq!(parse_theme_choice(choice, &themes).as_deref(), expected, "Choice {choice:?}");
   }
}
