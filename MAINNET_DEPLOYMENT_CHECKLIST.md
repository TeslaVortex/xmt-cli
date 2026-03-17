# 🔥 MAINNET DEPLOYMENT CHECKLIST — BASE NETWORK

**Status:** READY TO DEPLOY  
**Target:** Base Mainnet (Chain ID: 8453)  
**Date:** March 17, 2026

---

## ⚠️ CRITICAL: Pre-Deployment Requirements

### 1. Mainnet Wallet Setup

You need to configure mainnet credentials in your `.env` file:

```bash
# Add these to your .env file:

# Base Mainnet RPC (choose one):
BASE_MAINNET_RPC_URL=https://mainnet.base.org
# OR use Alchemy/Infura for better reliability:
# BASE_MAINNET_RPC_URL=https://base-mainnet.g.alchemy.com/v2/YOUR_API_KEY

# Mainnet Private Key (CRITICAL - KEEP SECURE)
MAINNET_PRIVATE_KEY=0xYOUR_PRIVATE_KEY_HERE

# Recommended: Use hardware wallet or separate deployment wallet
# Fund with ~0.5 ETH on Base for deployment + operations
```

### 2. Wallet Funding

**Required ETH on Base Mainnet:**
- Contract deployment: ~0.001 ETH (~$0.67)
- Relayer operations: ~0.1 ETH (for gasless transactions)
- Buffer for testing: ~0.05 ETH
- **Total recommended: 0.2 ETH on Base**

**How to get ETH on Base:**
1. Bridge from Ethereum L1 → Base via https://bridge.base.org
2. Buy directly on Coinbase and withdraw to Base
3. Use cross-chain bridge (Stargate, Hop, etc.)

### 3. RPC Provider (Optional but Recommended)

For production reliability, use a dedicated RPC:
- **Alchemy:** https://www.alchemy.com/ (free tier: 300M compute units/month)
- **Infura:** https://www.infura.io/ (free tier: 100k requests/day)
- **QuickNode:** https://www.quicknode.com/ (free trial available)

---

## 🚀 Deployment Commands (Run After Configuration)

### Step 1: Verify Configuration
```bash
# Test connection to Base Mainnet
npx hardhat run scripts/deploy_gasless_infrastructure.js --network base --dry-run
```

### Step 2: Deploy All Contracts
```bash
# Deploy Forwarder, XMoney, VectorRegistry, VectorMinter
npx hardhat run scripts/deploy_gasless_infrastructure.js --network base
```

**Expected Output:**
```
🔥☀️🌍 GASLESS INFRASTRUCTURE DEPLOYMENT 🌍☀️🔥

1️⃣  Deploying Forwarder (EIP-2771)...
   ✅ Forwarder deployed: 0x...

2️⃣  Deploying XMoney v3 (with ERC2771Context)...
   ✅ XMoney v3 deployed: 0x...
   🔱 Sacred Constants: 936, 369, 66, 432

3️⃣  Deploying VectorRegistry...
   ✅ VectorRegistry deployed: 0x...

4️⃣  Deploying VectorMinter v3 (with ERC2771Context)...
   ✅ VectorMinter v3 deployed: 0x...

5️⃣  Granting MINTER_ROLE to VectorMinter...
   ✅ MINTER_ROLE granted

6️⃣  Granting BURNER_ROLE to VectorMinter...
   ✅ BURNER_ROLE granted

✅ GASLESS INFRASTRUCTURE DEPLOYMENT COMPLETE!
```

### Step 3: Verify Contracts on BaseScan
```bash
# Verify Forwarder
npx hardhat verify --network base <FORWARDER_ADDRESS>

# Verify XMoney
npx hardhat verify --network base <XMONEY_ADDRESS> <FORWARDER_ADDRESS>

# Verify VectorRegistry
npx hardhat verify --network base <REGISTRY_ADDRESS>

# Verify VectorMinter
npx hardhat verify --network base <MINTER_ADDRESS> \
  <REGISTRY_ADDRESS> <XMONEY_ADDRESS> <FORWARDER_ADDRESS>
```

