//! Tests for the `ALF_OUTPUT` file contents

use super::app::ExitAction;
use super::build_exit_output;
use crate::config::AliasExpansion;
use crate::models::{AliasEntry, EntryType};
use std::path::PathBuf;

fn entry(entry_type: EntryType) -> AliasEntry {
   AliasEntry {
      comments: None,
      entry_type,
      name: "gs".to_string(),
      source_file: PathBuf::from("/tmp/.zshrc"),
      value: "git status".to_string(),
   }
}

#[test]
fn build_exit_output_picks_the_value_for_each_action_type_and_expansion() {
   let cases = [
      (ExitAction::Execute, EntryType::Alias, AliasExpansion::Name, "execute\ngs"),
      (ExitAction::Execute, EntryType::Alias, AliasExpansion::Script, "execute\ngs"),
      (ExitAction::Execute, EntryType::Function, AliasExpansion::Name, "execute\ngs"),
      (ExitAction::Execute, EntryType::Function, AliasExpansion::Script, "execute\ngs"),
      (ExitAction::Populate, EntryType::Alias, AliasExpansion::Name, "populate\ngs"),
      (ExitAction::Populate, EntryType::Alias, AliasExpansion::Script, "populate\ngit status"),
      (ExitAction::Populate, EntryType::Function, AliasExpansion::Name, "populate\ngs"),
      (ExitAction::Populate, EntryType::Function, AliasExpansion::Script, "populate\ngs"),
   ];

   for (action, entry_type, alias_expansion, expected) in cases {
      assert_eq!(
         build_exit_output(action, &entry(entry_type), alias_expansion),
         expected,
         "{action:?} {entry_type:?} {alias_expansion:?}"
      );
   }
}
