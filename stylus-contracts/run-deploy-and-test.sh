#!/bin/bash

# =============================================================================
# L{CORE} Master Deploy & Test Script
# Deploys contracts and runs complete test suite
# =============================================================================

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 L{CORE} Deploy & Test Master Script${NC}"
echo -e "${BLUE}======================================${NC}"

# Check prerequisites
if [ -z "$PRIVATE_KEY" ]; then
    echo -e "${RED}❌ PRIVATE_KEY environment variable not set${NC}"
    echo -e "${YELLOW}   Set it with: export PRIVATE_KEY=your_private_key_here${NC}"
    exit 1
fi

if ! command -v cargo &> /dev/null; then
    echo -e "${RED}❌ Cargo not found. Please install Rust first.${NC}"
    exit 1
fi

if ! command -v cast &> /dev/null; then
    echo -e "${RED}❌ cast (foundry) not found. Please install foundry first.${NC}"
    echo -e "${YELLOW}   Install with: curl -L https://foundry.paradigm.xyz | bash${NC}"
    exit 1
fi

# Install cargo-stylus if needed
if ! command -v cargo-stylus &> /dev/null; then
    echo -e "${YELLOW}⚠️ cargo-stylus not found. Installing...${NC}"
    cargo install --force cargo-stylus
fi

echo -e "${GREEN}✅ All prerequisites installed${NC}"

# =============================================================================
# Phase 1: Deploy Contracts
# =============================================================================

echo -e "\n${BLUE}📦 Phase 1: Deploying Contracts${NC}"
echo -e "${BLUE}================================${NC}"

if [ -f "deploy.sh" ]; then
    chmod +x deploy.sh
    echo -e "${YELLOW}Running deployment script...${NC}"
    ./deploy.sh
else
    echo -e "${RED}❌ deploy.sh not found${NC}"
    exit 1
fi

# Check if deployment was successful
if [ ! -f "deviceregistry_address.txt" ] || [ ! -f "iotdatapipeline_address.txt" ]; then
    echo -e "${RED}❌ Deployment failed - contract addresses not found${NC}"
    exit 1
fi

DEVICE_REGISTRY=$(cat deviceregistry_address.txt)
IOT_PIPELINE=$(cat iotdatapipeline_address.txt)

echo -e "${GREEN}✅ Deployment completed successfully!${NC}"
echo -e "${GREEN}DeviceRegistry: $DEVICE_REGISTRY${NC}"
echo -e "${GREEN}IoTDataPipeline: $IOT_PIPELINE${NC}"

# =============================================================================
# Phase 2: Test Contracts
# =============================================================================

echo -e "\n${BLUE}🧪 Phase 2: Testing Contracts${NC}"
echo -e "${BLUE}==============================${NC}"

if [ -f "test-contracts.sh" ]; then
    chmod +x test-contracts.sh
    echo -e "${YELLOW}Running contract tests...${NC}"
    ./test-contracts.sh
else
    echo -e "${RED}❌ test-contracts.sh not found${NC}"
    exit 1
fi

# =============================================================================
# Phase 3: Final Summary
# =============================================================================

echo -e "\n${GREEN}🎉 =================================${NC}"
echo -e "${GREEN}   DEPLOYMENT & TESTING COMPLETE!${NC}"
echo -e "${GREEN}=================================${NC}"

echo -e "\n${BLUE}📋 Final Contract Addresses:${NC}"
echo -e "${YELLOW}DeviceRegistry: $DEVICE_REGISTRY${NC}"
echo -e "${YELLOW}IoTDataPipeline: $IOT_PIPELINE${NC}"

echo -e "\n${BLUE}📊 All Tests Passed:${NC}"
echo -e "${GREEN}✅ Contract deployment${NC}"
echo -e "${GREEN}✅ Device registration${NC}"
echo -e "${GREEN}✅ Device verification${NC}"
echo -e "${GREEN}✅ Access control${NC}"
echo -e "${GREEN}✅ Data pipeline${NC}"
echo -e "${GREEN}✅ Cross-contract integration${NC}"

echo -e "\n${BLUE}🚀 Ready for Next Steps:${NC}"
echo -e "${YELLOW}1. Update lcore-node with contract addresses${NC}"
echo -e "${YELLOW}2. Build and deploy new lcore-node image${NC}"
echo -e "${YELLOW}3. Test end-to-end data flow${NC}"

echo -e "\n${BLUE}💾 Contract addresses saved to:${NC}"
echo -e "${YELLOW}- deviceregistry_address.txt${NC}"
echo -e "${YELLOW}- iotdatapipeline_address.txt${NC}"

echo -e "\n${GREEN}All systems ready for production! 🎉${NC}" 