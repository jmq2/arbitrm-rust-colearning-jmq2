use alloy::providers::{Provider, ProviderBuilder};
use alloy::primitives::{utils::format_units};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let rpc_url = "https://arbitrum-sepolia-rpc.publicnode.com".parse()?;
    let provider = ProviderBuilder::new().connect_http(rpc_url);
    let gas_price_wei = provider.get_gas_price().await?; // Get current gas price

    let gas_fee_wei = gas_price_wei * 21_000u128; // Estimate gas fee for a standard transaction (21,000 gas units)
    let gas_fee_eth = format_units(gas_fee_wei, 18)?; // Convert gas fee from wei to ether

    println!("\nEstimated gas fee for a standard transaction:\n{} ETH", gas_fee_eth);
    Ok(())
}