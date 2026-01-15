use alloy::providers::{Provider, ProviderBuilder};
use alloy::primitives::{Address, U256, utils::format_units};
use alloy::network::{TransactionBuilder, EthereumWallet};
use alloy::signers::local::PrivateKeySigner;
use alloy::rpc::types::TransactionRequest;
use std::error::Error;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let rpc_url = "https://arbitrum-sepolia-rpc.publicnode.com".parse()?;
    dotenv().ok();
    let private_key = env::var("PRIVATE_KEY")?;
    let signer: PrivateKeySigner = private_key.parse()?;
    let wallet = EthereumWallet::from(signer);

    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .connect_http(rpc_url);

    let to_address: Address = "0xF6FEBd05224397E58CFd604220b31f84c089A2e1".parse()?;

    let tx = TransactionRequest::default()
        .with_to(to_address)
        .with_value(U256::from(10_000_000_000_000_000_u64)) // 0.001 ETH in wei
        .with_gas_price(20_000_000_000); // 20 Gwei
        // .with_gas_limit(21_000); // Standard gas limit for ETH transfer

    let pending_tx = provider.send_transaction(tx).await?;
    let tx_hash = pending_tx.tx_hash();
    println!("\nTransaction sent! Hash:\n{:?}", tx_hash);
    // println!("From: {}\nTo: {}\nValue: 0.001 ETH", from_address, to_address);
    println!("View on Arbiscan: https://sepolia.arbiscan.io/tx/{:?}", tx_hash);
    Ok(())  
}