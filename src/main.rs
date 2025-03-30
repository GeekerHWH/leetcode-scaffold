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
    /// get leetcode problem description (Only for development)
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
            // create a new directory for go files
            let dir_path = "./solution";
            match fs::create_dir_all(dir_path) {
                Ok(_) => println!("Created directory: {}", dir_path),
                Err(e) => println!("Failed to create directory: {}", e),
            }

            let mut go = render::go::GoLang::new(dir_path);
            match go.render_unit_test() {
                Ok(_) => println!("Successfully rendered Go templates"),
                Err(err) => println!("Failed to render Go templates: {}", err),
            }
        }
        Some(Commands::Leetcode { url }) => {
            let body = reqwest::get(url).await.unwrap().text().await.unwrap();
            scaffold_scraper::leetcode::print_formatted_description(
                &scaffold_scraper::leetcode::get_leetcode_problem_description(body),
            );
        }
        _ => {}
    }
}
