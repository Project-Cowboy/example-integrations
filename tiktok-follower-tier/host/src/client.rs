use subxt::{OnlineClient, PolkadotConfig};
use subxt_signer::sr25519::dev;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use methods::{
    COWBOY_EXAMPLE_APPS_ELF, COWBOY_EXAMPLE_APPS_ID
};

use crate::api::api as cowboy_api;

// Mathod to upload the example program, with its particular uri, to the chain, using the well-known bob account
pub async fn upload(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let api = OnlineClient::<PolkadotConfig>::from_url(url).await?;
    let program_id: [u32; 8] = COWBOY_EXAMPLE_APPS_ID;
    let program = COWBOY_EXAMPLE_APPS_ELF.to_vec();

    // Url identifier of integration: www.tiktok.com/aweme/v2/data/insight/
    let selector_app_host = b"www.tiktok.com".to_vec();
    let selector_app_uri = b"/aweme/v2/data/insight/".to_vec();

    let add_program_call = cowboy_api::tx().cowboy().add_program(program_id, program, selector_app_host, selector_app_uri);

    let from = dev::bob();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&add_program_call, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    Ok(())
}


#[derive(Serialize)]
struct ProveRequest {
    data: String,
    app_id: Option<[u32; 8]>,
}

#[derive(Deserialize, Debug)]
pub struct ReceiptResponse {
    pub receipt: String,
}

// Get the current core proof id for verifying any application-specific proof on Cowboy
pub async fn get_core_proof_id(
    url: &str,
) -> Result<[u32; 8], Box<dyn std::error::Error>> {
    let api = OnlineClient::<PolkadotConfig>::from_url(url).await?;
    let core_proof_key = cowboy_api::storage().cowboy().core_proof_id();
    let core_proof_id = api
        .storage()
        .at_latest()
        .await
        .unwrap()
        .fetch(&core_proof_key)
        .await
        .unwrap();
    
    Ok(core_proof_id.expect("Core proof should exist onchain"))
}

// Connect to the prover and request a proof for the application-specific integration
pub async fn request_program_core_proof(
    url: &str,
    data_hex: &str,
    app_id: Option<[u32; 8]>,
) -> Result<ReceiptResponse, Box<dyn std::error::Error>> {
    let client = Client::new();

    let req_body = ProveRequest {
        data: data_hex.to_string(),
        app_id,
    };

    let response = client
        .post(url)
        .json(&req_body)
        .send()
        .await?;

    if !response.status().is_success() {
        Err(format!("HTTP error: {}", response.status()))?
    } else {
        let parsed: ReceiptResponse = response.json().await?;
        Ok(parsed)
    }
}
