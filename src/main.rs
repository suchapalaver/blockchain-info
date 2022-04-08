//! src/main.rs

use {
    blockchain_info::blockchain_info::blockchain_address_request,
    blockchain_info::blockchain_status::BlockchainStatus,
    blockchain_info::blockchain_address::BlockchainAddress,
    blockchain_info::blockchain_transaction::BlockchainTransaction,
    dotenv,
};

fn blockchain_info_app(address: &str) {
    let blockchain_status: BlockchainStatus = blockchain_info::blockchain_status_request();
    println!("\n\nQuerying {} - chain: {}\n\n", &blockchain_status.blockbook.coin, &blockchain_status.backend.chain);

    let blockchain_address: BlockchainAddress = blockchain_info::blockchain_address_request(address);
    println!("\n\nAnalyzing transactions for Bitcoin address {}", &blockchain_address.address);
}

fn main() {
    let entered_address = dotenv::var("WALLET").expect("Error reading env var");
    blockchain_info_app(&entered_address);
}
