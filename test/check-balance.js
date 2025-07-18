const { ethers } = require("hardhat");

async function checkBalance() {
    console.log("💰 Checking Account Balance & Details");
    console.log("=====================================");
    
    // Get current signer
    const [signer] = await ethers.getSigners();
    const address = signer.address;
    
    // Get balance
    const balance = await ethers.provider.getBalance(address);
    const balanceEth = ethers.formatEther(balance);
    
    // Get network info
    const network = await ethers.provider.getNetwork();
    const blockNumber = await ethers.provider.getBlockNumber();
    
    console.log("Current Account:", address);
    console.log("Network:", network.name, "Chain ID:", network.chainId.toString());
    console.log("Current Block:", blockNumber);
    console.log("Balance:", balanceEth, "ETH");
    
    // Derive address from the private key to confirm
    const providedKey = "0x5c1e13f56de66ba39fa5e41a40a8610081cf6df7c9903ef35b90a7e85dfca39d";
    const wallet = new ethers.Wallet(providedKey);
    const derivedAddress = wallet.address;
    
    console.log("Derived from provided key:", derivedAddress);
    
    if (address.toLowerCase() === derivedAddress.toLowerCase()) {
        console.log("✅ CONFIRMED: Using the provided funded account");
        
        if (parseFloat(balanceEth) > 0) {
            console.log("🎉 Account has funds! Ready for transaction testing");
            console.log("🚀 Can now run full integration tests");
        } else {
            console.log("⚠️ Account has 0 ETH - may need funding");
        }
    } else {
        console.log("❌ Account mismatch - environment variable may not be set correctly");
    }
    
    // Show what we can do now
    console.log("\n🧪 Available Tests:");
    console.log("- npm run test:working (no gas required)");
    console.log("- npm run test:integration (requires ETH) -", parseFloat(balanceEth) > 0 ? "✅ READY" : "❌ NEEDS ETH");
    
    // Fun fact: Check if this is the contract owner
    if (address === "0x0a9871196E546a277072a04a6E1C1bC2CC25aaA2") {
        console.log("\n🔑 BONUS: This is the contract OWNER account!");
        console.log("   - Can call admin functions on DeviceRegistry");
        console.log("   - Has special privileges in the contracts");
    }
}

checkBalance().catch(console.error); 