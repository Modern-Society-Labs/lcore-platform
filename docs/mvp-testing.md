# MVP Testing Strategy - Guide to Reproducing the Test

## 🧪 Testing Overview

The MVP's testing strategy has successfully validated the core dual-encryption pipeline. The end-to-end flow—from a simulated device to an on-chain commitment—is confirmed to be working. This document provides the exact steps and commands needed to recreate this successful test.

## ⚙️ Test Environment Setup

1.  **Clone the Monorepo**: Ensure you have the entire `IoT-L{CORE}` project.

2.  **Configure the Node**:
    -   Navigate to the `lcore-node-mvp` directory.
    -   Create a `config.toml` file from the template (`cp config.template.toml config.toml`).
    -   Fill in the `config.toml` with:
        -   `rpc_url`: Your KC-Chain RPC URL.
        -   `private_key`: The private key for a wallet funded with ETH on the KC-Chain devnet.
        -   `mvp_processor`: The deployed contract address: `0xd99061c28b9063d9651fea67930fc4ff598ba5b2`.

3.  **Run the Node**:
    -   From the `lcore-node-mvp` directory, run the server:
        ```bash
        source "$HOME/.cargo/env" # Or ensure cargo is in your PATH
        cargo run --release
        ```
    -   Wait for the log line confirming the server is running: `Listening on 127.0.0.1:3000`.

## 🔬 End-to-End Test Scenario

These are the exact steps that were executed to confirm the MVP is working.

### **Test Step 1: Register a Device**
-   **Purpose**: To add a device to the local SQLite database.
-   **Command**:
    ```bash
    curl -X POST http://127.0.0.1:3000/device/register \
         -H 'Content-Type: application/json' \
         -d '{"device_id":"device_001"}'
    ```
-   **Expected Result**: A JSON response confirming success.
    ```json
    {"success":true,"message":"Device registered"}
    ```

### **Test Step 2: Submit Data and Commit On-Chain**
-   **Purpose**: To test the full pipeline: data submission, encryption, local storage, and the on-chain transaction.
-   **Command**:
    ```bash
    curl -X POST http://127.0.0.1:3000/device/data \
         -H 'Content-Type: application/json' \
         -d '{"device_id":"device_001","data":"{\"temperature\":25.5,\"humidity\":60.1}","timestamp":1718576400}'
    ```
-   **Expected Result**: A JSON response confirming submission and providing the transaction hash.
    ```json
    {"success":true,"message":"Data submitted; tx 0x72921cb216b66826a130bc06ff340c1deadc02da38c23167f7777043fd906e8a"}
    ```
    *(Note: Your transaction hash will be different.)*

### **Test Step 3: Verify the On-Chain Result**
-   **Purpose**: To independently verify that the data commitment was written to the Stylus smart contract.
-   **Prerequisites**: [Foundry](https://getfoundry.sh/) must be installed.
-   **Command**:
    -   The `task_id` for the data submitted in Step 2 is `0x03c8deb30d8d0d830ba20cb607786f2b31310eff52408aca9a7a5adb08f791cd`.
    -   Use `cast call` to query the contract's `get_result` function.
    ```bash
    # Replace <YOUR_KC_CHAIN_RPC_URL> with the RPC URL from your config.toml
    cast call 0xd99061c28b9063d9651fea67930fc4ff598ba5b2 \
      "get_result(bytes32)" \
      0x03c8deb30d8d0d830ba20cb607786f2b31310eff52408aca9a7a5adb08f791cd \
      --rpc-url <YOUR_KC_CHAIN_RPC_URL>
    ```
-   **Expected Result**: The command will print the tuple of data stored on-chain, including the encrypted result, the proof hash, the timestamp, and the address of the submitter.

## ✅ MVP Success Criteria Validation

The successful execution of these steps validates the primary goals of the MVP:

-   ✅ **Device Registration**: The system can manage a simple device registry.
-   ✅ **Dual Encryption**: The data is processed through the encryption pipeline.
-   ✅ **Local Persistence**: Encrypted data is correctly stored in the SQLite database.
-   ✅ **On-Chain Commitment**: The system can successfully build, sign, and send a transaction to an Arbitrum Orbit chain and have it mined.
-   ✅ **Data Retrieval**: The on-chain data can be successfully queried and retrieved.

This validated test provides a solid baseline for future development and the planned migration to a full Cartesi architecture.