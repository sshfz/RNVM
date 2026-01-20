use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};
use std::{fmt::write, fs};

use crate::paths::config_file;

#[derive(Serialize, Deserialize, Default)]
pub(crate) struct Config {
  pub(crate) current: Option<String>, 
}

pub(crate) fn load() -> Result<Config> {
  let content = fs::read_to_string(config_file())?;
  Ok(serde_json::from_str(&content)?)
}

pub(crate) fn save(cfg: &Config) -> Result<()> {
  fs::write(config_file(), serde_json::to_string_pretty(cfg)?)?;
  Ok(())
}