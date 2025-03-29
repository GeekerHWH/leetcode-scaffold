mod render;
use clap::{Parser, Subcommand};
use leetcode_scaffold::scaffold_scraper;
use render::Render;
use std::fs;

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
    /// render template for rust
    Rust,
    /// render template for go (Not Available for Now)
    Go,
    /// get leetcode problem description
    Leetcode {
        /// url of the leetcode problem
        #[arg(short, long)]
        url: String,
    },
}

#[tokio::main]
async fn main() {
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
        Some(Commands::Leetcode { url }) => {
            println!(
                "{:?}",
                scaffold_scraper::leetcode::get_leetcode_problem_description(url.to_string()).await
            );
        }
        _ => {}
    }
}
