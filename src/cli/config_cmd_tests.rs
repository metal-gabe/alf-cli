//! Tests for the CLI config command (source file resolution)

use super::resolve_new_source_files;
use crate::config::{Config, GeneralConfig};
use crate::test_support::TempHome;
use std::fs;

fn config_with(shell_files: &[&str]) -> Config {
   Config {
      general: GeneralConfig {
         shell_files: shell_files.iter().map(|file| file.to_string()).collect(),
         ..Default::default()
      },
      ..Default::default()
   }
}

/// Create `<home>/dotfiles/<target_name>` and point `<home>/<link_name>` at it as a symlink,
/// returning the absolute path of the target
#[cfg(unix)]
fn link_to_dotfile(
   home: &TempHome,
   target_name: &str,
   link_name: &str,
) -> String {
   let dotfiles = home.path().join("dotfiles");
   fs::create_dir_all(&dotfiles).expect("Should create dotfiles dir");
   let target = dotfiles.join(target_name);
   fs::File::create(&target).expect("Should create target file");
   std::os::unix::fs::symlink(&target, home.path().join(link_name)).expect("Should create symlink");
   target.to_string_lossy().to_string()
}

fn raw(paths: &[&str]) -> Vec<String> {
   paths.iter().map(|path| path.to_string()).collect()
}

// ===== Adding new source files =====

#[test]
fn test_resolve_new_source_files_adds_a_single_existing_file() {
   let home = TempHome::new();
   home.touch(".work_aliases");
   let config = config_with(&[]);
   let (to_add, duplicates) =
      resolve_new_source_files(&config, &raw(&["~/.work_aliases"])).expect("Should resolve existing file");
   assert_eq!(to_add, vec!["~/.work_aliases".to_string()]);
   assert!(duplicates.is_empty());
}

#[test]
fn test_resolve_new_source_files_adds_multiple_existing_files() {
   let home = TempHome::new();
   home.touch(".work_aliases");
   home.touch(".personal_aliases");
   let config = config_with(&[]);
   let paths = raw(&["~/.work_aliases", "~/.personal_aliases"]);
   let (to_add, duplicates) = resolve_new_source_files(&config, &paths).expect("Should resolve existing files");
   assert_eq!(to_add, paths);
   assert!(duplicates.is_empty());
}

#[test]
fn test_resolve_new_source_files_preserves_the_raw_path_form() {
   let home = TempHome::new();
   home.touch(".zshrc");

   for path_form in ["~/.zshrc", "$HOME/.zshrc"] {
      let config = config_with(&[]);
      let (to_add, _) = resolve_new_source_files(&config, &raw(&[path_form])).expect("Should resolve existing file");
      assert_eq!(to_add, vec![path_form.to_string()], "Expected {path_form} to be stored as typed");
   }
}

// ===== Relative paths =====

#[test]
fn test_resolve_new_source_files_rejects_relative_paths() {
   let _home = TempHome::new();
   let config = config_with(&[]);

   for relative in ["aliases.sh", "./aliases.sh", "../aliases.sh", "nested/aliases.sh", "../../deep/aliases.sh"] {
      let error = resolve_new_source_files(&config, &raw(&[relative])).expect_err("Should reject a relative path");
      assert!(
         error.to_string().contains("Relative paths are not allowed"),
         "Expected a relative-path error for {relative}, got: {error}"
      );
   }
}

#[test]
fn test_resolve_new_source_files_rejects_a_relative_path_that_exists() {
   let _home = TempHome::new();
   let config = config_with(&[]);

   // Cargo runs tests from the package root, so these resolve to a file that really is on disk
   for relative in ["Cargo.toml", "./Cargo.toml"] {
      let error =
         resolve_new_source_files(&config, &raw(&[relative])).expect_err("Should reject a relative path on disk");
      assert!(
         error.to_string().contains("Relative paths are not allowed"),
         "Existence should not excuse the relative path {relative}, got: {error}"
      );
   }
}

#[test]
fn test_resolve_new_source_files_reports_relativeness_before_missing_files() {
   let _home = TempHome::new();
   let config = config_with(&[]);
   let error = resolve_new_source_files(&config, &raw(&["~/missing.sh", "./aliases.sh"]))
      .expect_err("Should reject the invocation");
   assert!(
      error.to_string().contains("Relative paths are not allowed"),
      "Relative paths should be reported before missing files, got: {error}"
   );
}

#[test]
fn test_resolve_new_source_files_rejects_a_relative_path_mixed_with_valid_paths() {
   let home = TempHome::new();
   home.touch(".zshrc");
   let config = config_with(&[]);
   let result = resolve_new_source_files(&config, &raw(&["~/.zshrc", "../aliases.sh"]));
   assert!(result.is_err(), "A single relative path should reject the whole invocation");
}

// ===== Missing files =====

