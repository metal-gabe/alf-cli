//! Tests for the CLI config command (source file resolution)

use super::{add_source_files, resolve_new_source_files};
use crate::config::{get_config_path, load_config, save_config, Config, ConfigLock, GeneralConfig};
use crate::test_support::TempHome;
use std::fs;

/// Comment stamped onto a seeded config so tests can detect whether it was rewritten
///
/// `toml::to_string_pretty` never emits comments, so a surviving marker proves `save_config` was
/// not called — which a value comparison alone cannot show when the values are meant to be equal.
const CONFIG_MARKER: &str = "\n# alf-test-marker\n";

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

// ===== Non-regular files =====

#[test]
fn test_resolve_new_source_files_errors_when_a_path_is_a_directory() {
   let home = TempHome::new();
   fs::create_dir_all(home.path().join("dotfiles")).expect("Should create dotfiles dir");
   let config = config_with(&[]);
   let error = resolve_new_source_files(&config, &raw(&["~/dotfiles"])).expect_err("Should reject a directory");
   assert!(
      error.to_string().contains("not a regular file"),
      "Expected a not-a-regular-file error for a directory, got: {error}"
   );
}

#[cfg(unix)]
#[test]
fn test_resolve_new_source_files_errors_when_a_path_is_a_fifo() {
   let home = TempHome::new();
   let fifo = home.path().join(".fifo_aliases");
   let status = std::process::Command::new("mkfifo").arg(&fifo).status().expect("Should run mkfifo");
   assert!(status.success(), "Should create the FIFO");
   let config = config_with(&[]);
   let error = resolve_new_source_files(&config, &raw(&["~/.fifo_aliases"])).expect_err("Should reject a FIFO");
   assert!(
      error.to_string().contains("not a regular file"),
      "Expected a not-a-regular-file error for a FIFO, got: {error}"
   );
}

#[cfg(unix)]
#[test]
fn test_resolve_new_source_files_accepts_a_symlink_to_a_regular_file() {
   let home = TempHome::new();
   link_to_dotfile(&home, "work_aliases", ".work_aliases");
   let config = config_with(&[]);
   let (to_add, duplicates) =
      resolve_new_source_files(&config, &raw(&["~/.work_aliases"])).expect("Should accept a symlinked file");
   assert_eq!(to_add, vec!["~/.work_aliases".to_string()]);
   assert!(duplicates.is_empty());
}

