use clap::{Parser, Subcommand};
use kvs::KvStore;
use std::process::exit;

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

fn main() {
    let cli = Cli::parse();
    let mut store = KvStore::new();
    match cli.command {
        Commands::Get { key } => match store.get(&key) {
            Some(value) => println!("{value}"),
            None => {
                eprintln!("Couldn't find a value for key {key}");
                exit(1);
            }
        },
        Commands::Set { key, value } => {
            store.set(key, value);
        }
        Commands::Rm { key } => {
            store.remove(key);
        }
    }

    exit(0);
}
