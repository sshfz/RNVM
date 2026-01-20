use anyhow::{Result, bail};
use std::{fmt::format, fs};

use crate::paths::versions_dir;

pub(crate) fn run(version: String) -> Result<()> {
    let dir = versions_dir().join(format!("v{}", version));

    if dir.exists() {
        bail!("node version {} already exists", version);
    }

    fs::create_dir_all(&dir)?;
    println!("Installed node version: {}", version);

    Ok(())
}