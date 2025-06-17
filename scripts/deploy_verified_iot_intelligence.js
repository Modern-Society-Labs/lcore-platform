const hre = require("hardhat");
const fs = require('fs');

async function main() {
  console.log("Deploying VerifiedIoTIntelligence contract...");

  const [deployer] = await hre.ethers.getSigners();
  console.log("Deploying contracts with the account:", deployer.address);
  
  // KC-Chain configuration
  console.log("Network information:");
  console.log("  Chain ID:", await deployer.provider.getChainId());
  console.log("  Block number:", await deployer.provider.getBlockNumber());

  // Check if we have existing deployment information
  let iotDataProcessorAddress;
  let cartesiVerifierAddress;
  
  try {
    // Try to find the latest deployment file
    const files = fs.readdirSync('./').filter(file => file.startsWith('deployment-') && file.endsWith('.json'));
    if (files.length > 0) {
      // Sort by timestamp (newest first)
      files.sort().reverse();
      const deploymentInfo = JSON.parse(fs.readFileSync(files[0]));
      
      iotDataProcessorAddress = deploymentInfo.iotDataProcessor;
      cartesiVerifierAddress = deploymentInfo.cartesiVerifier;
      
      console.log("Found existing deployment:");
      console.log("  IoTDataProcessor:", iotDataProcessorAddress);
      console.log("  CartesiVerifier:", cartesiVerifierAddress);
    }
  } catch (error) {
    console.log("No existing deployment found, will deploy new contracts");
  }
  
  // If we don't have existing contracts, deploy them
  if (!iotDataProcessorAddress || !cartesiVerifierAddress) {
    // Deploy CartesiVerifier
    console.log("\nDeploying CartesiVerifier contract...");
    const CartesiVerifier = await hre.ethers.getContractFactory("CartesiVerifier");
    const cartesiVerifier = await CartesiVerifier.deploy();
    await cartesiVerifier.waitForDeployment();
    cartesiVerifierAddress = await cartesiVerifier.getAddress();
    console.log("CartesiVerifier deployed to:", cartesiVerifierAddress);
    
    // Set Cartesi rollup contract
    const rollupContractAddress = "0x7d0E04186626AFcA86C1eD4d21d4842b3E32680F"; // From techContext.md
    console.log("Setting Cartesi rollup contract address:", rollupContractAddress);
    const setRollupTx = await cartesiVerifier.setRollupContract(rollupContractAddress);
    await setRollupTx.wait();
    console.log("Rollup contract set successfully");
    
    // Deploy IoTDataProcessor
    console.log("\nDeploying IoTDataProcessor contract...");
    const IoTDataProcessor = await hre.ethers.getContractFactory("IoTDataProcessor");
    const iotDataProcessor = await IoTDataProcessor.deploy();
    await iotDataProcessor.waitForDeployment();
    iotDataProcessorAddress = await iotDataProcessor.getAddress();
    console.log("IoTDataProcessor deployed to:", iotDataProcessorAddress);
    
    // Link IoTDataProcessor to CartesiVerifier
    console.log("\nLinking IoTDataProcessor to CartesiVerifier...");
    const setVerifierTx = await iotDataProcessor.setCartesiVerifier(cartesiVerifierAddress);
    await setVerifierTx.wait();
    console.log("CartesiVerifier linked successfully");
  }

  // Deploy VerifiedIoTIntelligence
  console.log("\nDeploying VerifiedIoTIntelligence contract...");
  const VerifiedIoTIntelligence = await hre.ethers.getContractFactory("VerifiedIoTIntelligence");
  const verifiedIoTIntelligence = await VerifiedIoTIntelligence.deploy(
    iotDataProcessorAddress,
    cartesiVerifierAddress
  );
  await verifiedIoTIntelligence.waitForDeployment();
  const intelligenceAddress = await verifiedIoTIntelligence.getAddress();
  console.log("VerifiedIoTIntelligence deployed to:", intelligenceAddress);
  
  // Summary
  console.log("\nDeployment Summary:");
  console.log("-------------------");
  console.log("CartesiVerifier:         ", cartesiVerifierAddress);
  console.log("IoTDataProcessor:        ", iotDataProcessorAddress);
  console.log("VerifiedIoTIntelligence: ", intelligenceAddress);
  console.log("\nSetup complete!");
  
  // Save deployment info to a file for future reference
  const deploymentInfo = {
    network: hre.network.name,
    chainId: await deployer.provider.getChainId(),
    cartesiVerifier: cartesiVerifierAddress,
    iotDataProcessor: iotDataProcessorAddress,
    verifiedIoTIntelligence: intelligenceAddress,
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