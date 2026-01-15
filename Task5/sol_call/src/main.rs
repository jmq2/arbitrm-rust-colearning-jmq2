use alloy::providers::{Provider, ProviderBuilder};
use alloy::primitives::{Address};
use alloy::sol;
use std::error::Error;
use std::str::FromStr;

sol! {
    #[sol(rpc)]
    contract MyERC20 {
        function name() external view returns (string);
        function symbol() external view returns (string);
        function decimals() external view returns (uint8);
        function totalSupply() external view returns (uint256);
        // function balanceOf(address account) external view returns (uint256);
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let rpc_url = "https://arbitrum-sepolia-rpc.publicnode.com".parse()?;
    let provider = ProviderBuilder::new().connect_http(rpc_url);

    let contract_address: Address = Address::from_str("0x75faf114eafb1BDbe2F0316DF893fd58CE46AA4d")?; // USDC on Arbitrum Sepolia
    
    let contract = MyERC20::new(contract_address, provider);

    let name = contract.name().call().await?;
    let symbol = contract.symbol().call().await?;
    let decimals = contract.decimals().call().await?;
    let total_supply = contract.totalSupply().call().await?;
    
    println!("\nERC-20 Token Details:");
    println!("Name: {}", name);
    println!("Symbol: {}", symbol);
    println!("Decimals: {}", decimals);
    println!("Total Supply: {}", total_supply); 
     
    Ok(())
}