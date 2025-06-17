const hre = require("hardhat");

async function main() {
  console.log("Deploying full Cartesi integration contracts...");

  const [deployer] = await hre.ethers.getSigners();
  console.log("Deploying contracts with the account:", deployer.address);
  
  // KC-Chain configuration
  console.log("Network information:");
  console.log("  Chain ID:", await deployer.provider.getChainId());
  console.log("  Block number:", await deployer.provider.getBlockNumber());

  // 1. Deploy CartesiVerifier
  console.log("\nDeploying CartesiVerifier contract...");
  const CartesiVerifier = await hre.ethers.getContractFactory("CartesiVerifier");
  const cartesiVerifier = await CartesiVerifier.deploy();
  await cartesiVerifier.waitForDeployment();
  const verifierAddress = await cartesiVerifier.getAddress();
  console.log("CartesiVerifier deployed to:", verifierAddress);
  
  // 2. Set Cartesi rollup contract
  const rollupContractAddress = "0x7d0E04186626AFcA86C1eD4d21d4842b3E32680F"; // From techContext.md
  console.log("Setting Cartesi rollup contract address:", rollupContractAddress);
  const setRollupTx = await cartesiVerifier.setRollupContract(rollupContractAddress);
  await setRollupTx.wait();
  console.log("Rollup contract set successfully");
  
  // 3. Deploy IoTDataProcessor
  console.log("\nDeploying IoTDataProcessor contract...");
  const IoTDataProcessor = await hre.ethers.getContractFactory("IoTDataProcessor");
  const iotDataProcessor = await IoTDataProcessor.deploy();
  await iotDataProcessor.waitForDeployment();
  const processorAddress = await iotDataProcessor.getAddress();
  console.log("IoTDataProcessor deployed to:", processorAddress);
  
  // 4. Link IoTDataProcessor to CartesiVerifier
  console.log("\nLinking IoTDataProcessor to CartesiVerifier...");
  const setVerifierTx = await iotDataProcessor.setCartesiVerifier(verifierAddress);
  await setVerifierTx.wait();
  console.log("CartesiVerifier linked successfully");
  
  // 5. Summary
  console.log("\nDeployment Summary:");
  console.log("-------------------");
  console.log("CartesiVerifier:   ", verifierAddress);
  console.log("IoTDataProcessor:  ", processorAddress);
  console.log("Cartesi Rollup:    ", rollupContractAddress);
  console.log("\nSetup complete!");
  
  // Save deployment info to a file for future reference
  const fs = require('fs');
  const deploymentInfo = {
    network: hre.network.name,
    chainId: await deployer.provider.getChainId(),
    cartesiVerifier: verifierAddress,
    iotDataProcessor: processorAddress,
    cartesiRollup: rollupContractAddress,
    deployer: deployer.address,
    timestamp: new Date().toISOString()
  };
  
  fs.writeFileSync(
    `deployment-${hre.network.name}-${deploymentInfo.timestamp}.json`,
    JSON.stringify(deploymentInfo, null, 2)
  );
  console.log("Deployment information saved to file");
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  }); 