#[test]
fn test_add_source_files_does_not_write_when_a_path_is_a_directory() {
   let home = TempHome::new();
   home.touch(".work_aliases");
   fs::create_dir_all(home.path().join("dotfiles")).expect("Should create dotfiles dir");
   seed_marked_config(&[]);
   add_source_files(&raw(&["~/.work_aliases", "~/dotfiles"])).expect_err("Should reject the directory");
   assert!(is_config_unwritten(), "A rejected add should leave the config untouched");
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

// ===== Persisting added source files =====

/// Save a config holding `shell_files` and stamp it with [`CONFIG_MARKER`]
fn seed_marked_config(shell_files: &[&str]) {
   save_config(&config_with(shell_files)).expect("Should seed config");
   let path = get_config_path().expect("Should resolve config path");
   let mut content = fs::read_to_string(&path).expect("Should read seeded config");
   content.push_str(CONFIG_MARKER);
   fs::write(&path, content).expect("Should stamp seeded config");
}

/// The `shell_files` currently recorded on disk
fn configured_shell_files() -> Vec<String> {
   load_config().expect("Should load config").general.shell_files
}

/// Whether the seeded config still carries its marker, i.e. it was never rewritten
fn is_config_unwritten() -> bool {
   let path = get_config_path().expect("Should resolve config path");
   fs::read_to_string(&path).expect("Should read config").contains(CONFIG_MARKER)
}

#[test]
fn test_add_source_files_persists_the_added_path() {
   let home = TempHome::new();
   home.touch(".work_aliases");
   seed_marked_config(&[]);
   add_source_files(&raw(&["~/.work_aliases"])).expect("Should add the source file");
   assert_eq!(configured_shell_files(), vec!["~/.work_aliases".to_string()]);
   assert!(!is_config_unwritten(), "Adding a new file should rewrite the config");
}

#[test]
fn test_add_source_files_appends_to_the_existing_list() {
   let home = TempHome::new();
   home.touch(".zshrc");
   home.touch(".work_aliases");
   seed_marked_config(&["~/.zshrc"]);
   add_source_files(&raw(&["~/.work_aliases"])).expect("Should add the source file");
   assert_eq!(configured_shell_files(), raw(&["~/.zshrc", "~/.work_aliases"]));
}

#[test]
fn test_add_source_files_persists_multiple_paths_in_one_invocation() {
   let home = TempHome::new();
   home.touch(".work_aliases");
   home.touch(".personal_aliases");
   seed_marked_config(&[]);
   let paths = raw(&["~/.work_aliases", "~/.personal_aliases"]);
   add_source_files(&paths).expect("Should add the source files");
   assert_eq!(configured_shell_files(), paths);
}

#[test]
fn test_add_source_files_persists_only_the_new_paths_when_some_are_duplicates() {
   let home = TempHome::new();
   home.touch(".zshrc");
   home.touch(".work_aliases");
   seed_marked_config(&["~/.zshrc"]);
   let absolute = home.absolute(".zshrc");
   add_source_files(&raw(&[absolute.as_str(), "~/.work_aliases"])).expect("Should add the new source file");
   assert_eq!(configured_shell_files(), raw(&["~/.zshrc", "~/.work_aliases"]));
}

#[test]
fn test_add_source_files_persists_the_raw_path_form() {
   let home = TempHome::new();
   home.touch(".zshrc");
   seed_marked_config(&[]);
   add_source_files(&raw(&["$HOME/.zshrc"])).expect("Should add the source file");
   assert_eq!(configured_shell_files(), vec!["$HOME/.zshrc".to_string()], "The typed form should be what is stored");
}

// ===== Leaving the config untouched =====

#[test]
fn test_add_source_files_does_not_write_when_every_path_is_a_duplicate() {
   let home = TempHome::new();
   home.touch(".zshrc");
   seed_marked_config(&["~/.zshrc"]);
   let absolute = home.absolute(".zshrc");
   add_source_files(&raw(&[absolute.as_str()])).expect("A duplicate should not fail the add");
   assert!(is_config_unwritten(), "An all-duplicate add should not rewrite the config");
   assert_eq!(configured_shell_files(), vec!["~/.zshrc".to_string()]);
}

#[test]
fn test_add_source_files_leaves_the_config_untouched_when_a_path_is_missing() {
   let home = TempHome::new();
   home.touch(".zshrc");
   home.touch(".work_aliases");
   seed_marked_config(&["~/.zshrc"]);
   let error = add_source_files(&raw(&["~/.work_aliases", "~/missing.sh"])).expect_err("Should reject the invocation");
   assert!(error.to_string().contains("Shell file not found"), "Expected a not-found error, got: {error}");
   assert!(is_config_unwritten(), "A rejected add should not rewrite the config");
   assert_eq!(configured_shell_files(), vec!["~/.zshrc".to_string()]);
}

#[test]
fn test_add_source_files_leaves_the_config_untouched_when_a_path_is_relative() {
   let home = TempHome::new();
   home.touch(".zshrc");
   home.touch(".work_aliases");
   seed_marked_config(&["~/.zshrc"]);
   let error = add_source_files(&raw(&["~/.work_aliases", "../aliases.sh"])).expect_err("Should reject the invocation");
   assert!(error.to_string().contains("Relative paths are not allowed"), "Expected a relative error, got: {error}");
   assert!(is_config_unwritten(), "A rejected add should not rewrite the config");
   assert_eq!(configured_shell_files(), vec!["~/.zshrc".to_string()]);
}

// ===== First run =====

#[test]
fn test_add_source_files_bails_when_no_config_exists() {
   let home = TempHome::new();
   home.touch(".work_aliases");
   let error = add_source_files(&raw(&["~/.work_aliases"])).expect_err("Should bail without a config");
   assert!(error.to_string().contains("Run `alf init` to create one."), "Expected an `alf init` hint, got: {error}");
}

#[test]
fn test_add_source_files_reports_the_config_path_when_no_config_exists() {
   let home = TempHome::new();
   home.touch(".work_aliases");
   let expected = get_config_path().expect("Should resolve config path");
   let error = add_source_files(&raw(&["~/.work_aliases"])).expect_err("Should bail without a config");
   assert!(
      error.to_string().contains(&expected.display().to_string()),
      "Expected the config path in the error, got: {error}"
   );
}

#[test]
fn test_add_source_files_bails_before_validating_paths_when_no_config_exists() {
   let _home = TempHome::new();
   let error = add_source_files(&raw(&["../aliases.sh"])).expect_err("Should bail without a config");
   assert!(
      error.to_string().contains("No config found at"),
      "A missing config should be reported before path validation, got: {error}"
   );
}

// ===== Locking the config during the add =====

/// Assert that no lock is still held, by taking and releasing one
fn assert_lock_is_free(context: &str) {
   let lock = ConfigLock::acquire().unwrap_or_else(|_| panic!("The lock should be free {context}"));
   drop(lock);
}

#[test]
fn test_add_source_files_releases_the_lock_after_a_successful_add() {
   let home = TempHome::new();
   home.touch(".work_aliases");
   seed_marked_config(&[]);
   add_source_files(&raw(&["~/.work_aliases"])).expect("Should add the source file");
   assert_lock_is_free("after a successful add");
}

#[test]
fn test_add_source_files_releases_the_lock_after_a_failed_add() {
   let home = TempHome::new();
   home.touch(".work_aliases");
   seed_marked_config(&[]);

   for paths in [raw(&["relative/path"]), raw(&["~/.does_not_exist"])] {
      add_source_files(&paths).expect_err("Should reject the path");
      assert_lock_is_free("after a failed add");
   }
}

#[test]
fn test_add_source_files_releases_the_lock_after_an_all_duplicate_add() {
   let home = TempHome::new();
   home.touch(".zshrc");
   seed_marked_config(&["~/.zshrc"]);
   add_source_files(&raw(&["~/.zshrc"])).expect("A duplicate should not fail the add");
   assert_lock_is_free("after an early return");
}
