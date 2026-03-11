//! First-run initialization wizard for alf.

use crate::config::{is_first_run, save_config, Config, GeneralConfig, UiConfig};
use crate::tui::themes::Theme;
use anyhow::Result;
use std::io::{self, Write};
use std::path::PathBuf;

/// Standard shell configuration files to check
const STANDARD_SHELL_FILES: &[&str] = &[".bashrc", ".zshrc", ".kshrc", "config.fish", ".profile", ".zprofile"];

/// Run the initialization wizard
pub fn run_init_wizard() -> Result<()> {
   // Check if already configured
   if !is_first_run()? {
      eprintln!("Config already exists at $HOME/.config/alf/config.toml");
      eprintln!("To reconfigure, run: alf config reset");
      return Ok(());
   }

   println!("Welcome to alf!\n");

   // Auto-detect standard shell files
   let home = std::env::var("HOME").map_err(|_| {
      let _ = anyhow::anyhow!("HOME environment variable not set");
   });
   let detected_files = detect_shell_files(&home.unwrap());

   println!("Detected shell files:");
   if detected_files.is_empty() {
      println!("  (none found)");
   } else {
      for file in &detected_files {
         println!("  ✓ {}", file);
      }
   }
   println!();

   // Prompt for additional files
   print!("Additional files? (comma-separated paths, or Enter to skip):\n> ");
   io::stdout().flush()?;

   let mut additional = String::new();
   io::stdin().read_line(&mut additional)?;

   let mut all_files = detected_files;
   if !additional.trim().is_empty() {
      for path in additional.split(',') {
         let trimmed = path.trim();
         if !trimmed.is_empty() {
            all_files.push(trimmed.to_string());
         }
      }
   }

   println!();

   // Theme selection
   println!("Choose a theme:");
   let themes = Theme::available_themes();
   for (i, theme_name) in themes.iter().enumerate() {
      println!("  {}) {}", i + 1, theme_name);
   }

   print!("> ");
   io::stdout().flush()?;

   let mut choice = String::new();
   io::stdin().read_line(&mut choice)?;

   let theme_idx = choice.trim().parse::<usize>().unwrap_or(1).saturating_sub(1);
   let selected_theme = themes.get(theme_idx).cloned().unwrap_or_else(|| {
      eprintln!("Invalid selection, using default theme.");
      "default".to_string()
   });

   println!();

   // Create and save config
   let config = Config {
      general: GeneralConfig { shell_files: all_files },
      ui: UiConfig { theme: selected_theme, keybind_mode: "vim".to_string() },
      ..Default::default()
   };

   save_config(&config)?;

   let config_path = crate::config::get_config_path()?;
   println!("Config saved to {}", config_path.display());
   println!("Run `alf` to start.");

   Ok(())
}

/// Detect which standard shell files exist in the home directory
pub fn detect_shell_files(home: &str) -> Vec<String> {
   STANDARD_SHELL_FILES
      .iter()
      .filter_map(|filename| {
         let mut path = PathBuf::from(home).join(filename);

         if *filename == "config.fish" {
            path = PathBuf::from(home).join(".config/fish").join(filename);
         }

         if path.exists() {
            Some(path.to_string_lossy().to_string())
         } else {
            None
         }
      })
      .collect()
}
