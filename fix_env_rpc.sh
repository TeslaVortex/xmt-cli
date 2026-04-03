#!/bin/bash
#
# ⚡ FIX .env RPC CONFIGURATION ⚡
# CRITICAL: SEPOLIA_RPC_URL is currently pointing to Base mainnet
# This script fixes the RPC URLs to point to correct networks
#

set -e

echo "☀️☀️☀️ FIXING .env RPC CONFIGURATION ☀️☀️☀️"
echo ""

if [ ! -f .env ]; then
    echo "❌ Error: .env file not found"
    exit 1
fi

# Backup .env
cp .env .env.backup
echo "✓ Backed up .env to .env.backup"

# Check current values
echo ""
echo "📊 CURRENT VALUES:"
grep "^SEPOLIA_RPC_URL=" .env || echo "SEPOLIA_RPC_URL not found"
grep "^BASE_RPC_URL=" .env || echo "BASE_RPC_URL not found"
echo ""

# Fix SEPOLIA_RPC_URL (currently pointing to Base mainnet - WRONG!)
# Replace with actual Sepolia RPC
if grep -q "^SEPOLIA_RPC_URL=https://mainnet.base.org" .env; then
    echo "⚠️  CRITICAL: SEPOLIA_RPC_URL is pointing to Base mainnet!"
    echo "   Fixing to use Sepolia testnet..."
    
    # Use Alchemy Sepolia endpoint (free tier available)
    sed -i 's|^SEPOLIA_RPC_URL=https://mainnet.base.org|SEPOLIA_RPC_URL=https://eth-sepolia.g.alchemy.com/v2/demo|g' .env
    echo "✓ Fixed SEPOLIA_RPC_URL to Sepolia testnet"
fi

# Ensure BASE_RPC_URL exists and is correct
if ! grep -q "^BASE_RPC_URL=" .env; then
    echo "BASE_RPC_URL=https://mainnet.base.org" >> .env
    echo "✓ Added BASE_RPC_URL for Base mainnet"
elif ! grep -q "^BASE_RPC_URL=https://mainnet.base.org" .env; then
    sed -i 's|^BASE_RPC_URL=.*|BASE_RPC_URL=https://mainnet.base.org|g' .env
    echo "✓ Fixed BASE_RPC_URL to Base mainnet"
fi

echo ""
echo "📊 NEW VALUES:"
grep "^SEPOLIA_RPC_URL=" .env
grep "^BASE_RPC_URL=" .env
echo ""

echo "═══════════════════════════════════════════════════════"
echo "✅ .env RPC CONFIGURATION FIXED"
echo "═══════════════════════════════════════════════════════"
echo ""
echo "SEPOLIA_RPC_URL now points to: Sepolia testnet (eth-sepolia.g.alchemy.com)"
echo "BASE_RPC_URL now points to: Base mainnet (mainnet.base.org)"
echo ""
echo "⚠️  NOTE: Using Alchemy demo endpoint for Sepolia"
echo "   For production, get your own API key from https://alchemy.com"
echo "   and update SEPOLIA_RPC_URL=https://eth-sepolia.g.alchemy.com/v2/YOUR_KEY"
echo ""
echo "✅ Ready to register vectors on-chain"
echo ""
