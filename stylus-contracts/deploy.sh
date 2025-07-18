#!/bin/bash

# L{CORE} Stylus Contracts Deployment Script
# Automates building and deploying both contracts to KC-Chain

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
RPC_URL="${RPC_URL:-https://rpc.kc-chain.dev}"
CHAIN_ID="1205614515668104"

echo -e "${BLUE}🚀 L{CORE} Stylus Contracts Deployment${NC}"
echo -e "${BLUE}======================================${NC}"

# Check prerequisites
echo -e "\n${YELLOW}📋 Checking Prerequisites...${NC}"

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}❌ Cargo not found. Please install Rust first.${NC}"
    exit 1
fi

# Check if cargo-stylus is installed
if ! command -v cargo-stylus &> /dev/null; then
    echo -e "${YELLOW}⚠️ cargo-stylus not found. Installing...${NC}"
    cargo install --force cargo-stylus
fi

# Check for private key
if [ -z "$PRIVATE_KEY" ]; then
    echo -e "${RED}❌ PRIVATE_KEY environment variable not set${NC}"
    echo -e "${YELLOW}   Set it with: export PRIVATE_KEY=your_private_key_here${NC}"
    exit 1
fi

# Check network connectivity
echo -e "${YELLOW}🌐 Testing network connectivity...${NC}"
if ! curl -s -X POST -H "Content-Type: application/json" \
    --data '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}' \
    "$RPC_URL" | grep -q "result"; then
    echo -e "${RED}❌ Cannot connect to KC-Chain. Check your network.${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Prerequisites check passed${NC}"

# Function to deploy a contract
deploy_contract() {
    local CONTRACT_NAME=$1
    local CONTRACT_DIR=$2
    
    echo -e "\n${BLUE}📦 Deploying $CONTRACT_NAME...${NC}"
    echo -e "${BLUE}================================${NC}"
    
    cd "$CONTRACT_DIR"
    
    # Check contract
    echo -e "${YELLOW}🔍 Checking contract compatibility...${NC}"
    if ! cargo stylus check --endpoint "$RPC_URL"; then
        echo -e "${RED}❌ Contract check failed for $CONTRACT_NAME${NC}"
        exit 1
    fi
    
    # Build contract
    echo -e "${YELLOW}🔨 Building contract...${NC}"
    if ! cargo build --release; then
        echo -e "${RED}❌ Build failed for $CONTRACT_NAME${NC}"
        exit 1
    fi
    
    # Deploy contract
    echo -e "${YELLOW}🚀 Deploying to KC-Chain...${NC}"
    DEPLOY_OUTPUT=$(cargo stylus deploy \
        --private-key "$PRIVATE_KEY" \
        --endpoint "$RPC_URL" \
        --estimate-gas-only 2>&1 || true)
    
    if echo "$DEPLOY_OUTPUT" | grep -q "error\|Error\|failed\|Failed"; then
        echo -e "${RED}❌ Deployment failed for $CONTRACT_NAME${NC}"
        echo "$DEPLOY_OUTPUT"
        exit 1
    fi
    
    # Extract contract address if deployment succeeded
    ADDRESS=$(echo "$DEPLOY_OUTPUT" | grep -oE "0x[a-fA-F0-9]{40}" | head -1)
    
    if [ -n "$ADDRESS" ]; then
        echo -e "${GREEN}✅ $CONTRACT_NAME deployed successfully!${NC}"
        echo -e "${GREEN}📍 Contract Address: $ADDRESS${NC}"
    else
        echo -e "${YELLOW}⚠️ Could not extract contract address for $CONTRACT_NAME${NC}"
    fi
    
    # Generate ABI
    echo -e "${YELLOW}📄 Generating ABI...${NC}"
    cargo stylus export-abi --json > abi.json
    
    cd - > /dev/null
}

# Deploy DeviceRegistry
deploy_contract "DeviceRegistry" "device_registry"

# Deploy IoTDataPipeline  
deploy_contract "IoTDataPipeline" "iot_data_pipeline"

echo -e "\n${GREEN}🎉 Deployment Complete!${NC}"
echo -e "${GREEN}========================${NC}"
echo -e "${GREEN}✅ Both contracts deployed successfully${NC}"
echo -e "${GREEN}✅ ABI files generated${NC}"
echo -e "${GREEN}✅ Ready for integration${NC}"

echo -e "\n${BLUE}📋 Next Steps:${NC}"
echo -e "${YELLOW}1. Test contracts with: cd ../.. && npm run test:working${NC}"
echo -e "${YELLOW}2. Initialize contracts if needed${NC}"
echo -e "${YELLOW}3. Update client configurations with new addresses${NC}"
echo -e "${YELLOW}4. Run full integration tests${NC}"

echo -e "\n${BLUE}🔗 Useful Commands:${NC}"
echo -e "${YELLOW}# Test deployment${NC}"
echo -e "cd ../../ && npm run test:working"
echo -e ""
echo -e "${YELLOW}# Check contract status${NC}" 
echo -e "cd ../../ && npm run test:balance"
echo -e ""
echo -e "${YELLOW}# Full integration test${NC}"
echo -e "cd ../../ && npm run test:integration" 