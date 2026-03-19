use clap::{Parser, Subcommand};
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
    match cli.command {
        Commands::Get { key } => {
            eprintln!("unimplemented");
            exit(1);
        }
        Commands::Set { key, value } => {
            eprintln!("unimplemented");
            exit(1);
        }
        Commands::Rm { key } => {
            eprintln!("unimplemented");
            exit(1);
        }
    }
}
