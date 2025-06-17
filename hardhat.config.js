require("@nomicfoundation/hardhat-toolbox");
require("dotenv").config();

// Load environment variables
const PRIVATE_KEY = process.env.PRIVATE_KEY || "0x4f3edf983ac636a65a842ce7c78d9aa706d3b113bce9c46f30d7d21715b23b1d";
const KC_CHAIN_RPC = process.env.KC_CHAIN_RPC || "https://rpc.devnet.alchemy.com/7eade438-d743-4dc5-ac64-3480de391200";

/** @type import('hardhat/config').HardhatUserConfig */
module.exports = {
  solidity: "0.8.24",
  defaultNetwork: "kcchain",
  networks: {
    // KC-Chain (Arbitrum Orbit) configuration
    kcchain: {
      url: KC_CHAIN_RPC,
      accounts: [PRIVATE_KEY],
      chainId: 1205614515668104,
      gasPrice: 100000000, // 100 Gwei
      timeout: 60000 // 60 seconds
    },
    hardhat: {
      // Local testing network
    },
    localhost: {
      url: "http://127.0.0.1:8545",
      chainId: 31337,
    },
    // Example of a testnet configuration
    sepolia: {
      url: `https://sepolia.infura.io/v3/${process.env.INFURA_API_KEY || ""}`,
      accounts: process.env.PRIVATE_KEY ? [PRIVATE_KEY] : []
    }
  },
  paths: {
    sources: "./contracts",
    tests: "./test",
    cache: "./cache",
    artifacts: "./artifacts"
  },
  // Gas reporter configuration
  gasReporter: {
    enabled: process.env.REPORT_GAS !== undefined,
    currency: "USD"
  },
  // Etherscan API key for contract verification
  etherscan: {
    apiKey: process.env.ETHERSCAN_API_KEY
  }
}; 