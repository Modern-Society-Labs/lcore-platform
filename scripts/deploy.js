const hre = require("hardhat");

async function main() {
  console.log("Deploying DeviceRegistry contract...");

  const [deployer] = await hre.ethers.getSigners();
  console.log("Deploying contracts with the account:", deployer.address);

  const DeviceRegistry = await hre.ethers.getContractFactory("DeviceRegistry");
  const deviceRegistry = await DeviceRegistry.deploy();

  await deviceRegistry.waitForDeployment();

  const contractAddress = await deviceRegistry.getAddress();
  console.log("DeviceRegistry deployed to:", contractAddress);
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  }); 