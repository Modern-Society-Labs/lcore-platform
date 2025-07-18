import { ethers } from "hardhat";
import * as fs from "fs";
import * as path from "path";

async function main() {
  console.log("🔄 Updating CartesiDApp with REAL templateHash...");
  
  const [deployer] = await ethers.getSigners();
  console.log("📝 Deploying with account:", deployer.address);
  
  // Read the real templateHash
  const cartesiHashPath = path.join(__dirname, "../../.cartesi/image/hash");
  
  if (!fs.existsSync(cartesiHashPath)) {
    throw new Error("❌ Cannot find .cartesi/image/hash file!");
  }
  
  const hashBuffer = fs.readFileSync(cartesiHashPath);
  const templateHash = "0x" + hashBuffer.toString('hex');
  console.log("📄 Using REAL templateHash:", templateHash);
  
  // Read the existing deployment to get the factory and authority addresses
  const deploymentPath = path.join(__dirname, "../deployments/kc-chain.json");
  if (!fs.existsSync(deploymentPath)) {
    throw new Error("❌ No existing deployment found! Run ./deploy.sh first");
  }
  
  const deployment = JSON.parse(fs.readFileSync(deploymentPath, "utf8"));
  const factoryAddress = deployment.contracts.CartesiDAppFactory;
  const authorityAddress = deployment.contracts.Authority;
  
  console.log("🏭 Using existing factory:", factoryAddress);
  console.log("🏛️ Using existing authority:", authorityAddress);
  
  // Create new CartesiDApp with real templateHash
  console.log("\n📦 Creating NEW CartesiDApp with REAL templateHash...");
  const factory = await ethers.getContractAt("CartesiDAppFactory", factoryAddress);
  
  const createTx = await factory.newApplication(
    authorityAddress,
    deployer.address, // dapp owner
    templateHash
  );
  
  const createReceipt = await createTx.wait();
  
  // Extract application address from ApplicationCreated event
  let newApplicationAddress = "";
  for (const log of createReceipt?.logs || []) {
    try {
      const parsed = factory.interface.parseLog(log);
      if (parsed && parsed.name === "ApplicationCreated") {
        newApplicationAddress = parsed.args.application;
        break;
      }
    } catch (e) {
      // Skip unparseable logs
    }
  }
  
  if (!newApplicationAddress) {
    throw new Error("Failed to extract new application address from deployment");
  }
  
  console.log("✅ NEW CartesiDApp deployed to:", newApplicationAddress);
  
  // Update deployment info
  deployment.contracts.CartesiDApp = newApplicationAddress;
  deployment.templateHash = templateHash;
  deployment.timestamp = new Date().toISOString();
  
  fs.writeFileSync(deploymentPath, JSON.stringify(deployment, null, 2));
  
  // Update variables file with new application address and real templateHash
  const variablesPath = path.join(__dirname, "../../variables");
  let variablesContent = fs.readFileSync(variablesPath, "utf8");
  
  // Update with new application address
  variablesContent = variablesContent.replace(
    /KC_CHAIN_CONTRACT_ADDRESS=0x[a-fA-F0-9]+/,
    `KC_CHAIN_CONTRACT_ADDRESS=${newApplicationAddress}`
  );
  
  // Update the MACHINE_HASH with real hash
  variablesContent = variablesContent.replace(
    /MACHINE_HASH=0x[0-9a-fA-F]+/,
    `MACHINE_HASH=${templateHash}`
  );
  variablesContent = variablesContent.replace(
    /DAPP_ADDRESS=0x[a-fA-F0-9]+/,
    `DAPP_ADDRESS=${newApplicationAddress}`
  );
  
  fs.writeFileSync(variablesPath, variablesContent);
  
  console.log("\n🎉 CartesiDApp updated with REAL templateHash!");
  console.log("📝 Updated variables file with new application address");
  console.log("\n🔗 Updated Contract Addresses:");
  console.log("   NEW CartesiDApp:", newApplicationAddress);
  console.log("   TemplateHash:", templateHash);
  console.log("\n🚀 Ready to test with lcore-node using REAL Cartesi infrastructure!");
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error("❌ Update failed:", error);
    process.exit(1);
  }); 