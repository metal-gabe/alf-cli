//! CLI argument parsing and command handling.

pub mod config_cmd;
pub mod init;

use clap::{Parser, Subcommand};

/// Alias & Function CLI Searching Tool
#[derive(Debug, Parser)]
#[command(name = "alf")]
#[command(version, about, long_about = None, disable_help_subcommand = true)]
pub struct Cli {
   #[command(subcommand)]
   pub command: Option<Commands>,
}

/// Available subcommands
#[derive(Debug, Subcommand)]
pub enum Commands {
   /// Launch the interactive TUI search interface with required query
   Search {
      /// Required search query to filter results on startup
      query: String,
   },

   /// Initialize configuration (first-run setup)
   Init {
      /// Print shell integration wrapper for the given shell and exit
      #[arg(long, value_name = "SHELL")]
      print_shell_hook: Option<String>,
   },

   /// Print shell integration wrapper function
   ShellHook {
      /// Shell to generate the wrapper for (zsh, bash)
      shell: String,
   },

   /// Manage configuration
   Config {
      #[command(subcommand)]
      action: ConfigAction,
   },
}

/// Configuration management actions
#[derive(Debug, Subcommand)]
pub enum ConfigAction {
   /// Show current configuration
   Show,

   /// Edit configuration file
   Edit,

   /// Reset to default configuration
   Reset,
}
