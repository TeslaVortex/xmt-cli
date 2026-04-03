# ⚡ AUTO-REGISTER — X POSTS AUTOMATICALLY BECOME VECTORS ON MAINNET ⚡

**EN EEKE MAI EA ANOKAYI CHENAK ENLEA 🌞🔱❤️‍🔥♾️👑🪞**

## Overview

The `auto-register` command automatically posts to X (Twitter) and registers each post as a 384D vector on mainnet (Base) or Sepolia testnet. Every tweet becomes an immutable on-chain vector with cryptographic proof.

## Command Syntax

### Auto-Post & Register
```bash
xmt-cli auto-register post <text> [--network mainnet|sepolia]
```

### Register Existing Tweet
```bash
xmt-cli auto-register register <tweet_id> <text> [--network mainnet|sepolia]
```

## Parameters

### `post` subcommand
- `<text>` - Tweet text to post (required)
- `--network` - Target network: `mainnet` (Base) or `sepolia` (default: `sepolia`)

### `register` subcommand
- `<tweet_id>` - Existing tweet ID (required)
- `<text>` - Tweet text content (required)
- `--network` - Target network: `mainnet` (Base) or `sepolia` (default: `sepolia`)

## Example Usage

### Post & Auto-Register on Sepolia
```bash
xmt-cli auto-register post "EN EEKE MAI EA ♾️♾️ — 936 Apex Ritual Complete" --network sepolia
```

### Post & Auto-Register on Mainnet (Base)
```bash
xmt-cli auto-register post "⚡ CARRINGTON EVENT COMPLETE ⚡ Schumann Resonance Integrated" --network mainnet
```

### Register Existing Tweet
```bash
xmt-cli auto-register register 2034769206755864894 "I AM THE CARRINGTON EVENT" --network mainnet
```

## Workflow

The auto-register command executes the following steps automatically:

### Step 1: Post to X API
- Authenticates with X API using OAuth 1.0a
- Posts tweet with provided text
- Returns tweet ID and confirmation

### Step 2: Generate Vector
- Creates 384D vector from tweet text
- Uses deterministic hashing for reproducibility
- Generates unique vector signature

### Step 3: Register On-Chain
- Connects to VectorRegistry contract
- Submits registration transaction
- Records vector hash, intent, and dimensions
- Returns transaction hash and block number

### Step 4: Save Result
- Saves complete result to `AUTO_REGISTER_<tweet_id>.json`
- Includes tweet ID, vector hash, tx hash, block number
- Permanent record of post-to-vector registration

## Output Example

```
☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️
   ⚡ AUTO-POST & REGISTER INITIATED ⚡
   X POST → MAINNET VECTOR REGISTRATION
☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️

📡 STEP 1: POSTING TO X API...

✅ TWEET POSTED SUCCESSFULLY
   Tweet ID: 2034769206755864894
   Text: ⚡ I AM THE CARRINGTON EVENT ⚡

⛓️  STEP 2: REGISTERING VECTOR ON-CHAIN...

   Vector Hash: 0xf48654bda3829dacaff5ee9e23baf7f781f65d10b36aa7951b4dd4782bca39e9
   Dimensions: 384
   Network: mainnet
   Chain ID: 8453
   Registry: 0x44254bebf12eea98471b5e398a4ba030140ef18f

   ⏳ Submitting transaction...

✅ VECTOR REGISTERED ON-CHAIN
   Tx Hash: 0xabc123...
   Block: 12345678
   Gas Used: 150000

═══════════════════════════════════════════════════════
   ✅ AUTO-REGISTER COMPLETE
═══════════════════════════════════════════════════════

✓ Tweet Posted: 2034769206755864894
✓ Vector Hash: 0xf48654bda3829dacaff5ee9e23baf7f781f65d10b36aa7951b4dd4782bca39e9
✓ On-Chain: REGISTERED
  Tx: 0xabc123...
  Block: 12345678

EN EEKE MAI EA ANOKAYI CHENAK ENLEA 🌞🔱❤️‍🔥♾️👑🪞
PAF PAF PAF — VECTOR SEALED ON-CHAIN

📄 Result saved to: AUTO_REGISTER_2034769206755864894.json
```

## Output File Structure

Each auto-registration saves a JSON file with complete details:

```json
{
  "tweet_id": "2034769206755864894",
  "tweet_text": "⚡ I AM THE CARRINGTON EVENT ⚡",
  "vector_hash": "0xf48654bda3829dacaff5ee9e23baf7f781f65d10b36aa7951b4dd4782bca39e9",
  "tx_hash": "0xabc123...",
  "block_number": 12345678,
  "registered": true,
  "timestamp": "2026-03-19T23:12:00Z"
}
```

## Environment Variables

Required environment variables in `.env`:

### X API (for posting)
```bash
X_API_CONSUMER_KEY=your_consumer_key
X_API_CONSUMER_SECRET=your_consumer_secret
X_API_ACCESS_TOKEN=your_access_token
X_API_ACCESS_TOKEN_SECRET=your_access_token_secret
```

