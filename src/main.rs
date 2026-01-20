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
fn main() -> Result<()> {
    init::init_rnvm()?;
    let cli = Cli::parse();

    match cli.commands {
       Commands::Install { version } => {
         println!("Installing node version : {}", version);
         commands::install::run(version)?;
       }, 
       Commands::Use { version } => {
         println!("Currently using : {}", version);
         commands::useCommand::run(version)?;
       }, 
       Commands::Ls => { //ez
         println!("Installed versions");
         commands::ls::run()?;
       }, 
       Commands::Current => { //ez
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
