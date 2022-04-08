//! src/blockchain_info.rs

use crate::BlockchainAddress;
use crate::BlockchainStatus;
use crate::BlockchainTransaction;
use {dotenv, reqwest, serde_json::Result, tokio};

const HOST_ROOT: &str = "https://btcbook.nownodes.io/api/"; // GET status

#[tokio::main]
pub async fn send_request(url: &str) -> String {
    let client = reqwest::Client::new();
    client
        .get(url)
        .header(
            "api-key",
            dotenv::var("API_KEY").expect("Could not find key: API_KEY"),
        )
        .send()
        .await
        .expect("Failed to get response")
        .text()
        .await
        .expect("Failed to convert payload")
}

pub fn blockchain_status_request() -> BlockchainStatus {
    let response = send_request(&HOST_ROOT);
    // println!("{}", response);
    serde_json::from_str(&response).expect("Failed to parse JSON")
}

pub fn blockchain_address_request(address: &str) -> BlockchainAddress {
    let response = send_request(&[HOST_ROOT, "v2/address/", &address].join(""));
    serde_json::from_str(&response).expect("Failed to parse JSON")
}
/*
pub fn blockchain_transaction_request(transaction: &str) -> BlockchainTransaction {
    let response = send_request(&[HOST_ROOT, "v2/tx/", &transaction].join(""));
    serde_json::from_str(&response).expect("Failed to parse JSON")
}
*/