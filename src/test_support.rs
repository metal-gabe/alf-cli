//! Shared helpers for tests that need an isolated `$HOME`.

use std::env::{remove_var, set_var, var};
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

   /// Create an empty file named `name` inside the temporary home
   pub fn touch(
      &self,
      name: &str,
   ) {
      fs::File::create(self.path().join(name)).expect("Should create file");
   }
}

impl Default for TempHome {
   fn default() -> Self {
      Self::new()
   }
}

impl Drop for TempHome {
   fn drop(&mut self) {
      match &self.old_home {
         Some(home) => set_var("HOME", home),
         None => remove_var("HOME"),
      }
   }
}
