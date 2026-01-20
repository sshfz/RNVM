use std::fs;

use anyhow::{Result, Context};

use crate::paths::{bin_dir, config_file, versions_dir};

pub(crate) fn init_rnvm() -> Result<()> {
   fs::create_dir_all(versions_dir()) 
    .context("failed to create versions dir")?;

   fs::create_dir_all(bin_dir())
    .context("failed to create bin dir")?;

   if !config_file().exists() {
      fs::write(config_file(), "{}")
        .context("failed to create config file")?;
   }

   Ok(())
}