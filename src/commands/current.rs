use anyhow::{Ok, Result};

use crate::config::{self, Config};

pub(crate) fn run() -> Result<()>{
   let cfg = config::load()?;
   match cfg.current {
       Some(v) => println!("{}", v),
       None => println!("none"),
   }

   Ok(())
}