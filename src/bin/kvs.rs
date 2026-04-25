use clap::{Parser, Subcommand};
use kvs::{KvStore, Result};
use std::env::current_dir;

#[derive(Parser)]
#[command(
    version,
    about,
    subcommand_required = true,
    disable_help_subcommand = true
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Get { key: String },
    Set { key: String, value: String },
    Rm { key: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut store = KvStore::open(current_dir()?)?;
    match cli.command {
        Commands::Get { key } => {
            let value = store.get(&key)?;
            match value {
                Some(value) => println!("{value}"),
                None => {
                    eprintln!("Couldn't find a value for key {key}");
                }
            }
        }
        Commands::Set { key, value } => {
            store.set(key, value)?;
        }
        Commands::Rm { key } => {
            store.remove(key)?;
        }
    }
    Ok(())
}
