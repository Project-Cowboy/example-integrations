mod run;

use helpers::client::upload;
use methods::{COWBOY_EXAMPLE_APPS_ELF, COWBOY_EXAMPLE_APPS_ID};
use run::run;

use clap::{Parser, Subcommand};

const INTEGRATION_HOST_NAME: &[u8] = b"www.tiktok.com";
const INTEGRATION_URL_PATH: &[u8] = b"/aweme/v2/data/insight/";

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        #[arg(long)]
        node_url: String,
    },
    Upload {
        #[arg(long)]
        node_url: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Run { node_url } => {
            run(&node_url).await;
        }
        Commands::Upload { node_url } => {
            upload(
                &node_url,
                COWBOY_EXAMPLE_APPS_ID,
                COWBOY_EXAMPLE_APPS_ELF.to_vec(),
                INTEGRATION_HOST_NAME.to_vec(),
                INTEGRATION_URL_PATH.to_vec(),
            )
            .await
            .unwrap();
        }
    }
}
