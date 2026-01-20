use std::fs;

use anyhow::{Result, bail};

use crate::paths::versions_dir;

pub(crate) fn run(version: String) -> Result<()> {
   let dir = versions_dir().join(format!("v{}", version));  

   if !dir.exists() {
      bail!("Node version {} is not installed", version);
   }

   fs::remove_dir_all(&dir)?;
   println!("uninstalled version {}", version);
  
   Ok(())
} 