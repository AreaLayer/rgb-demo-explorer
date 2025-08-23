#[derive(Parser)]
#[command(name = "rgb-demo-explorer")]
#[command(about = "A minimal RGB explorer CLI demo", long_about = None)]
struct Cli {
    /// Bitcoin network to use
    #[arg(long, value_enum, default_value_t = Network::Testnet3)]
    network: Network,

    /// Optional: Use a custom RGB Node URL instead of the default for the network
    #[arg(long)]
    node_url: Option<String>,

    #[command(subcommand)]
    command: Commands,
}
