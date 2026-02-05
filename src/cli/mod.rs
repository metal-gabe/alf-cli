//! CLI argument parsing and command handling.

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
    /// Launch the interactive TUI search interface (default)
    Search,

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
