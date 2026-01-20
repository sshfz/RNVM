use anyhow::{Ok, Result};
use std::fs;

use crate::paths::versions_dir;

pub(crate) fn run() -> Result<()> {
  let dir = versions_dir();  

  if !dir.exists() {
    return Ok(());
  }

  for entry in fs::read_dir(dir)? {
    let entry = entry?;
    if entry.path().is_dir() {
        if let Some(name) = entry.file_name().to_str() {
            println!("{}", name);
        }
    }
  }

  Ok(())
}