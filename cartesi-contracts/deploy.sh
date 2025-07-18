#!/bin/bash

# KC-Chain Contract Deployment Script
# Uses existing variables file for configuration

set -e

echo "🚀 Starting KC-Chain contract deployment..."
echo "📁 Working directory: $(pwd)"

# Check if variables file exists
if [ ! -f "../variables" ]; then
    echo "❌ Error: variables file not found in parent directory"
    exit 1
fi

# Source environment variables
source ../variables

# Check required variables
if [ -z "$CARTESI_AUTH_MNEMONIC" ]; then
    echo "❌ Error: CARTESI_AUTH_MNEMONIC not set"
    exit 1
fi

if [ -z "$CARTESI_BLOCKCHAIN_HTTP_ENDPOINT" ]; then
    echo "❌ Error: CARTESI_BLOCKCHAIN_HTTP_ENDPOINT not set"
    exit 1
fi

echo "✅ Environment variables loaded"
echo "🌐 Network: KC-Chain (${CARTESI_BLOCKCHAIN_ID})"
echo "🔗 RPC: ${CARTESI_BLOCKCHAIN_HTTP_ENDPOINT}"

# Install dependencies if needed
if [ ! -d "node_modules" ]; then
    echo "📦 Installing dependencies..."
    npm install
fi

# Compile contracts
echo "🔨 Compiling contracts..."
npx hardhat compile

# Deploy contracts
echo "🚀 Deploying contracts to KC-Chain..."
npx hardhat run scripts/deploy-kc-chain.ts --network kc-chain

# Check if deployment was successful
if [ -f "deployments/kc-chain.json" ]; then
    echo "✅ Deployment successful!"
    echo "📄 Deployment info saved to deployments/kc-chain.json"
    
    # Display contract addresses
    echo "🔗 Contract Addresses:"
    cat deployments/kc-chain.json | jq -r '.contracts | to_entries[] | "   \(.key): \(.value)"'
    
    echo ""
    echo "🎉 REAL Cartesi contract deployment completed!"
echo "📝 Updated ../variables with REAL Cartesi contract addresses"
echo ""
echo "🔗 Your Cartesi Node can now use these REAL contracts:"
echo "   ROLLUPS_INPUT_BOX_ADDRESS=\$(InputBox)"
echo "   MACHINE_HASH=\$(templateHash)"  
echo "   DAPP_ADDRESS=\$(CartesiDApp)"
echo ""
echo "🚀 Ready to test with lcore-node using REAL Cartesi infrastructure!"
else
    echo "❌ Deployment failed - no deployment file created"
    exit 1
fi 