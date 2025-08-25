use anyhow::{Result, anyhow};
use rgbstd::{ContractId};
use std::str::FromStr;

// Define supported networks as a constant for better maintainability
const SUPPORTED_NETWORKS: &[&str] = &["mainnet", "testnet3", "testnet4"];

// Returns the appropriate RGB node URL for the given network
fn rgb_node_url(network: &str) -> Result<&'static str> {
    // Validate network input
    if !SUPPORTED_NETWORKS.contains(&network) {
        return Err(anyhow!("Unsupported network: {}. Supported networks: {:?}", network, SUPPORTED_NETWORKS));
    }
    
    Ok(match network {
        "mainnet" => "https://mainnet.rgbtools.org",
        "testnet3" => "https://testnet3.rgbtools.org",
        "testnet4" => "https://testnet4.rgbtools.org",
        _ => unreachable!("Network already validated"), // Safe due to prior validation
    })
}

// Shows details of a specific contract
pub fn show_contract(network: &str, contract_id: &str) -> Result<()> {
    // Validate inputs
    if network.is_empty() {
        return Err(anyhow!("Network cannot be empty"));
    }
    if contract_id.is_empty() {
        return Err(anyhow!("Contract ID cannot be empty"));
    }

    // Validate network
    rgb_node_url(network)?;

    // Parse contract ID
    let id = ContractId::from_str(contract_id)
        .map_err(|e| anyhow!("Invalid contract ID: {}", e))?;

    // Print contract details
    println!("Network: {}", network);
    println!("Contract ID: {}", id);
    println!("Schema ID: {}", id);
    println!("State: <mocked>");
    Ok(())
}

// Lists assets for the given network
pub fn list_assets(network: &str) -> Result<()> {
    // Validate network
    if network.is_empty() {
        return Err(anyhow!("Network cannot be empty"));
    }
    rgb_node_url(network)?;

    // Print asset list
    println!("Network: {}", network);
    println!("Assets:");
    println!("- RGB21 Token: TICKER, Contract ID: 010203...abcd");
    println!("- RGB25 NFT: NFT-ART, Contract ID: 0987ef...1234");
    Ok(())
}

// Synchronizes with the RGB node
pub fn sync_with_node(network: &str) -> Result<()> {
    // Validate network
    if network.is_empty() {
        return Err(anyhow!("Network cannot be empty"));
    }

    let url = format!("{}/sync", rgb_node_url(network)?);
    println!("Syncing with RGB Node at {}...", url);
    // Placeholder: real implementation would send an HTTP request to the node
    // let resp = reqwest::blocking::get(&url)?.text()?;
    // println!("Node response: {}", resp);
    println!("(Mock) Node sync completed.");
    Ok(())
}

// Retrieves balance for a specific asset
pub fn get_balance(network: &str, asset_id: &str) -> Result<()> {
    // Validate inputs
    if network.is_empty() {
        return Err(anyhow!("Network cannot be empty"));
    }
    if asset_id.is_empty() {
        return Err(anyhow!("Asset ID cannot be empty"));
    }

    let url = format!("{}/balance/{}", rgb_node_url(network)?, asset_id);
    println!("Fetching balance for asset {} on {} from {}...", asset_id, network, url);
    // Placeholder: real implementation would send an HTTP request to the node
    // let resp = reqwest::blocking::get(&url)?.text()?;
    // println!("Node response: {}", resp);
    println!("(Mock) Balance: 123.456");
    Ok(())
}