#[test]
fn test_resolve_new_source_files_errors_when_a_path_is_missing() {
   let _home = TempHome::new();
   let config = config_with(&[]);

   for missing in ["~/missing.sh", "$HOME/missing.sh", "/nope/missing.sh"] {
      let error = resolve_new_source_files(&config, &raw(&[missing])).expect_err("Should reject a missing file");
      assert!(
         error.to_string().contains("Shell file not found"),
         "Expected a not-found error for {missing}, got: {error}"
      );
   }
}

#[test]
fn test_resolve_new_source_files_reports_the_expanded_path_when_missing() {
   let home = TempHome::new();
   let config = config_with(&[]);
   let error = resolve_new_source_files(&config, &raw(&["~/missing.sh"])).expect_err("Should reject a missing file");
   assert!(
      error.to_string().contains(&home.absolute("missing.sh")),
      "Expected the expanded path in the error, got: {error}"
   );
}

#[test]
fn test_resolve_new_source_files_errors_before_adding_when_the_list_is_mixed() {
   let home = TempHome::new();
   home.touch(".work_aliases");
   let config = config_with(&[]);
   let result = resolve_new_source_files(&config, &raw(&["~/.work_aliases", "~/missing.sh"]));
   assert!(result.is_err(), "A single missing path should reject the whole invocation");
}

// ===== Duplicates =====

#[test]
fn test_resolve_new_source_files_detects_duplicates_across_path_forms() {
   let home = TempHome::new();
   home.touch(".zshrc");
   let absolute = home.absolute(".zshrc");

   for (stored, argument) in [
      ("~/.zshrc", absolute.as_str()),
      ("$HOME/.zshrc", absolute.as_str()),
      (absolute.as_str(), "~/.zshrc"),
      ("~/.zshrc", "~/.zshrc"),
   ] {
      let config = config_with(&[stored]);
      let (to_add, duplicates) =
         resolve_new_source_files(&config, &raw(&[argument])).expect("Should resolve existing file");
      assert!(to_add.is_empty(), "Expected no additions for stored {stored} and argument {argument}");
      assert_eq!(duplicates, vec![argument.to_string()]);
   }
}

#[cfg(unix)]
#[test]
fn test_resolve_new_source_files_detects_a_symlink_and_its_target_as_one_file() {
   let home = TempHome::new();
   let target = link_to_dotfile(&home, "zshrc", ".zshrc");

   for (stored, argument) in [("~/.zshrc", target.as_str()), (target.as_str(), "~/.zshrc")] {
      let config = config_with(&[stored]);
      let (to_add, duplicates) =
         resolve_new_source_files(&config, &raw(&[argument])).expect("Should resolve a symlinked file");
      assert!(to_add.is_empty(), "Expected no additions for stored {stored} and argument {argument}");
      assert_eq!(duplicates, vec![argument.to_string()]);
   }
}

#[cfg(unix)]
#[test]
fn test_resolve_new_source_files_dedupes_a_symlink_and_its_target_in_one_invocation() {
   let home = TempHome::new();
   let target = link_to_dotfile(&home, "zshrc", ".zshrc");
   let config = config_with(&[]);
   let (to_add, duplicates) =
      resolve_new_source_files(&config, &raw(&["~/.zshrc", target.as_str()])).expect("Should resolve a symlinked file");
   assert_eq!(to_add, vec!["~/.zshrc".to_string()]);
   assert_eq!(duplicates, vec![target]);
}

#[test]
fn test_resolve_new_source_files_treats_a_dot_dot_segment_as_the_same_file() {
   let home = TempHome::new();
   home.touch(".zshrc");
   let round_trip = home.absolute(".config/../.zshrc");
   fs::create_dir_all(home.path().join(".config")).expect("Should create dir");
   let config = config_with(&["~/.zshrc"]);
   let (to_add, duplicates) =
      resolve_new_source_files(&config, &raw(&[round_trip.as_str()])).expect("Should resolve the file");
   assert!(to_add.is_empty(), "A `..` round trip should not add a second entry");
   assert_eq!(duplicates, vec![round_trip]);
}

#[test]
fn test_resolve_new_source_files_tolerates_a_stale_configured_entry() {
   let home = TempHome::new();
   home.touch(".zshrc");
   let config = config_with(&["~/gone.sh"]);
   let (to_add, duplicates) =
      resolve_new_source_files(&config, &raw(&["~/.zshrc"])).expect("An unresolvable entry should not fail the add");
   assert_eq!(to_add, vec!["~/.zshrc".to_string()]);
   assert!(duplicates.is_empty());
}

#[test]
fn test_resolve_new_source_files_dedupes_repeats_within_one_invocation() {
   let home = TempHome::new();
   home.touch(".zshrc");
   let config = config_with(&[]);
   let paths = raw(&["~/.zshrc", "~/.zshrc"]);
   let (to_add, duplicates) = resolve_new_source_files(&config, &paths).expect("Should resolve existing file");
   assert_eq!(to_add, vec!["~/.zshrc".to_string()]);
   assert_eq!(duplicates, vec!["~/.zshrc".to_string()]);
}
