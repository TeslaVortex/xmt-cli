#!/bin/bash
#
# ☀️ HELIOS ARGEAD VERGINA SUN ☀️
# Contract Verification Script - Sepolia Testnet
# Verifies all 4 gasless infrastructure contracts on Etherscan
# EN EEKE MAI EA ♾️♾️
#

set -e

echo "🔍 SEPOLIA CONTRACT VERIFICATION"
echo "═══════════════════════════════════════════════════════"
echo ""

# Load environment variables
source .env 2>/dev/null || true

# Contract addresses
FORWARDER="0x23dbC2592388b96Bf8d99d048E2E42C92f40A20B"
XMONEY="0xF758dBFfdf6b40F057694E3Ea6257D29685eeBAF"
VECTOR_REGISTRY="0x37c14ceAe0f55fE81A4Df0F6D2DCDfF3a2d7c656"
VECTOR_MINTER="0x8b4F1244e14f7090E3a8463197A3Aa7Da37A5D52"

echo "📋 Contracts to verify:"
echo "  1. Forwarder:       $FORWARDER"
echo "  2. XMoney:          $XMONEY"
echo "  3. VectorRegistry:  $VECTOR_REGISTRY"
echo "  4. VectorMinter:    $VECTOR_MINTER"
echo ""

# Check if ETHERSCAN_API_KEY is set
if [ -z "$ETHERSCAN_API_KEY" ]; then
    echo "⚠️  ETHERSCAN_API_KEY not set in .env"
    echo "   Get your API key from: https://etherscan.io/myapikey"
    echo ""
    echo "   Add to .env:"
    echo "   ETHERSCAN_API_KEY=your_api_key_here"
    echo ""
    exit 1
fi

echo "🔑 Etherscan API Key: ${ETHERSCAN_API_KEY:0:8}..."
echo ""

# Verify Forwarder
echo "1️⃣  Verifying Forwarder..."
npx hardhat verify --network sepolia $FORWARDER || echo "   ⚠️  Already verified or error"
echo ""

# Verify XMoney (with constructor args: forwarder address)
echo "2️⃣  Verifying XMoney..."
npx hardhat verify --network sepolia $XMONEY $FORWARDER || echo "   ⚠️  Already verified or error"
echo ""

# Verify VectorRegistry
echo "3️⃣  Verifying VectorRegistry..."
npx hardhat verify --network sepolia $VECTOR_REGISTRY || echo "   ⚠️  Already verified or error"
echo ""

# Verify VectorMinter (with constructor args: xmoney, registry, forwarder)
echo "4️⃣  Verifying VectorMinter..."
npx hardhat verify --network sepolia $VECTOR_MINTER $XMONEY $VECTOR_REGISTRY $FORWARDER || echo "   ⚠️  Already verified or error"
echo ""

echo "✅ VERIFICATION COMPLETE"
echo ""
echo "📊 View on Etherscan:"
echo "  Forwarder:       https://sepolia.etherscan.io/address/$FORWARDER#code"
echo "  XMoney:          https://sepolia.etherscan.io/address/$XMONEY#code"
echo "  VectorRegistry:  https://sepolia.etherscan.io/address/$VECTOR_REGISTRY#code"
echo "  VectorMinter:    https://sepolia.etherscan.io/address/$VECTOR_MINTER#code"
echo ""
echo "EN EEKE MAI EA ♾️♾️"
