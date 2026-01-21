use std::{fs::{File, copy}, path::Path};
use std::io::Write;
use anyhow::Result;
use tar::Archive;
use xz2::read::XzDecoder;

pub(crate) async fn download(url: &str, dest: &Path) -> Result<()> {
   let resp = reqwest::get(url).await?;
   let mut file = File::create(dest)?;
   let bytes = resp.bytes().await?;
   file.write_all(&bytes)?;
   Ok(())
}

pub(crate) async fn extract(archive:&Path, target:&Path) -> Result<()> {
    let file = File::open(archive)?;
    let decompressor = XzDecoder::new(file);
    let mut archive = Archive::new(decompressor);
    archive.unpack(target);
    Ok(())
}