use anyhow::{Result, bail};

use crate::{config, paths::{rnvm_dirs, versions_dir}};

pub(crate) fn run(version: String) -> Result<()> {
    let dir = versions_dir().join(format!("v{}", version));

    if !dir.exists() {
       bail!("node version {} does is not installed", version);
    }

    let mut config = config::load()?;
    config.current = Some(format!("v{}", version));
    config::save(&config)?;

        println!(
        r#"export RVM_HOME="{}"
export PATH="$RVM_HOME/versions/v{}/bin:$PATH""#,
        rnvm_dirs().display(),
        version
    );

    Ok(())
}