import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";
import * as dotenv from "dotenv";

// Load environment variables from parent directory
dotenv.config({ path: "../variables" });

const config: HardhatUserConfig = {
  solidity: {
    version: "0.8.19",
    settings: {
      optimizer: {
        enabled: true,
        runs: 200,
      },
    },
  },
  networks: {
    "kc-chain": {
      url: process.env.CARTESI_BLOCKCHAIN_HTTP_ENDPOINT || "",
      chainId: parseInt(process.env.CARTESI_BLOCKCHAIN_ID || "1205614515668104"),
      accounts: {
        mnemonic: process.env.CARTESI_AUTH_MNEMONIC || "",
      },
      gas: 8000000,
      gasPrice: 20000000000, // 20 gwei
    },
  },
  paths: {
    sources: "./contracts",
    tests: "./test",
    cache: "./cache",
    artifacts: "./artifacts",
  },
};

export default config; 