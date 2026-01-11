use alloy::providers::{Provider, ProviderBuilder};
use alloy::primitives::{Address, utils::format_units};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let rpc_url = "https://arbitrum-sepolia-rpc.publicnode.com".parse()?;
    let provider = ProviderBuilder::new().connect_http(rpc_url);

    let target_address: Address = "0x3019826431baaacc91604A595791a2d84acf5a56".parse()?;

    let balance_wei = provider.get_balance(target_address).await?;
    let balance_eth = format_units(balance_wei, 18)?;

    println!("\nBalance of address {}:\n{} ETH", target_address, balance_eth);

    Ok(())
}