use anyhow::Result;
use clap::Parser;

use alf::cli::{Cli, Commands};

fn main() -> Result<()> {
   env_logger::init();
   let cli = Cli::parse();

   match cli.command {
      Some(Commands::Search { query }) => {
         if query.is_none() {
            eprintln!("Error: search query required");
            eprintln!("Usage: alf search <QUERY>");
            std::process::exit(1);
         }
         alf::tui::run(query)
      }
      None => alf::tui::run(None),
      Some(Commands::Init) => alf::cli::init::run_init_wizard(),
      Some(Commands::Config { action }) => alf::cli::config_cmd::run_config_action(action),
   }
}
