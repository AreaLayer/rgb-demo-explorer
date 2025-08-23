mod explorer;

use clap::{Parser, Subcommand, ValueEnum};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "rgb-demo-explorer")]
#[command(about = "A minimal RGB explorer CLI demo", long_about = None)]
struct Cli {
    /// Bitcoin network to use
    #[arg(long, value_enum, default_value_t = Network::Testnet3)]
    network: Network,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Network {
    Mainnet,
    Testnet3,
    Testnet4,
}

#[derive(Subcommand)]
enum Commands {
    /// Show contract info for a given contract ID
    ShowContract {
        #[arg(help = "RGB contract ID (as hex)")]
        contract_id: String,
    },
    /// List assets (demo: from mocked data)
    ListAssets,
    /// Sync with RGB Node
    Sync,
    /// Get balance for a given asset
    GetBalance {
        #[arg(help = "Asset contract ID (as hex)")]
        asset_id: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let net = match cli.network {
        Network::Mainnet => "mainnet",
        Network::Testnet3 => "testnet3",
        Network::Testnet4 => "testnet4",
    };
    match cli.command {
        Commands::ShowContract { contract_id } => explorer::show_contract(net, &contract_id)?,
        Commands::ListAssets => explorer::list_assets(net)?,
        Commands::Sync => explorer::sync_with_node(net)?,
        Commands::GetBalance { asset_id } => explorer::get_balance(net, &asset_id)?,
    }
    Ok(())
}