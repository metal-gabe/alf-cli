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
      general: GeneralConfig {
         shell_files: all_files,
         ..Default::default()
      },
      ui: UiConfig {
         theme: selected_theme,
         keybind_mode: "vim".to_string(),
      },
      ..Default::default()
   };

   save_config(&config)?;

   let config_path = crate::config::get_config_path()?;
   println!("Config saved to {}", config_path.display());
   println!();
   println!("Shell integration");
   println!("─────────────────");
   println!("Add the following to your shell config to enable command-line population.");
   println!("This installs the `alf` command wrapper.\n");

   println!("For zsh (add to ~/.zshrc):");
   println!("{}\n", get_shell_hook("zsh"));

   println!("For bash (add to ~/.bashrc):");
   println!("{}\n", get_shell_hook("bash"));

   println!("Or run: eval \"$(alf activate <zsh|bash>)\"");
   println!();
   println!("Usage:");
   println!("  - Type `alf` at the prompt to open the picker.");
   println!("  - In the TUI, Tab populates the prompt with the entry; Enter runs it.");
   println!("  - Note: in bash, Tab cannot populate the readline buffer;");
   println!("    it will print the entry instead.");
   println!();
   println!("Run `alf` to start.");

   Ok(())
}

/// Print the shell integration wrapper for a given shell
pub fn print_shell_hook(shell: &str) -> Result<()> {
   match shell.to_lowercase().as_str() {
      "zsh" | "bash" => {
         println!("{}", get_shell_hook(shell));
         Ok(())
      },
      _ => {
         eprintln!("Unsupported shell: {}. Use 'zsh' or 'bash'.", shell);
         Err(anyhow::anyhow!("Unsupported shell: {}", shell))
      },
   }
}

fn get_shell_hook(shell: &str) -> &'static str {
   match shell.to_lowercase().as_str() {
      "zsh" => {
         r#"alf() {
  local tmp action entry rc
  tmp="$(mktemp)" || return 1
  ALF_OUTPUT="$tmp" command alf "$@"
  rc=$?
  if [[ -s "$tmp" ]]; then
    action="$(sed -n '1p' "$tmp")"
    entry="$(sed -n '2p' "$tmp")"
    rm -f "$tmp"
    if [[ -n "$entry" ]]; then
      if [[ "$action" == "execute" ]]; then
        print -s -- "$entry"
        fc -A
        if (( ${+functions[_atuin_preexec]} )); then
          _atuin_preexec "$entry"
          eval -- "$entry"
          _atuin_precmd
        else
          eval -- "$entry"
        fi
        return
      else
        print -z -- "$entry"
      fi
    fi
  else
    rm -f "$tmp"
  fi
  return $rc
}"#
      },
      "bash" => {
         r#"alf() {
  local tmp action entry rc
  tmp="$(mktemp)" || return 1
  ALF_OUTPUT="$tmp" command alf "$@"
  rc=$?
  if [[ -s "$tmp" ]]; then
    action="$(sed -n '1p' "$tmp")"
    entry="$(sed -n '2p' "$tmp")"
    rm -f "$tmp"
    if [[ -n "$entry" ]]; then
      if [[ "$action" == "execute" ]]; then
        history -s -- "$entry"
        history -a
        eval -- "$entry"
        return
      else
        printf '%s\n' "$entry"
      fi
    fi
  else
    rm -f "$tmp"
  fi
  return $rc
}"#
      },
      _ => "",
   }
}

/// Detect which standard shell files exist in the home directory
pub(super) fn detect_shell_files(home: &str) -> Vec<String> {
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

#[cfg(test)]
#[path = "init_tests.rs"]
mod init_tests;
