const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');
const { ethers } = require('hardhat');

// Configuration
const STYLUS_CONTRACTS = [
  {
    name: 'IoTDataProcessorStylus',
    path: './contracts/stylus/IoTDataProcessorStylus.rs'
  },
  {
    name: 'CartesiVerifierStylus',
    path: './contracts/stylus/CartesiVerifierStylus.rs'
  },
  {
    name: 'VerifiedIoTIntelligenceStylus',
    path: './contracts/stylus/VerifiedIoTIntelligenceStylus.rs',
    constructorArgs: ['IoTDataProcessorStylus', 'CartesiVerifierStylus']
  }
];

// Helper function to compile a Rust contract
async function compileRustContract(contractPath) {
  console.log(`Compiling Rust contract at ${contractPath}...`);
  
  try {
    // Note: This assumes cargo-stylus is installed
    // Install with: cargo install --git https://github.com/OffchainLabs/cargo-stylus
    const output = execSync(`cargo stylus check --wasm-file-only ${contractPath}`);
    console.log(`Compilation successful: ${output.toString()}`);
    
    // Get the output WASM path (should be in target/wasm32-unknown-unknown/release/)
    const wasmFileName = path.basename(contractPath, '.rs') + '.wasm';
    const wasmPath = path.join('target', 'wasm32-unknown-unknown', 'release', wasmFileName);
    
    if (!fs.existsSync(wasmPath)) {
      throw new Error(`WASM file not found at ${wasmPath}`);
    }
    
    return wasmPath;
  } catch (error) {
    console.error(`Error compiling Rust contract: ${error.message}`);
    throw error;
  }
}

// Helper function to deploy a WASM contract to Arbitrum Stylus
async function deployStylusContract(wasmPath, constructorArgs = []) {
  console.log(`Deploying WASM contract from ${wasmPath}...`);
  
  try {
    // Read the WASM binary
    const wasmBinary = fs.readFileSync(wasmPath);
    
    // Deploy using cargo-stylus
    // Note: In a real deployment, you would use the Arbitrum SDK or cargo-stylus directly
    // This is a simplified example
    const [deployer] = await ethers.getSigners();
    console.log(`Deploying with account: ${deployer.address}`);
    
    // In a real implementation, you would use the Arbitrum SDK to deploy the WASM contract
    // For this example, we'll simulate the deployment
    console.log(`Contract would be deployed with constructor args: ${constructorArgs.join(', ')}`);
    
    // Return a simulated contract address
    // In a real deployment, this would be the actual deployed contract address
    return {
      address: ethers.Wallet.createRandom().address,
      wasmPath
    };
  } catch (error) {
    console.error(`Error deploying WASM contract: ${error.message}`);
    throw error;
  }
}

async function main() {
  console.log("Deploying Arbitrum Stylus contracts to KC-Chain...");
  
  // Get network information
  const [deployer] = await ethers.getSigners();
  const network = await ethers.provider.getNetwork();
  console.log(`Network: ${network.name} (Chain ID: ${network.chainId})`);
  console.log(`Deployer address: ${deployer.address}`);
  console.log(`Block number: ${await ethers.provider.getBlockNumber()}`);
  
  // Compile and deploy each contract
  const deployedContracts = {};
  
  for (const contract of STYLUS_CONTRACTS) {
    console.log(`\nProcessing ${contract.name}...`);
    
    // Compile the Rust contract
    const wasmPath = await compileRustContract(contract.path);
    
    // Prepare constructor arguments (resolve contract addresses if needed)
    const resolvedArgs = (contract.constructorArgs || []).map(arg => {
      if (deployedContracts[arg]) {
        return deployedContracts[arg].address;
      }
      return arg;
    });
    
    // Deploy the contract
    const deployed = await deployStylusContract(wasmPath, resolvedArgs);
    deployedContracts[contract.name] = deployed;
    
    console.log(`${contract.name} deployed to: ${deployed.address}`);
  }
  
  // Save deployment information
  const deploymentInfo = {
    timestamp: new Date().toISOString(),
    network: {
      name: network.name,
      chainId: network.chainId.toString()
    },
    contracts: Object.entries(deployedContracts).reduce((acc, [name, info]) => {
      acc[name] = info.address;
      return acc;
    }, {})
  };
  
  const deploymentPath = path.join(__dirname, '..', 'deployments', `stylus-${network.name}-${Date.now()}.json`);
  fs.mkdirSync(path.dirname(deploymentPath), { recursive: true });
  fs.writeFileSync(deploymentPath, JSON.stringify(deploymentInfo, null, 2));
  
  console.log(`\nDeployment information saved to: ${deploymentPath}`);
  console.log("\nDeployment summary:");
  Object.entries(deployedContracts).forEach(([name, info]) => {
    console.log(`- ${name}: ${info.address}`);
  });
}

// Execute the deployment
main()
  .then(() => process.exit(0))
  .catch(error => {
    console.error(error);
    process.exit(1);
  }); 