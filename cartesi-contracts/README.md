# Cartesi Rollups Infrastructure

This directory contains the Cartesi rollups smart contracts that provide the fraud-proof computation layer for the L{CORE} platform.

## Overview

The Cartesi contracts enable:
- **Fraud-proof computation** in RISC-V virtual machine
- **Data input/output** between IoT devices and blockchain
- **Authority management** for device authorization
- **Integration** with Stylus contracts for 10x gas efficiency

## Architecture

```
cartesi-contracts/
├── contracts/              # Cartesi smart contract source code
│   ├── dapp/               # DApp factory and application contracts
│   ├── inputs/             # InputBox for receiving IoT data
│   ├── consensus/          # Authority and consensus mechanisms
│   └── portals/            # Asset transfer portals
├── scripts/                # Deployment scripts
│   ├── deploy-kc-chain.ts  # Main deployment script
│   └── update-template-hash.ts # Update with real machine hash
├── deploy.sh               # One-command deployment
└── hardhat.config.ts       # KC-Chain network configuration
```

## Quick Deployment

Deploy complete Cartesi infrastructure:

```bash
cd cartesi-contracts
./deploy.sh
```

This will deploy:
- **InputBox**: Receives data from IoT devices
- **Authority**: Manages device authorization 
- **CartesiDApp**: Processes data in RISC-V VM
- **CartesiDAppFactory**: Creates new applications

## Integration with Stylus Contracts

The Cartesi contracts work together with the Stylus contracts:

1. **IoT Data Flow**: Device → Cartesi InputBox → RISC-V VM Processing
2. **Results Output**: VM → Cartesi Vouchers → Stylus Contracts
3. **Gas Efficiency**: Heavy computation (Cartesi) + Fast execution (Stylus)

## Contract Addresses (KC-Chain)

After deployment, contracts will be available at:
- InputBox: `(deployed address)`
- CartesiDApp: `(deployed address)`
- Authority: `(deployed address)`
- DappFactory: `(deployed address)`

## Prerequisites

```bash
# Install dependencies
npm install

# Ensure environment variables are set
source ../variables
```

## Testing

Test the complete Cartesi + Stylus integration:

```bash
# Deploy Cartesi infrastructure
./deploy.sh

# Test with lcore-node
cd ../lcore-node
./build.sh test-cartesi
```

## Machine Hash Integration

The deployment automatically integrates with your Cartesi machine:
- Reads templateHash from `.cartesi/image/hash`
- Updates deployment with real machine hash
- Enables fraud-proof verification

---

**Part of the L{CORE} Platform Smart Contract Stack** 