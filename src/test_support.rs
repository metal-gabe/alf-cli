//! Shared helpers for tests that need an isolated `$HOME`.

use std::env::{remove_var, set_var, var, var_os};
use std::ffi::{OsStr, OsString};
use std::fs;
use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard};
use tempfile::TempDir;

static ENV_MUTEX: Mutex<()> = Mutex::new(());

/// A temporary `$HOME` directory that is restored when dropped
///
/// Environment mutation is serialized through a shared mutex, so tests that swap `$HOME` cannot
/// race one another when the suite runs on threads instead of processes.
pub struct TempHome {
   _guard: MutexGuard<'static, ()>,
   dir: TempDir,
   old_home: Option<String>,
   overrides: Vec<(String, Option<OsString>)>,
}

impl TempHome {
   /// Create a temporary directory and point `$HOME` at it
   pub fn new() -> Self {
      let guard = ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
      let temp_dir = TempDir::new().expect("Should create temp dir");
      let old_home = var("HOME").ok();
      set_var("HOME", temp_dir.path());
      Self {
         _guard: guard,
         dir: temp_dir,
         old_home,
         overrides: Vec::new(),
      }
   }

   /// Absolute path of `name` inside the temporary home, as a string
   pub fn absolute(
      &self,
      name: &str,
   ) -> String {
      self.path().join(name).to_string_lossy().to_string()
   }

   /// Path of the temporary home
   pub fn path(&self) -> PathBuf {
      self.dir.path().to_path_buf()
   }

   /// Override an environment variable until this temporary home is dropped
   ///
   /// The original value is restored on drop, so a test can point home resolution at an empty or
   /// deliberately malformed value without leaking it into the rest of the suite.
   pub fn set_env(
      &mut self,
      key: &str,
      value: impl AsRef<OsStr>,
   ) {
      self.remember(key);
      set_var(key, value);
   }

   /// Create an empty file named `name` inside the temporary home
   pub fn touch(
      &self,
      name: &str,
   ) {
      fs::File::create(self.path().join(name)).expect("Should create file");
   }

   /// Remove an environment variable until this temporary home is dropped
   pub fn unset_env(
      &mut self,
      key: &str,
   ) {
      self.remember(key);
      remove_var(key);
   }

   /// Record the current value of `key` the first time it is overridden, so `Drop` can put it back
   fn remember(
      &mut self,
      key: &str,
   ) {
      if !self.overrides.iter().any(|(name, _)| name == key) {
         self.overrides.push((key.to_string(), var_os(key)));
      }
   }
}

impl Default for TempHome {
   fn default() -> Self {
      Self::new()
   }
}

impl Drop for TempHome {
   fn drop(&mut self) {
      for (key, value) in self.overrides.drain(..) {
         match value {
            Some(value) => set_var(&key, value),
            None => remove_var(&key),
         }
      }

      match &self.old_home {
         Some(home) => set_var("HOME", home),
         None => remove_var("HOME"),
      }
   }
}
