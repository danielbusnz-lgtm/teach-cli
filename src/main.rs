use clap::{Parser, Subcommand};
use teach_cli::fundamentals::{python, rust};

#[derive(Parser)]
#[command(name = "mycli")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Rust,
    Python,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Python => {
            let value = python::rand_topic();
            println!("{}", value.unwrap());
        }
        Commands::Rust => {
            let value = rust::rand_topic();
            println!("{}", value.unwrap());
        }
    }
}
