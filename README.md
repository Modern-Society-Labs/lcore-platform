# lcore-platform

This repository contains the smart contracts and dApp integration logic for the IoT-L{CORE} SDK ecosystem. It uses Hardhat as its development environment.

## Features

-   **Solidity Smart Contracts**: The on-chain logic for device registration, data verification, and other platform features.
-   **Hardhat Environment**: A complete environment for compiling, testing, and deploying smart contracts.
-   **Deployment Scripts**: Reusable scripts to deploy contracts to various Ethereum networks.
-   **Comprehensive Tests**: Unit tests for all contract functionality using `ethers.js` and `chai`.

## Project Structure

```
lcore-platform/
├── contracts/          # Solidity source files (e.g., DeviceRegistry.sol)
├── scripts/            # Deployment scripts
├── test/               # Test files
├── hardhat.config.js   # Hardhat configuration
├── package.json        # Project dependencies
└── README.md
```

## Getting Started

### Prerequisites

-   Node.js (version 18 or higher)
-   NPM or Yarn

### Setup

1.  **Clone the repository.**
2.  **Install dependencies:**
    ```bash
    npm install
    ```

### Compiling the Contracts

```bash
npx hardhat compile
```

This will generate ABI and bytecode artifacts in the `artifacts/` directory.

### Running Tests

```bash
npx hardhat test
```

### Deploying the Contracts

To deploy the contracts to the local Hardhat network (for testing):

```bash
npx hardhat run scripts/deploy.js --network localhost
```

To deploy to a live testnet (e.g., Sepolia), you will need to:
1.  Add a network configuration in `hardhat.config.js`.
2.  Set environment variables for your RPC URL and private key.
3.  Run the deployment script with the appropriate network flag.

## License

MIT
