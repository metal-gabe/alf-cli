use anyhow::Result;
use clap::Parser;

use alf::cli::{Cli, Commands};

fn main() -> Result<()> {
   env_logger::init();
   let cli = Cli::parse();

   match cli.command {
      Some(Commands::Search {
         query,
      }) => alf::tui::run(Some(query)),
      None => alf::tui::run(None),
      Some(Commands::Init {
         print_shell_hook: Some(shell),
      }) => alf::cli::init::print_shell_hook(&shell),
      Some(Commands::Init {
         print_shell_hook: None,
      }) => alf::cli::init::run_init_wizard(),
      Some(Commands::ShellHook {
         shell,
      }) => alf::cli::init::print_shell_hook(&shell),
      Some(Commands::Config {
         action,
      }) => alf::cli::config_cmd::run_config_action(action),
   }
}
