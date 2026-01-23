use clap::{Parser, Subcommand};
mod paths;
mod init;
mod commands;
mod config;
mod node;

use anyhow::{Ok, Result};

#[derive(Parser)]
#[command(name="rnvm")]
#[command(about="node version manager")]
struct Cli {
   #[command(subcommand)]
   commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Install { version: String },
    Use { version: String },
    Ls, 
    Current, 
    Uninstall { version: String },
}

#[tokio::main]
async fn main() -> Result<()> {
    init::init_rnvm()?;
    let cli = Cli::parse();

    match cli.commands {
       Commands::Install { version } => {
         println!("Installing node version : {}", version);
         commands::install::run(version).await?;
       }, 
       Commands::Use { version } => {
        //  println!("use {}", version);
         commands::useCommand::run(version)?;
       }, 
       Commands::Ls => {
         println!("Installed versions");
         commands::ls::run()?;
       }, 
       Commands::Current => {
         println!("Current node verison");
         commands::current::run()?;
       }, 
       Commands::Uninstall { version } => {
         println!("Uninstalling node version : {}", version);
         commands::uninstall::run(version)?;
       }
       _ => {}
    }

    Ok(())
}