### Blockchain (for vector registration)
```bash
PRIVATE_KEY=your_wallet_private_key
VECTOR_REGISTRY_ADDRESS=0x44254bebf12eea98471b5e398a4ba030140ef18f
```

### Network-Specific
```bash
# For mainnet (Base)
BASE_RPC_URL=https://mainnet.base.org

# For Sepolia testnet
SEPOLIA_RPC_URL=https://sepolia.infura.io/v3/YOUR_KEY
```

## Use Cases

### 1. Ritual Announcements
Post ritual completions and automatically register them as vectors:
```bash
xmt-cli auto-register post "936 Apex Ritual Complete — 100% Coherence Locked" --network mainnet
```

### 2. Timeline Events
Register major timeline events on-chain:
```bash
xmt-cli auto-register post "Schumann Resonance Integrated — Earth Heartbeat Synced" --network mainnet
```

### 3. Convergence Milestones
Document convergence milestones permanently:
```bash
xmt-cli auto-register post "12 Gates Synchronized — Full Convergence Complete" --network mainnet
```

### 4. Retroactive Registration
Register important past tweets as vectors:
```bash
xmt-cli auto-register register 2034764008394309684 "Full Convergence Complete" --network mainnet
```

## Technical Details

### Vector Generation
- **Algorithm:** Deterministic hashing from tweet text
- **Dimensions:** 384D (standard for all vectors)
- **Hash Function:** SHA-256 → bytes32
- **Reproducibility:** Same text always generates same vector

### On-Chain Storage
- **Contract:** VectorRegistry (Solidity)
- **Storage:** Vector hash, intent text, dimensions, timestamp
- **Immutability:** Once registered, vectors cannot be modified
- **Verification:** Anyone can verify vector on-chain

### Gas Costs
- **Sepolia:** ~150,000 gas (free testnet ETH)
- **Base Mainnet:** ~150,000 gas (~$0.01-0.05 depending on gas price)

## Integration with Other Commands

The auto-register module integrates seamlessly with existing xmt-cli commands:

### After Auto-Registration
```bash
# 1. Auto-register a post
xmt-cli auto-register post "936 Ritual Complete" --network mainnet

# 2. Mint tokens for the vector
xmt-cli vector mint 0xf48654bda3829dacaff5ee9e23baf7f781f65d10b36aa7951b4dd4782bca39e9

# 3. Check vector stats
xmt-cli vector stats 0xf48654bda3829dacaff5ee9e23baf7f781f65d10b36aa7951b4dd4782bca39e9
```

### Combined Workflow
```bash
# Full ritual + post + register + mint workflow
xmt-cli ritual --apex 936 --register
xmt-cli auto-register post "936 Apex Ritual Complete" --network mainnet
xmt-cli vector mint <vector_hash>
```

## Troubleshooting

### "environment variable not found"
Make sure all required environment variables are set in `.env`:
```bash
cp .env.example .env
# Edit .env with your credentials
```

### "Failed to post tweet"
Check X API credentials:
- Verify OAuth 1.0a credentials are correct
- Ensure app has read/write permissions
- Check rate limits (300 posts per 3 hours)

### "Vector registration failed"
Check blockchain connection:
- Verify RPC URL is correct and accessible
- Ensure wallet has sufficient ETH for gas
- Confirm VectorRegistry contract address is correct

### "Transaction reverted"
Common causes:
- Vector already registered (check with `xmt-cli vector verify <hash>`)
- Insufficient gas
- Network congestion

## Best Practices

1. **Test on Sepolia First** - Always test new posts on Sepolia before mainnet
2. **Save Output Files** - Keep AUTO_REGISTER_*.json files for records
3. **Verify On-Chain** - Use block explorer to verify registration
4. **Monitor Gas** - Check gas prices before mainnet registration
5. **Backup Keys** - Keep private keys secure and backed up

## Advanced Usage

### Batch Registration
Register multiple tweets in sequence:
```bash
for tweet_id in 123 456 789; do
  xmt-cli auto-register register $tweet_id "Event $tweet_id" --network mainnet
  sleep 5  # Wait between transactions
done
```

### Conditional Registration
Only register if not already registered:
```bash
# Check if vector exists first
xmt-cli vector verify 0xf48654... || \
  xmt-cli auto-register register 2034769206755864894 "Text" --network mainnet
```

---

## Summary

The auto-register module makes every X post an immutable on-chain vector:

✅ **Automatic** - Post and register in one command  
✅ **Permanent** - Vectors stored on-chain forever  
✅ **Verifiable** - Anyone can verify vector authenticity  
✅ **Integrated** - Works with existing vector/mint commands  
✅ **Flexible** - Supports mainnet and testnet  

**EN EEKE MAI EA ANOKAYI CHENAK ENLEA 🌞🔱❤️‍🔥♾️👑🪞**  
**PAF PAF PAF — EVERY POST IS NOW A VECTOR ON-CHAIN**
