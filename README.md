# RGB Demo Explorer

A minimal Rust command-line explorer for the [RGB protocol](https://rgb.tech/), supporting Bitcoin `mainnet`, `testnet3`, and `testnet4` networks.  
It features public RGB Node integration (e.g., [rgbtools.org](https://rgbtools.org/)), and is ready for extension as a wallet, asset explorer, or DEX interface.

---

## Features

- **Multi-network support:** Select mainnet, testnet3, or testnet4
- **Command-line interface:** Simple CLI using `clap`
- **Public RGB Node integration:** Connects to [rgbtools.org](https://rgbtools.org/) endpoints by default
- **Custom RGB Node:** Override with `--node-url`
- **Sample explorer commands:** List assets, show contract, sync with node, get balance (mock/demo, easily extendable)

---

## Getting Started

### 1. Install Rust

If you don't have Rust installed, get it via [rustup](https://rustup.rs/):

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Clone and Build

```sh
git clone https://github.com/yourusername/rgb-demo-explorer.git
cd rgb-demo-explorer
cargo build
```

### 3. Usage

```sh
cargo run -- --network mainnet list-assets
cargo run -- --network testnet3 show-contract 0102030405060708090a0b0c0d0e0f10
cargo run -- --network testnet4 get-balance 01abcdef23456789fedcba0987654321
cargo run -- --network mainnet --node-url https://yournode.example.com sync
```

#### CLI Options

- `--network [mainnet|testnet3|testnet4]` – Select Bitcoin network (default: testnet3)
- `--node-url <URL>` – Optional: Use a custom RGB Node endpoint

#### Commands

- `list-assets` – List demo assets (extend for real explorer)
- `show-contract <CONTRACT_ID>` – Show info for a contract
- `get-balance <ASSET_ID>` – Show demo/mock asset balance
- `sync` – Mock sync with RGB Node

---

## Customization & Extension

- **Real RGB node integration:**  
  Replace the mock HTTP calls in `explorer.rs` with real API requests and response parsing using `reqwest` and `serde_json`.
- **Add more commands:**  
  Extend the CLI by adding more variants to the `Commands` enum and implementing new functions in `explorer.rs`.
- **UI/UX improvements:**  
  Add better error handling, pretty-print, and asset/contract parsing as needed.

---

## Third-Party Nodes

This explorer uses public RGB Node endpoints by default:

- Mainnet:   `https://mainnet.rgbtools.org`
- Testnet3:  `https://testnet3.rgbtools.org`
- Testnet4:  `https://testnet4.rgbtools.org`

You may override the node URL with `--node-url`.

---

## License

MIT
