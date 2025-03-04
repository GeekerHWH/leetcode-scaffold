mod render;
use clap::{Parser, Subcommand};
use render::Render;
use std::{fs, process::exit};

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
            // create lib.rs at the current directory
            match fs::exists("./lib.rs") {
                Ok(true) => println!("already has lib.rs, overwrite it."),
                Ok(false) => println!("lib.rs not exist, create new one"),
                _ => (),
            }
            let mut rs = render::rust::RustLang::new("./lib.rs");
            match rs.render_unit_test() {
                Ok(_) => println!("sucssesfully render unit test"),
                Err(err) => println!("failed to  render unit test: {}", err),
            }
        }
        Some(Commands::Go) => {
            println!("Go");
        }
        _ => {}
    }
}
