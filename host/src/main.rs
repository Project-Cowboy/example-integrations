mod run;
mod client;
pub mod api;

use run::run;
use client::upload;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run,
    Upload
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Run => {
            run().await;
        },
        Commands::Upload => {
            upload().await.unwrap();
        }
    }
}
