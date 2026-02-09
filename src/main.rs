//! Main entry point for the alf CLI.

use anyhow::Result;
use clap::Parser;

use alf::cli::{Cli, Commands};

fn main() -> Result<()> {
   let cli = Cli::parse();

   match cli.command {
      Some(Commands::Search) | None => {
         // Default action: launch the TUI search interface
         alf::tui::run()
      }
      Some(Commands::Init) => {
         println!("Running first-time configuration...");
         // TODO: Run init wizard
         todo!("Implement init wizard")
      }
      Some(Commands::Config { action }) => {
         println!("Config command: {:?}", action);
         // TODO: Handle config subcommands
         todo!("Implement config commands")
      }
   }
}
