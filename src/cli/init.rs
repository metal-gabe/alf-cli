//! First-run initialization wizard for alf.

use crate::cli::config_cmd::resolve_new_source_files;
use crate::config::{is_first_run, resolve_home_dir, save_config, Config, GeneralConfig, UiConfig};
use crate::tui::themes::Theme;
use anyhow::{Context, Result};
use std::io::{self, Write};
use std::path::PathBuf;

/// Standard shell configuration files to check
const STANDARD_SHELL_FILES: &[&str] = &[".bashrc", ".zshrc", ".kshrc", "config.fish", ".profile", ".zprofile"];

/// Run the initialization wizard
pub fn run_init_wizard() -> Result<()> {
   if !is_first_run()? {
      eprintln!("Config already exists at $HOME/.config/alf/config.toml");
      eprintln!("To reconfigure, run: alf config reset");
      return Ok(());
   }

   println!("Welcome to alf!\n");

   let home = resolve_home_dir().context("Could not determine the home directory: set HOME or USERPROFILE")?;
   let detected_files = detect_shell_files(&home.to_string_lossy());

   println!("Detected shell files:");
   if detected_files.is_empty() {
      println!("  (none found)");
   } else {
      for file in &detected_files {
         println!("  ✓ {}", file);
      }
   }
   println!();

   let all_files = prompt_additional_files(detected_files)?;

   println!();

   println!("Choose a theme:");
   let themes = Theme::available_themes();
   for (i, theme_name) in themes.iter().enumerate() {
      println!("  {}) {}", i + 1, theme_name);
   }

   let selected_theme = prompt_theme(&themes)?;

   println!();

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

/// Ask for extra shell files until every given path is valid, returning them after `detected_files`
///
/// Paths get the same checks as `alf config add`. Empty input, including end of input, skips the step.
fn prompt_additional_files(detected_files: Vec<String>) -> Result<Vec<String>> {
   let mut config = Config::default();
   config.general.shell_files = detected_files;

   loop {
      print!("Additional files? (comma-separated paths, or Enter to skip):\n> ");
      io::stdout().flush()?;

      let mut additional = String::new();
      io::stdin().read_line(&mut additional)?;

      let raw_paths: Vec<String> =
         additional.split(',').map(str::trim).filter(|path| !path.is_empty()).map(str::to_string).collect();

      match resolve_new_source_files(&config, &raw_paths) {
         Ok((to_add, _)) => {
            config.general.shell_files.extend(to_add);
            return Ok(config.general.shell_files);
         },
         Err(error) => eprintln!("{}\nTry again, or press Enter to skip.", error),
      }
   }
}

/// Ask for a theme from the numbered `themes` menu until the answer is valid
fn prompt_theme(themes: &[String]) -> Result<String> {
   loop {
      print!("(Enter for default)\n> ");
      io::stdout().flush()?;

      let mut choice = String::new();
      io::stdin().read_line(&mut choice)?;

      match parse_theme_choice(&choice, themes) {
         Some(theme) => return Ok(theme),
         None => eprintln!("Invalid selection: enter a number from 1 to {}.", themes.len()),
      }
   }
}

/// Resolve a theme menu answer, where empty input, including end of input, picks the `default` theme
fn parse_theme_choice(
   choice: &str,
   themes: &[String],
) -> Option<String> {
   let choice = choice.trim();

   if choice.is_empty() {
      return Some("default".to_string());
   }

   let number = choice.parse::<usize>().ok()?;
   themes.get(number.checked_sub(1)?).cloned()
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
  if [[ "$1" == "which" ]]; then
    local arg name=""
    for arg in "${@:2}"; do
      case "$arg" in
        '-h'|'--help') name=""; break ;;
        '-'*) ;;
        *) [[ -z "$name" ]] && name="$arg" ;;
      esac
    done
    if [[ -n "$name" ]]; then
      local -x ALF_LOOKUP_NAME="$name" ALF_WHICH_RESULT="" ALF_TYPE_RESULT=""
      ALF_WHICH_RESULT="$(which "$name" 2>/dev/null)" || ALF_WHICH_RESULT=""
      ALF_TYPE_RESULT="$(type "$name" 2>/dev/null)" || ALF_TYPE_RESULT=""
    fi
  fi
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
  if [[ "$1" == "which" ]]; then
    local arg name=""
    for arg in "${@:2}"; do
      case "$arg" in
        '-h'|'--help') name=""; break ;;
        '-'*) ;;
        *) [[ -z "$name" ]] && name="$arg" ;;
      esac
    done
    if [[ -n "$name" ]]; then
      local -x ALF_LOOKUP_NAME="$name" ALF_WHICH_RESULT="" ALF_TYPE_RESULT=""
      ALF_WHICH_RESULT="$(which "$name" 2>/dev/null)" || ALF_WHICH_RESULT=""
      ALF_TYPE_RESULT="$(type "$name" 2>/dev/null)" || ALF_TYPE_RESULT=""
    fi
  fi
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
pub(crate) fn detect_shell_files(home: &str) -> Vec<String> {
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
