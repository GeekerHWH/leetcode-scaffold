use clap::{Parser, Subcommand};

/// Simple program to greet a person
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Specify the language of the scaffold
    #[command(subcommand)]
    language: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Rust,
    Go,
}

fn main() {
    let cli = Cli::parse();
    match &cli.language {
        Some(Commands::Rust) => {
            println!("Rust");
        }
        Some(Commands::Go) => {
            println!("Go");
        }
        _ => {}
    }
}
