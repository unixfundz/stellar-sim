# stellar-sim

A local Stellar/Soroban transaction simulator with a CLI interface. Simulate smart contract transactions, inspect diagnostic events, and test WASM contracts without touching the network.

## Installation

```bash
cargo install stellar-sim
```

Or build from source:

```bash
git clone https://github.com/unixfundz/stellar-sim
cd stellar-sim
cargo build --release
```

## Usage

```bash
stellar-sim simulate --wasm ./contract.wasm --function transfer --args '["GABC", "GXYZ", "100"]'
stellar-sim events --tx <base64-xdr>
stellar-sim inspect --wasm ./contract.wasm
stellar-sim snapshot --rpc https://soroban-testnet.stellar.org --contract GABC... --output snapshot.json
```

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md). Check open issues for tasks labelled `good first issue`.

## License

Apache 2.0