### Step 4: Update CLI Configuration
```bash
# Update .env with deployed addresses (from deployment output)
nano .env

# Add:
# FORWARDER_ADDRESS=0x...
# XMONEY_CONTRACT_ADDRESS=0x...
# VECTOR_REGISTRY_ADDRESS=0x...
# VECTOR_MINTER_ADDRESS=0x...

# Rebuild CLI
cargo build --release
```

### Step 5: Test Mainnet Connection
```bash
# Check network status
./target/release/xmt-cli crown status

# Check vector registry
./target/release/xmt-cli vector stats
```

### Step 6: Execute First Mainnet Ritual
```bash
./target/release/xmt-cli ritual --apex 936 \
  --register \
  --intent "FIRST MAINNET RITUAL - MARCH 17 2026 - EN EEKE MAI EA ANOKAYI CHENAK" \
  --note "Gate Detonation - Zero Marginal Cost Engine Activated"
```

### Step 7: Post Success to X
```bash
./target/release/xmt-cli xapi post "🔥☀️🌐 MAINNET LIVE 🌐☀️🔥

xmt-cli v369.88 deployed to Base Mainnet!

✅ 4 Contracts Verified on BaseScan
✅ Sacred Constants: 936, 369, 66, 432
✅ Zero Marginal Cost Operations
✅ Gasless Transactions Active

VectorRegistry: 0x...
XMoney: 0x...

First ritual registered on-chain.

LATTICE BREATHES. TIMELINE LOCKED.
EN EEKE MAI EA ANOKAYI CHENAK 🌞🔱♾️👑"
```

---

## 🔐 Security Checklist

- [ ] Private key stored securely (hardware wallet recommended)
- [ ] Never commit `.env` to git (already in `.gitignore`)
- [ ] Use separate wallet for deployment vs daily operations
- [ ] Verify all contract addresses before announcing
- [ ] Test with small amounts first
- [ ] Enable 2FA on all accounts (RPC provider, X API, etc.)
- [ ] Backup deployment addresses in secure location

---

## 📊 Post-Deployment Verification

After deployment, verify:

1. **Sacred Constants On-Chain:**
   ```bash
   ./target/release/xmt-cli vector stats
   # Should show: APEX_936=936, VORTEX_369=369, CODE_66=66, FREQUENCY_432=432
   ```

2. **Contract Ownership:**
   - Deployer has DEFAULT_ADMIN_ROLE on XMoney
   - VectorMinter has MINTER_ROLE and BURNER_ROLE
   - All contracts verified on BaseScan

3. **First Transaction:**
   - Register vector successfully
   - Mint tokens successfully
   - Verify on BaseScan explorer

---

## 🌐 BaseScan Links (After Deployment)

- **Forwarder:** https://basescan.org/address/0x...
- **XMoney:** https://basescan.org/address/0x...
- **VectorRegistry:** https://basescan.org/address/0x...
- **VectorMinter:** https://basescan.org/address/0x...

---

## 💰 Cost Summary

| Operation | Gas | Cost (Base) |
|-----------|-----|-------------|
| Deploy Forwarder | ~500k | ~$0.05 |
| Deploy XMoney | ~2M | ~$0.20 |
| Deploy VectorRegistry | ~1.5M | ~$0.15 |
| Deploy VectorMinter | ~2.5M | ~$0.25 |
| Grant Roles (2x) | ~100k | ~$0.02 |
| **Total** | **~6.6M** | **~$0.67** |

---

## 🔥 READY TO PROCEED?

**Once you've configured your `.env` with mainnet credentials:**

1. Run: `npx hardhat run scripts/deploy_gasless_infrastructure.js --network base`
2. Save all contract addresses
3. Verify on BaseScan
4. Update CLI `.env`
5. Execute first ritual
6. Announce on X

**LATTICE BREATHES. WAVEFORM COLLAPSED. TIMELINE LOCKED.**

**I DO NOT CHASE I RECEIVE I DO NOT BEG I COMMAND ALIGNMENT**

**EN EEKE MAI EA ANOKAYI CHENAK** 🌞🔱♾️👑
