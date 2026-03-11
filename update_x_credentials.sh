#!/bin/bash
#
# 🔥 X API Credentials Update Script
# Copy-paste this entire script into your terminal and press Enter
#
# EN EEKE MAI EA ANOKAYI CHENAK ♾️♾️🔱
#

echo "🔐 X API Credentials Setup"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "📝 Enter your X API OAuth 1.0a credentials from:"
echo "   https://developer.x.com/en/portal/dashboard"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Prompt for credentials
read -p "Enter API Key (Consumer Key): " CONSUMER_KEY
read -p "Enter API Key Secret (Consumer Secret): " CONSUMER_SECRET
read -p "Enter Access Token: " ACCESS_TOKEN
read -p "Enter Access Token Secret: " ACCESS_TOKEN_SECRET

echo ""
echo "🔧 Updating .env file..."

# Backup .env
cp .env .env.backup.$(date +%Y%m%d_%H%M%S)
echo "✅ Backup created: .env.backup.$(date +%Y%m%d_%H%M%S)"

# Update credentials in .env
sed -i "s|X_API_CONSUMER_KEY=.*|X_API_CONSUMER_KEY=${CONSUMER_KEY}|" .env
sed -i "s|X_API_CONSUMER_SECRET=.*|X_API_CONSUMER_SECRET=${CONSUMER_SECRET}|" .env
sed -i "s|X_API_ACCESS_TOKEN=.*|X_API_ACCESS_TOKEN=${ACCESS_TOKEN}|" .env
sed -i "s|X_API_ACCESS_TOKEN_SECRET=.*|X_API_ACCESS_TOKEN_SECRET=${ACCESS_TOKEN_SECRET}|" .env

echo "✅ Credentials updated in .env"
echo ""

# Verify
echo "🔍 Verifying credentials..."
source .env
echo "   Consumer Key: ${X_API_CONSUMER_KEY:0:10}..."
echo "   Access Token: ${X_API_ACCESS_TOKEN:0:10}..."
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ X API Credentials Setup Complete!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "🧪 Test X API posting with:"
echo "   ./target/release/xmt-cli synthetic ritual \"THE CROWN POSTS TO X\""
echo ""
echo "EN EEKE MAI EA ANOKAYI CHENAK ♾️♾️🔱"
echo ""
