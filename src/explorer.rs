use anyhow::{Result, anyhow};
use rgbstd::{ContractId, stl::NIA_SCHEMA_ID};
use std::str::FromStr;

fn rgb_node_url(network: &str) -> &'static str {
    match network {
        "mainnet" => "https://mainnet.rgbtools.org",        // Public RGB Node by rgbtools.org
        "testnet3" => "https://testnet3.rgbtools.org",      // Public Testnet3 endpoint
        "testnet4" => "https://testnet4.rgbtools.org",      // Public Testnet4 endpoint
        _ => "https://testnet3.rgbtools.org",
    }
}

pub fn show_contract(network: &str, contract_id: &str) -> Result<()> {
    let id = ContractId::from_str(contract_id)
        .map_err(|_| anyhow!("Invalid contract id"))?;
    println!("Network: {network}");
    println!("Contract ID: {id}");
    println!("Schema ID: {}", NIA_SCHEMA_ID); // Demo: show a known schema
    println!("State: <mocked>");
    Ok(())
}

pub fn list_assets(network: &str) -> Result<()> {
    println!("Network: {network}");
    println!("Assets:");
    println!("- RGB21 Token: TICKER, Contract ID: 010203...abcd");
    println!("- RGB25 NFT: NFT-ART, Contract ID: 0987ef...1234");
    Ok(())
}

pub fn sync_with_node(network: &str) -> Result<()> {
    let url = format!("{}/sync", rgb_node_url(network));
    println!("Syncing with RGB Node at {url}...");
    // Placeholder: real implementation would send an HTTP request to the node
    // let resp = reqwest::blocking::get(&url)?.text()?;
    // println!("Node response: {resp}");
    println!("(Mock) Node sync completed.");
    Ok(())
}

pub fn get_balance(network: &str, asset_id: &str) -> Result<()> {
    let url = format!("{}/balance/{}", rgb_node_url(network), asset_id);
    println!("Fetching balance for asset {asset_id} on {network} from {url}...");
    // Placeholder: real implementation would send an HTTP request to the node
    // let resp = reqwest::blocking::get(&url)?.text()?;
    // println!("Node response: {resp}");
    println!("(Mock) Balance: 123.456");
    Ok(())
}
