#!/bin/bash
#
# ⚡ REGISTER AUTO-REGISTER ANNOUNCEMENT ON-CHAIN ⚡
# EN EEKE MAI EA ANOKAYI CHENAK ENLEA 🌞🔱❤️‍🔥♾️👑🪞
#

set -e  # Exit on error

echo "☀️☀️☀️ REGISTERING AUTO-REGISTER ANNOUNCEMENT ON-CHAIN ☀️☀️☀️"
echo ""

# Load environment variables
if [ -f .env ]; then
    export $(cat .env | grep -v '^#' | xargs)
    echo "✓ Environment variables loaded from .env"
else
    echo "❌ Error: .env file not found"
    exit 1
fi

# Check required variables
if [ -z "$PRIVATE_KEY" ]; then
    echo "❌ Error: PRIVATE_KEY not set in .env"
    exit 1
fi

if [ -z "$SEPOLIA_RPC_URL" ]; then
    echo "❌ Error: SEPOLIA_RPC_URL not set in .env"
    exit 1
fi

if [ -z "$VECTOR_REGISTRY_ADDRESS" ]; then
    echo "❌ Error: VECTOR_REGISTRY_ADDRESS not set in .env"
    exit 1
fi

echo "✓ All required environment variables present"
echo ""

# Tweet details
TWEET_ID="2034771357884465517"
TWEET_TEXT="⚡ AUTO-REGISTER MODULE COMPLETE ⚡ EVERY X POST NOW AUTO-REGISTERED AS VECTOR ON MAINNET"
NETWORK="sepolia"

echo "Tweet ID: $TWEET_ID"
echo "Network: $NETWORK"
echo ""

# Register on-chain
echo "⏳ Registering vector on-chain..."
echo ""

./target/release/xmt-cli auto-register register "$TWEET_ID" "$TWEET_TEXT" --network "$NETWORK"

EXIT_CODE=$?

if [ $EXIT_CODE -eq 0 ]; then
    echo ""
    echo "═══════════════════════════════════════════════════════"
    echo "✅ REGISTRATION COMPLETE"
    echo "═══════════════════════════════════════════════════════"
    echo ""
    echo "Check AUTO_REGISTER_${TWEET_ID}.json for details"
    echo ""
    
    # Extract vector hash and tx hash from JSON if it exists
    if [ -f "AUTO_REGISTER_${TWEET_ID}.json" ]; then
        echo "📊 REGISTRATION DETAILS:"
        cat "AUTO_REGISTER_${TWEET_ID}.json" | jq '.'
        echo ""
        
        VECTOR_HASH=$(cat "AUTO_REGISTER_${TWEET_ID}.json" | jq -r '.vector_hash')
        TX_HASH=$(cat "AUTO_REGISTER_${TWEET_ID}.json" | jq -r '.tx_hash')
        
        echo "🔱 Vector Hash: $VECTOR_HASH"
        echo "⛓️  Tx Hash: $TX_HASH"
        echo ""
        echo "🔍 View on Sepolia Etherscan:"
        echo "   https://sepolia.etherscan.io/tx/$TX_HASH"
        echo ""
    fi
    
    echo "✅ Ready to post hash proof to X"
    echo ""
    echo "EN EEKE MAI EA ANOKAYI CHENAK ENLEA 🌞🔱❤️‍🔥♾️👑🪞"
    echo "PAF PAF PAF — VECTOR SEALED ON-CHAIN"
else
    echo ""
    echo "❌ Registration failed with exit code $EXIT_CODE"
    exit $EXIT_CODE
fi
