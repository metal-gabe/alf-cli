//! Tests for the CLI init module (shell file detection and shell hooks)

use super::{detect_shell_files, print_shell_hook};
use std::fs;
use tempfile::TempDir;

fn make_home() -> TempDir {
   tempfile::tempdir().unwrap()
}

fn touch(dir: &TempDir, name: &str) {
   fs::File::create(dir.path().join(name)).unwrap();
}

// ===== detect_shell_files =====

#[test]
fn test_detect_shell_files_empty_dir_returns_empty() {
   let home = make_home();
   let result = detect_shell_files(home.path().to_str().unwrap());
   assert!(result.is_empty());
}

#[test]
fn test_detect_shell_files_finds_bashrc() {
   let home = make_home();
   touch(&home, ".bashrc");
   let result = detect_shell_files(home.path().to_str().unwrap());
   assert!(result.iter().any(|p| p.ends_with(".bashrc")));
}

#[test]
fn test_detect_shell_files_finds_zshrc() {
   let home = make_home();
   touch(&home, ".zshrc");
   let result = detect_shell_files(home.path().to_str().unwrap());
   assert!(result.iter().any(|p| p.ends_with(".zshrc")));
}

#[test]
fn test_detect_shell_files_finds_multiple_standard_files() {
   let home = make_home();
   touch(&home, ".bashrc");
   touch(&home, ".zshrc");
   let result = detect_shell_files(home.path().to_str().unwrap());
   assert_eq!(result.len(), 2);
}

#[test]
fn test_detect_shell_files_ignores_nonstandard_files() {
   let home = make_home();
   touch(&home, ".myrc");
   let result = detect_shell_files(home.path().to_str().unwrap());
   assert!(result.is_empty());
}

#[test]
fn test_detect_shell_files_finds_fish_config() {
   let home = make_home();
   let fish_dir = home.path().join(".config").join("fish");
   fs::create_dir_all(&fish_dir).unwrap();
   fs::File::create(fish_dir.join("config.fish")).unwrap();
   let result = detect_shell_files(home.path().to_str().unwrap());
   assert!(result.iter().any(|p| p.ends_with("config.fish")));
}

// ===== print_shell_hook =====

#[test]
fn test_print_shell_hook_zsh_returns_ok() {
   assert!(print_shell_hook("zsh").is_ok());
}

#[test]
fn test_print_shell_hook_bash_returns_ok() {
   assert!(print_shell_hook("bash").is_ok());
}

#[test]
fn test_print_shell_hook_unsupported_shell_returns_err() {
   assert!(print_shell_hook("fish").is_err());
}

#[test]
fn test_print_shell_hook_case_insensitive_zsh() {
   assert!(print_shell_hook("ZSH").is_ok());
}

#[test]
fn test_print_shell_hook_case_insensitive_bash() {
   assert!(print_shell_hook("BASH").is_ok());
}
