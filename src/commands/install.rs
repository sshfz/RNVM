use anyhow::{Result, bail};
use std::{fmt::format, fs};

use crate::{node::{download, url::url}, paths::versions_dir};

pub(crate) async fn run(version: String) -> Result<()> {
    let dir = versions_dir().join(format!("v{}", version));

    if dir.exists() {
        bail!("node version {} already exists", version);
    }

    let url = url(&version);
    let tmp = std::env::temp_dir().join(format!("node-v{}.tar.xz", version));

    println!("Downloading, {}", &url);
    download::download(&url, &tmp).await?;

    let extract_dir = std::env::temp_dir().join(format!("node-v{}", version));
    fs::create_dir_all(&extract_dir);

    println!("extracting...");
    download::extract(&tmp, &extract_dir).await?;

    let extracted = extract_dir
        .read_dir()?
        .next()
        .unwrap()?
        .path();

    fs::rename(extracted, &dir);

    println!("Installed node version: {}", version);
    Ok(())
}