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
    Run {
        // #[arg(default_value = "http://localhost:1881")]
        // prover_url: String,
        #[arg(long)]
        node_url: String
    },
    Upload {
        #[arg(long)]
        node_url: String
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Run { node_url } => {
            run(&node_url).await;
        },
        Commands::Upload { node_url } => {
            upload(&node_url).await.unwrap();
        }
    }
}
