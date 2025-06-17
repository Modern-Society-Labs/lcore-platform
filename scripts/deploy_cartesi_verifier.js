const hre = require("hardhat");

async function main() {
  console.log("Deploying CartesiVerifier contract...");

  const [deployer] = await hre.ethers.getSigners();
  console.log("Deploying contracts with the account:", deployer.address);
  
  // KC-Chain configuration
  console.log("Network information:");
  console.log("  Chain ID:", await deployer.provider.getChainId());
  console.log("  Block number:", await deployer.provider.getBlockNumber());

  const CartesiVerifier = await hre.ethers.getContractFactory("CartesiVerifier");
  const cartesiVerifier = await CartesiVerifier.deploy();

  await cartesiVerifier.waitForDeployment();

  const contractAddress = await cartesiVerifier.getAddress();
  console.log("CartesiVerifier deployed to:", contractAddress);
  
  // For verification purposes
  console.log("Deployment transaction hash:", cartesiVerifier.deploymentTransaction().hash);
  
  // Set the Cartesi rollup contract address for KC-Chain
  // This is the address of the deployed Cartesi rollup contract on KC-Chain
  const rollupContractAddress = "0x7d0E04186626AFcA86C1eD4d21d4842b3E32680F"; // From techContext.md
  
  console.log("Setting Cartesi rollup contract address:", rollupContractAddress);
  const setRollupTx = await cartesiVerifier.setRollupContract(rollupContractAddress);
  await setRollupTx.wait();
  
  console.log("Rollup contract set successfully");
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  }); 