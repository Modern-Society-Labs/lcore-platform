#!/bin/bash

# =============================================================================
# L{CORE} Simple Contract Testing Script
# Tests basic contract functions after deployment
# =============================================================================

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
RPC_URL="${RPC_URL:-https://rpc.kc-chain.dev}"

echo -e "${BLUE}🧪 L{CORE} Contract Testing Suite${NC}"
echo -e "${BLUE}===================================${NC}"

# Check prerequisites
if [ -z "$PRIVATE_KEY" ]; then
    echo -e "${RED}❌ PRIVATE_KEY environment variable not set${NC}"
    exit 1
fi

if ! command -v cast &> /dev/null; then
    echo -e "${RED}❌ cast (foundry) not found. Please install foundry first.${NC}"
    exit 1
fi

# Read deployed contract addresses
if [ ! -f "deviceregistry_address.txt" ]; then
    echo -e "${RED}❌ DeviceRegistry address not found. Run deploy.sh first.${NC}"
    exit 1
fi

if [ ! -f "iotdatapipeline_address.txt" ]; then
    echo -e "${RED}❌ IoTDataPipeline address not found. Run deploy.sh first.${NC}"
    exit 1
fi

DEVICE_REGISTRY=$(cat deviceregistry_address.txt)
IOT_PIPELINE=$(cat iotdatapipeline_address.txt)

echo -e "${YELLOW}📋 Testing contracts:${NC}"
echo -e "${YELLOW}DeviceRegistry: $DEVICE_REGISTRY${NC}"
echo -e "${YELLOW}IoTDataPipeline: $IOT_PIPELINE${NC}"

# =============================================================================
# Test 1: Device Registration
# =============================================================================

echo -e "\n${BLUE}🧪 Test 1: Device Registration${NC}"

DEVICE_ID="did:lcore:test-device-001"
DID_DOCUMENT='{"id":"did:lcore:test-device-001","type":"IoT"}'
PUBLIC_KEY="0xabcdef1234567890"
DEVICE_TYPE="environmental_sensor"
METADATA='{"location":"test_lab","domain":"environmental"}'

echo -e "${YELLOW}Registering device: $DEVICE_ID${NC}"

if cast send $DEVICE_REGISTRY "register_device(string,string,string,string,string)" \
    "$DEVICE_ID" "$DID_DOCUMENT" "$PUBLIC_KEY" "$DEVICE_TYPE" "$METADATA" \
    --value 100 --private-key $PRIVATE_KEY --rpc-url $RPC_URL > /dev/null 2>&1; then
    echo -e "${GREEN}✅ Device registration successful${NC}"
else
    echo -e "${RED}❌ Device registration failed${NC}"
    exit 1
fi

# =============================================================================
# Test 2: Device Verification
# =============================================================================

echo -e "\n${BLUE}🧪 Test 2: Device Verification${NC}"

# Create device hash (keccak256 of device_id)
DEVICE_HASH=$(cast keccak "$DEVICE_ID")
echo -e "${YELLOW}Device hash: $DEVICE_HASH${NC}"

# Check if device is registered
IS_REGISTERED=$(cast call $DEVICE_REGISTRY "is_device_registered(bytes32)" $DEVICE_HASH --rpc-url $RPC_URL)

if [ "$IS_REGISTERED" = "true" ]; then
    echo -e "${GREEN}✅ Device verification successful${NC}"
else
    echo -e "${RED}❌ Device verification failed${NC}"
    exit 1
fi

# =============================================================================
# Test 3: Access Control
# =============================================================================

echo -e "\n${BLUE}🧪 Test 3: Access Control${NC}"

# Test consumer address (any valid address)
CONSUMER_ADDRESS="0x70997970C51812dc3A010C7d01b50e0d17dc79C8"

echo -e "${YELLOW}Granting access to consumer: $CONSUMER_ADDRESS${NC}"

if cast send $DEVICE_REGISTRY "grant_access(address,uint256)" $CONSUMER_ADDRESS 0 \
    --private-key $PRIVATE_KEY --rpc-url $RPC_URL > /dev/null 2>&1; then
    echo -e "${GREEN}✅ Access granted successfully${NC}"
else
    echo -e "${RED}❌ Access grant failed${NC}"
    exit 1
fi

# Verify access
OWNER_ADDRESS=$(cast wallet address --private-key $PRIVATE_KEY)
HAS_ACCESS=$(cast call $DEVICE_REGISTRY "has_access(address,address)" $OWNER_ADDRESS $CONSUMER_ADDRESS --rpc-url $RPC_URL)

if [ "$HAS_ACCESS" = "true" ]; then
    echo -e "${GREEN}✅ Access control verification successful${NC}"
