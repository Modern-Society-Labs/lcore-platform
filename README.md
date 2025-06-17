# IoT-L{CORE} Platform

This repository contains the smart contracts and deployment scripts for the IoT-L{CORE} platform, which integrates with our KC-Chain Arbitrum Orbit devnet.

## MVP Contracts

### `MVPIoTProcessor`

The primary contract for the MVP is `MVPIoTProcessor`, a simple yet effective on-chain commitment ledger. It is written in Rust and compiled to WASM for use with Arbitrum Stylus.

**Location**: `stylus_contracts/src/lib.rs`

### Features
- **`submit_result(bytes32 task_id, bytes encrypted_result, bytes32 proof_hash)`**: Allows the `lcore-node` to commit a record of a data processing task. It stores the encrypted result, a proof hash, the block timestamp, and the sender's address.
- **`get_result(bytes32 task_id)`**: A public getter to retrieve the stored results for a given task ID.

This contract serves as the immutable "bulletin board" for data processed by the off-chain node, providing a trust-minimized anchor for the system.

### `DeviceRegistryStylus` (NEW)

`DeviceRegistryStylus` is a lightweight Stylus contract that maintains a mapping of `deviceId → bool` indicating whether a device is authorised.  It replaces the temporary SQLite `devices` table used during early MVP testing and allows anyone to query on-chain whether a device is registered.  Only the contract owner can add or revoke devices, and multiple devices can be added over time without redeploying the contract.

Location: `contracts/stylus/DeviceRegistryStylus.rs`

Key functions:
* `register_device(bytes32 deviceId)` – owner-only
* `revoke_device(bytes32 deviceId)` – owner-only
* `is_device_registered(bytes32 deviceId) → bool` – public view

## Future Contracts (Planned)

The following contracts are part of the broader architectural vision but are **not implemented in the current MVP**:

- **`IoTDataProcessorStylus`**: A future contract intended to handle more complex logic, such as BLS signature aggregation and rolling analytics.
- **`VerifiedIoTIntelligenceStylus`**: A future contract designed to run TinyML models for on-chain inference.

## Development & Deployment

### Prerequisites

- Rust and `cargo-stylus`
- A funded wallet for deployment.

### Installation

```bash
# Install cargo-stylus
cargo install --force cargo-stylus
```

### Build

From the `lcore-platform/stylus_contracts` directory:
```bash
# Build the WASM for the contract
cargo stylus wasm
```
This will produce `target/wasm32-unknown-unknown/release/mvp_iot_processor.wasm`.

### Deployment

Deploy the contract to the KC-Chain devnet.

```bash
# Set your private key as an environment variable
export PRIVATE_KEY=your_private_key

# Deploy the contract
cargo stylus deploy -e https://rpc.devnet.alchemy.com/7eade438-d743-4dc5-ac64-3480de391200 --private-key $PRIVATE_KEY
```
Upon successful deployment, the CLI will output the new contract address.

## KC-Chain Information

- **Chain ID**: `1205614515668104`
- **Public RPC Endpoint**: `https://rpc.devnet.alchemy.com/7eade438-d743-4dc5-ac64-3480de391200`
- **Framework**: Arbitrum Orbit
- **Settlement Layer**: Ethereum Sepolia
- **Gas Token**: ETH
- **MVP Contract Address**: `0xd99061c28b9063d9651fea67930fc4ff598ba5b2`

## Testing

```bash
# Run tests
npx hardhat test
```

## License

MIT
