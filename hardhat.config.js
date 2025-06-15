require("@nomicfoundation/hardhat-toolbox");

/** @type import('hardhat/config').HardhatUserConfig */
module.exports = {
  solidity: "0.8.24",
  networks: {
    hardhat: {
      // Default network for testing
    },
    // Example of a testnet configuration
    // sepolia: {
    //   url: `https://sepolia.infura.io/v3/${process.env.INFURA_API_KEY}`,
    //   accounts: [`0x${process.env.PRIVATE_KEY}`]
    // }
  },
  paths: {
    sources: "./contracts",
    tests: "./test",
    cache: "./cache",
    artifacts: "./artifacts"
  }
}; 