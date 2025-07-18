import { ethers } from "hardhat";
import * as fs from "fs";
import * as path from "path";

async function main() {
  console.log("🚀 Starting REAL Cartesi contract deployment to KC-Chain...");
  
  const [deployer] = await ethers.getSigners();
  console.log("📝 Deploying with account:", deployer.address);
  
  const balance = await deployer.provider.getBalance(deployer.address);
  console.log("💰 Account balance:", ethers.formatEther(balance), "ETH");
  
  // Read templateHash from .cartesi directory
  const cartesiHashPath = path.join(__dirname, "../../.cartesi/image/hash");
  let templateHash = "0x" + "00".repeat(32); // fallback
  
  if (fs.existsSync(cartesiHashPath)) {
    const hashBuffer = fs.readFileSync(cartesiHashPath);
    templateHash = "0x" + hashBuffer.toString('hex');
    console.log("📄 Using REAL templateHash:", templateHash);
  } else {
    console.log("⚠️ No .cartesi/image/hash found, using placeholder templateHash");
  }
  
  // 1. Deploy InputBox (mandatory for the whole chain)
  console.log("\n📦 Deploying InputBox...");
  const InputBox = await ethers.getContractFactory("InputBox");
  const inputBox = await InputBox.deploy();
  await inputBox.waitForDeployment();
  const inputBoxAddress = await inputBox.getAddress();
  console.log("✅ InputBox deployed to:", inputBoxAddress);
  
  // 2. Deploy Authority (settlement module)
  console.log("\n📦 Deploying Authority...");
  const Authority = await ethers.getContractFactory("Authority");
  const authority = await Authority.deploy(deployer.address);
  await authority.waitForDeployment();
  const authorityAddress = await authority.getAddress();
  console.log("✅ Authority deployed to:", authorityAddress);
  
  // 3. Deploy required libraries in dependency order
  console.log("\n📦 Deploying Bitmask library...");
  const Bitmask = await ethers.getContractFactory("contracts/Bitmask.sol:Bitmask");
  const bitmask = await Bitmask.deploy();
  await bitmask.waitForDeployment();
  const bitmaskAddress = await bitmask.getAddress();
  console.log("✅ Bitmask deployed to:", bitmaskAddress);
  
  console.log("\n📦 Deploying CartesiMathV2 library...");
  const CartesiMathV2 = await ethers.getContractFactory("contracts/CartesiMathV2.sol:CartesiMathV2");
  const cartesiMathV2 = await CartesiMathV2.deploy();
  await cartesiMathV2.waitForDeployment();
  const cartesiMathV2Address = await cartesiMathV2.getAddress();
  console.log("✅ CartesiMathV2 deployed to:", cartesiMathV2Address);
  
  console.log("\n📦 Deploying MerkleV2 library...");
  const MerkleV2 = await ethers.getContractFactory("contracts/MerkleV2.sol:MerkleV2", {
    libraries: {
      "contracts/CartesiMathV2.sol:CartesiMathV2": cartesiMathV2Address,
    },
  });
  const merkleV2 = await MerkleV2.deploy();
  await merkleV2.waitForDeployment();
  const merkleV2Address = await merkleV2.getAddress();
  console.log("✅ MerkleV2 deployed to:", merkleV2Address);
  
  // 4. Deploy CartesiDAppFactory with library linking
  console.log("\n📦 Deploying CartesiDAppFactory with libraries...");
  const CartesiDAppFactory = await ethers.getContractFactory("CartesiDAppFactory", {
    libraries: {
      "@cartesi/util/contracts/Bitmask.sol:Bitmask": bitmaskAddress,
      "@cartesi/util/contracts/MerkleV2.sol:MerkleV2": merkleV2Address,
    },
  });
  const factory = await CartesiDAppFactory.deploy();
  await factory.waitForDeployment();
  const factoryAddress = await factory.getAddress();
  console.log("✅ CartesiDAppFactory deployed to:", factoryAddress);
  
  // 4. Create CartesiDApp via factory
  console.log("\n📦 Creating CartesiDApp via factory...");
  const createTx = await factory.newApplication(
    authorityAddress,
    deployer.address, // dapp owner
    templateHash
  );
  
  const createReceipt = await createTx.wait();
  
  // Extract application address from ApplicationCreated event
  let applicationAddress = "";
  for (const log of createReceipt?.logs || []) {
    try {
      const parsed = factory.interface.parseLog(log);
      if (parsed && parsed.name === "ApplicationCreated") {
        applicationAddress = parsed.args.application;
        break;
      }
    } catch (e) {
      // Skip unparseable logs
    }
  }
  
  if (!applicationAddress) {
    throw new Error("Failed to extract application address from deployment");
  }
  
  console.log("✅ CartesiDApp deployed to:", applicationAddress);
  
  // Save deployment addresses to a file
  const deploymentInfo = {
    network: "kc-chain",
    chainId: process.env.CARTESI_BLOCKCHAIN_ID,
    deployer: deployer.address,
    timestamp: new Date().toISOString(),
    templateHash: templateHash,
    contracts: {
      InputBox: inputBoxAddress,
      Authority: authorityAddress,
      CartesiDAppFactory: factoryAddress,
      CartesiDApp: applicationAddress,
    }
  };
  
  const deploymentPath = path.join(__dirname, "../deployments/kc-chain.json");
  fs.mkdirSync(path.dirname(deploymentPath), { recursive: true });
  fs.writeFileSync(deploymentPath, JSON.stringify(deploymentInfo, null, 2));
  
  console.log("\n📄 Deployment info saved to:", deploymentPath);
  
  // Update parent variables file with REAL Cartesi contract addresses
  const variablesPath = path.join(__dirname, "../../variables");
  let variablesContent = fs.readFileSync(variablesPath, "utf8");
  
  // Update the REAL Cartesi contract addresses that Cartesi Node expects
  variablesContent = variablesContent.replace(
    /# KC_CHAIN_CONTRACT_ADDRESS=0x0000000000000000000000000000000000000000/,
    `KC_CHAIN_CONTRACT_ADDRESS=${applicationAddress}`
  );
  variablesContent = variablesContent.replace(
    /# DEVICE_ADDRESS=0x0000000000000000000000000000000000000000/,
    `DEVICE_ADDRESS=${authorityAddress}`
  );
  
  // Add the Cartesi-specific variables that Cartesi Node needs
  variablesContent += `\n# REAL Cartesi Contract Addresses (auto-deployed)`;
  variablesContent += `\nCARTESI_CONTRACTS_INPUT_BOX_ADDRESS=${inputBoxAddress}`;
  variablesContent += `\nCARTESI_CONTRACTS_APPLICATION_ADDRESS=${applicationAddress}`;
  variablesContent += `\nCARTESI_CONTRACTS_AUTHORITY_ADDRESS=${authorityAddress}`;
  variablesContent += `\nMACHINE_HASH=${templateHash}`;
  variablesContent += `\nDAPP_ADDRESS=${applicationAddress}`;
  variablesContent += `\nROLLUPS_INPUT_BOX_ADDRESS=${inputBoxAddress}`;
  
  fs.writeFileSync(variablesPath, variablesContent);
  
  console.log("\n🎉 REAL Cartesi contract deployment completed successfully!");
  console.log("📝 Updated variables file with REAL Cartesi addresses");
  console.log("\n🔗 REAL Cartesi Contract Addresses:");
  console.log("   InputBox:", inputBoxAddress);
  console.log("   CartesiDApp:", applicationAddress);
  console.log("   Authority:", authorityAddress);
  console.log("   Factory:", factoryAddress);
  console.log("   TemplateHash:", templateHash);
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error("❌ Deployment failed:", error);
    process.exit(1);
  }); 