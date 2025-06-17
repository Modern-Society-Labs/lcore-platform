const hre = require("hardhat");

async function main() {
  console.log("Deploying IoTDataProcessor contract...");

  const [deployer] = await hre.ethers.getSigners();
  console.log("Deploying contracts with the account:", deployer.address);
  
  // KC-Chain configuration
  console.log("Network information:");
  console.log("  Chain ID:", await deployer.provider.getChainId());
  console.log("  Block number:", await deployer.provider.getBlockNumber());

  const IoTDataProcessor = await hre.ethers.getContractFactory("IoTDataProcessor");
  const iotDataProcessor = await IoTDataProcessor.deploy();

  await iotDataProcessor.waitForDeployment();

  const contractAddress = await iotDataProcessor.getAddress();
  console.log("IoTDataProcessor deployed to:", contractAddress);
  
  // For verification purposes
  console.log("Deployment transaction hash:", iotDataProcessor.deploymentTransaction().hash);
  
  // Optional: Set up a mock Cartesi verifier for testing
  // Uncomment the following lines to set a mock verifier
  /*
  console.log("Setting up a mock Cartesi verifier...");
  const mockVerifierAddress = "0x0000000000000000000000000000000000000000"; // Replace with actual address
  await iotDataProcessor.setCartesiVerifier(mockVerifierAddress);
  console.log("Mock Cartesi verifier set to:", mockVerifierAddress);
  */
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  }); 