//! CLI argument parsing and command handling.

pub mod config_cmd;
pub mod init;

use clap::{Parser, Subcommand};

/// Alias & Function CLI Searching Tool
#[derive(Debug, Parser)]
#[command(name = "alf")]
#[command(version, about, long_about = None)]
pub struct Cli {
   #[command(subcommand)]
   pub command: Option<Commands>,
}

/// Available subcommands
#[derive(Debug, Subcommand)]
pub enum Commands {
   /// Launch the interactive TUI search interface with optional query
   Search {
      /// Search query to filter results on startup
      query: Option<String>,
   },

   /// Initialize configuration (first-run setup)
   Init,

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
