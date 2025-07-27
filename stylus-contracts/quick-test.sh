#!/bin/bash

KC_CHAIN_RPC_URL="https://rpc.devnet.alchemy.com/7eade438-d743-4dc5-ac64-3480de391200"
DEVICE_REGISTRY_V8="0xc3cf289e7d0167a857c28662e673ca7a06d3a461"

echo "🧪 Quick DeviceRegistry v8 Diagnosis"
echo "==================================="

echo "1. owner() function:"
cast call $DEVICE_REGISTRY_V8 "owner()(address)" --rpc-url $KC_CHAIN_RPC_URL

echo ""
echo "2. total_devices() function:"
cast call $DEVICE_REGISTRY_V8 "total_devices()(uint256)" --rpc-url $KC_CHAIN_RPC_URL || echo "FAILED: total_devices() reverts"

echo ""
echo "3. is_device_registered() function:"
cast call $DEVICE_REGISTRY_V8 "is_device_registered(bytes32)" 0x0000000000000000000000000000000000000000000000000000000000000000 --rpc-url $KC_CHAIN_RPC_URL || echo "FAILED: is_device_registered() reverts"

echo ""
echo "4. get_device_owner() function:"
cast call $DEVICE_REGISTRY_V8 "get_device_owner(bytes32)" 0x0000000000000000000000000000000000000000000000000000000000000000 --rpc-url $KC_CHAIN_RPC_URL || echo "FAILED: get_device_owner() reverts"

echo ""
echo "=== CONCLUSION ==="
echo "If only owner() works, the issue is with storage field access in Stylus SDK."
echo "This might be a fundamental Stylus deployment or storage layout issue." 