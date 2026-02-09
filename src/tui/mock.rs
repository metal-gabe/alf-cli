//! Mock data for TUI development and testing.

use std::path::PathBuf;

use crate::models::{AliasEntry, EntryType};

/// Generate a set of realistic mock alias and function entries.
///
/// This data is used during development to populate the TUI and
/// is also useful for unit/integration tests of the parser and search modules.
pub fn mock_entries() -> Vec<AliasEntry> {
   vec![
      // ── Aliases ──────────────────────────────────────────
      AliasEntry {
         name: "gs".to_string(),
         entry_type: EntryType::Alias,
         value: "git status".to_string(),
         comments: Some(vec!["Check git working tree status".to_string()]),
         source_file: PathBuf::from("~/.zshrc"),
      },
      AliasEntry {
         name: "ga".to_string(),
         entry_type: EntryType::Alias,
         value: "git add".to_string(),
         comments: Some(vec!["Stage files for commit".to_string()]),
         source_file: PathBuf::from("~/.zshrc"),
      },
      AliasEntry {
         name: "gc".to_string(),
         entry_type: EntryType::Alias,
         value: "git commit".to_string(),
         comments: Some(vec!["Create a new commit".to_string()]),
         source_file: PathBuf::from("~/.zshrc"),
      },
      AliasEntry {
         name: "gp".to_string(),
         entry_type: EntryType::Alias,
         value: "git push".to_string(),
         comments: Some(vec!["Push commits to remote".to_string()]),
         source_file: PathBuf::from("~/.zshrc"),
      },
      AliasEntry {
         name: "gl".to_string(),
         entry_type: EntryType::Alias,
         value: "git log --oneline --graph --decorate".to_string(),
         comments: Some(vec![
            "View compact git log with graph".to_string(),
            "Shows branch topology and tags".to_string(),
         ]),
         source_file: PathBuf::from("~/.zshrc"),
      },
      AliasEntry {
         name: "ll".to_string(),
         entry_type: EntryType::Alias,
         value: "ls -lah".to_string(),
         comments: Some(vec!["List files in long format with hidden files".to_string()]),
         source_file: PathBuf::from("~/.bashrc"),
      },
      AliasEntry {
         name: "..".to_string(),
         entry_type: EntryType::Alias,
         value: "cd ..".to_string(),
         comments: Some(vec!["Navigate up one directory".to_string()]),
         source_file: PathBuf::from("~/.bashrc"),
      },
      AliasEntry {
         name: "...".to_string(),
         entry_type: EntryType::Alias,
         value: "cd ../..".to_string(),
         comments: Some(vec!["Navigate up two directories".to_string()]),
         source_file: PathBuf::from("~/.bashrc"),
      },
      AliasEntry {
         name: "proj".to_string(),
         entry_type: EntryType::Alias,
         value: "cd ~/projects".to_string(),
         comments: Some(vec!["Navigate to projects directory".to_string()]),
         source_file: PathBuf::from("~/.zshrc"),
      },
      AliasEntry {
         name: "dps".to_string(),
         entry_type: EntryType::Alias,
         value: "docker ps".to_string(),
         comments: Some(vec!["List running Docker containers".to_string()]),
         source_file: PathBuf::from("~/.zshrc"),
      },
      AliasEntry {
         name: "grep".to_string(),
         entry_type: EntryType::Alias,
         value: "grep --color=auto".to_string(),
         comments: None,
         source_file: PathBuf::from("~/.bashrc"),
      },
      AliasEntry {
         name: "ports".to_string(),
         entry_type: EntryType::Alias,
         value: "lsof -i -P -n | grep LISTEN".to_string(),
         comments: Some(vec!["Show all listening ports on this machine".to_string()]),
         source_file: PathBuf::from("~/.zshrc"),
      },
      // ── Functions ────────────────────────────────────────
      AliasEntry {
         name: "mkcd".to_string(),
         entry_type: EntryType::Function,
         value: "mkcd() {\n    mkdir -p \"$1\"\n    cd \"$1\"\n}".to_string(),
         comments: Some(vec!["Create a directory and cd into it".to_string()]),
         source_file: PathBuf::from("~/.zshrc"),
      },
      AliasEntry {
         name: "extract".to_string(),
         entry_type: EntryType::Function,
         value: "extract() {\n    if [ -f \"$1\" ]; then\n        case \"$1\" in\n            \
                    *.tar.bz2) tar xjf \"$1\" ;;\n            *.tar.gz)  tar xzf \"$1\" ;;\n       \
                     *.bz2)     bunzip2 \"$1\" ;;\n            *.rar)     unrar x \"$1\" ;;\n       \
                     *.gz)      gunzip \"$1\" ;;\n            *.tar)     tar xf \"$1\" ;;\n         \
                   *.tbz2)    tar xjf \"$1\" ;;\n            *.tgz)     tar xzf \"$1\" ;;\n         \
                   *.zip)     unzip \"$1\" ;;\n            *.Z)       uncompress \"$1\" ;;\n         \
                   *)         echo \"'$1' cannot be extracted\" ;;\n        esac\n    else\n        \
                    echo \"'$1' is not a valid file\"\n    fi\n}"
            .to_string(),
         comments: Some(vec![
            "Extract various archive formats".to_string(),
            "Supports tar, gz, bz2, rar, zip, and more".to_string(),
            "Usage: extract <file>".to_string(),
         ]),
         source_file: PathBuf::from("~/.bashrc"),
      },
      AliasEntry {
         name: "gbr".to_string(),
         entry_type: EntryType::Function,
         value: "gbr() {\n    git branch --sort=-committerdate\n}".to_string(),
         comments: Some(vec!["List git branches sorted by last commit date".to_string()]),
         source_file: PathBuf::from("~/.zshrc"),
      },
      AliasEntry {
         name: "docker_cleanup".to_string(),
         entry_type: EntryType::Function,
         value: "docker_cleanup() {\n    echo \"Removing stopped containers...\"\n    docker \
                    container prune -f\n    echo \"Removing dangling images...\"\n    docker image \
                    prune -f\n    echo \"Removing unused volumes...\"\n    docker volume prune \
                    -f\n    echo \"Docker cleanup complete.\"\n}"
            .to_string(),
         comments: Some(vec![
            "Remove all stopped containers, dangling images, and unused volumes".to_string(),
            "Warning: this is destructive and cannot be undone".to_string(),
         ]),
         source_file: PathBuf::from("~/.zshrc"),
      },
      AliasEntry {
         name: "weather".to_string(),
         entry_type: EntryType::Function,
         value: "weather() {\n    curl -s \"wttr.in/${1:-}\"\n}".to_string(),
         comments: None,
         source_file: PathBuf::from("~/.bashrc"),
      },
      AliasEntry {
         name: "backup".to_string(),
         entry_type: EntryType::Function,
         value: "backup() {\n    local src=\"$1\"\n    local timestamp\n    \
                    timestamp=$(date +%Y%m%d_%H%M%S)\n    local dest=\"${src}.backup_${timestamp}\"\n    \
                    cp -r \"$src\" \"$dest\"\n    echo \"Backed up to: $dest\"\n}"
            .to_string(),
         comments: Some(vec![
            "Create a timestamped backup of a file or directory".to_string(),
            "Usage: backup <path>".to_string(),
         ]),
         source_file: PathBuf::from("~/.zshrc"),
      },
   ]
}