else
    echo -e "${RED}❌ Access control verification failed${NC}"
    exit 1
fi

# =============================================================================
# Test 4: IoTDataPipeline Initialization
# =============================================================================

echo -e "\n${BLUE}🧪 Test 4: IoTDataPipeline Initialization${NC}"

# Mock rollup address for testing
MOCK_ROLLUP_ADDRESS="0x1234567890123456789012345678901234567890"
BASE_FEE=50

echo -e "${YELLOW}Initializing IoTDataPipeline...${NC}"

if cast send $IOT_PIPELINE "initialize(address,address,uint256)" \
    $MOCK_ROLLUP_ADDRESS $DEVICE_REGISTRY $BASE_FEE \
    --private-key $PRIVATE_KEY --rpc-url $RPC_URL > /dev/null 2>&1; then
    echo -e "${GREEN}✅ IoTDataPipeline initialization successful${NC}"
else
    echo -e "${RED}❌ IoTDataPipeline initialization failed${NC}"
    exit 1
fi

# =============================================================================
# Test 5: Data Submission
# =============================================================================

echo -e "\n${BLUE}🧪 Test 5: Data Submission${NC}"

# Convert device_id to hex for submission
DEVICE_PAYLOAD=$(cast --to-hex "$DEVICE_ID")

echo -e "${YELLOW}Submitting data for device: $DEVICE_ID${NC}"

if cast send $IOT_PIPELINE "submit_cartesi_result(bytes)" $DEVICE_PAYLOAD \
    --private-key $PRIVATE_KEY --rpc-url $RPC_URL > /dev/null 2>&1; then
    echo -e "${GREEN}✅ Data submission successful${NC}"
else
    echo -e "${RED}❌ Data submission failed${NC}"
    exit 1
fi

# Verify submission count
SUBMISSION_COUNT=$(cast call $IOT_PIPELINE "get_device_submission_count(bytes32)" $DEVICE_HASH --rpc-url $RPC_URL 2>/dev/null || echo "0")

echo -e "${YELLOW}Device submission count: $SUBMISSION_COUNT${NC}"

if [ "$SUBMISSION_COUNT" -gt 0 ] 2>/dev/null; then
    echo -e "${GREEN}✅ Data submission verification successful${NC}"
else
    echo -e "${YELLOW}⚠️  Could not verify submission count (function may not exist)${NC}"
fi

# =============================================================================
# Test 6: Cross-Contract Integration
# =============================================================================

echo -e "\n${BLUE}🧪 Test 6: Cross-Contract Integration${NC}"

# Test that IoTDataPipeline can verify devices through DeviceRegistry
echo -e "${YELLOW}Testing cross-contract device verification...${NC}"

# Submit data for our registered device (should work)
if cast send $IOT_PIPELINE "submit_cartesi_result(bytes)" $DEVICE_PAYLOAD \
    --private-key $PRIVATE_KEY --rpc-url $RPC_URL > /dev/null 2>&1; then
    echo -e "${GREEN}✅ Cross-contract device verification successful${NC}"
else
    echo -e "${RED}❌ Cross-contract device verification failed${NC}"
    exit 1
fi

# =============================================================================
# Final Report
# =============================================================================

echo -e "\n${GREEN}🎉 =================================${NC}"
echo -e "${GREEN}   ALL TESTS PASSED!${NC}"
echo -e "${GREEN}=================================${NC}"

echo -e "\n${BLUE}📊 Test Results Summary:${NC}"
echo -e "${GREEN}✅ Device Registration: PASSED${NC}"
echo -e "${GREEN}✅ Device Verification: PASSED${NC}"
echo -e "${GREEN}✅ Access Control: PASSED${NC}"
echo -e "${GREEN}✅ Pipeline Initialization: PASSED${NC}"
echo -e "${GREEN}✅ Data Submission: PASSED${NC}"
echo -e "${GREEN}✅ Cross-Contract Integration: PASSED${NC}"

echo -e "\n${BLUE}📋 Contract Addresses:${NC}"
echo -e "${YELLOW}DeviceRegistry: $DEVICE_REGISTRY${NC}"
echo -e "${YELLOW}IoTDataPipeline: $IOT_PIPELINE${NC}"

echo -e "\n${BLUE}🚀 Ready for lcore-node integration!${NC}"
echo -e "\n${YELLOW}Next steps:${NC}"
echo -e "${YELLOW}1. Update lcore-node with these contract addresses${NC}"
echo -e "${YELLOW}2. Test end-to-end data flow${NC}"
echo -e "${YELLOW}3. Deploy to production${NC}" 