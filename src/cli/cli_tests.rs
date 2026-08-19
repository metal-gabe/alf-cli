//! Tests for the CLI argument parsing contract

use super::{Cli, Commands, ConfigAction};
use clap::{CommandFactory, Parser};

/// Parse `args` as a full command line, including the binary name
fn parse(args: &[&str]) -> Result<Cli, clap::Error> {
   Cli::try_parse_from(args)
}

/// Extract the `ConfigAction` from an invocation expected to parse as `config`
fn config_action(args: &[&str]) -> ConfigAction {
   match parse(args).expect("Should parse a config invocation").command {
      Some(Commands::Config {
         action,
      }) => action,
      other => panic!("Expected a config subcommand, got: {other:?}"),
   }
}

/// Extract the paths from an invocation expected to parse as `config add`
fn added_paths(args: &[&str]) -> Vec<String> {
   match config_action(args) {
      ConfigAction::Add {
         paths,
      } => paths,
      other => panic!("Expected an add action, got: {other:?}"),
   }
}

// ===== Command definition =====

#[test]
fn test_cli_definition_is_internally_consistent() {
   Cli::command().debug_assert();
}

// ===== `config add` =====

#[test]
fn test_config_add_accepts_one_or_more_paths() {
   for paths in [vec!["~/.work_aliases"], vec!["~/.work_aliases", "~/.personal_aliases"]] {
      let mut args = vec!["alf", "config", "add"];
      args.extend(paths.iter());
      let expected: Vec<String> = paths.iter().map(|path| path.to_string()).collect();
      assert_eq!(added_paths(&args), expected, "Unexpected paths for {paths:?}");
   }
}

#[test]
fn test_config_add_preserves_path_order_and_repeats() {
   let paths = added_paths(&["alf", "config", "add", "~/.b", "~/.a", "~/.b"]);
   assert_eq!(paths, vec!["~/.b".to_string(), "~/.a".to_string(), "~/.b".to_string()]);
}

#[test]
fn test_config_add_requires_at_least_one_path() {
   let error = parse(&["alf", "config", "add"]).expect_err("Should require a path");
   assert_eq!(error.kind(), clap::error::ErrorKind::MissingRequiredArgument);
}

// ===== Sibling config actions =====

#[test]
fn test_config_actions_parse_to_their_variants() {
   assert!(matches!(config_action(&["alf", "config", "show"]), ConfigAction::Show));
   assert!(matches!(config_action(&["alf", "config", "edit"]), ConfigAction::Edit));
   assert!(matches!(config_action(&["alf", "config", "reset"]), ConfigAction::Reset));
}

#[test]
fn test_config_rejects_an_unknown_action() {
   let error = parse(&["alf", "config", "remove"]).expect_err("Should reject an unknown action");
   assert_eq!(error.kind(), clap::error::ErrorKind::InvalidSubcommand);
}

#[test]
fn test_config_requires_an_action() {
   assert!(parse(&["alf", "config"]).is_err(), "`config` should require an action");
}
