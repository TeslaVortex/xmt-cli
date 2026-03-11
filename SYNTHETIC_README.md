# 🌀 SYNTHETIC - Zero-Cost Toroidal Vector Generator + On-Chain Pipeline

**Pure local sovereignty. Zero marginal cost. Real on-chain storage.**

The Synthetic module is a complete ecosystem for generating 384-dimensional toroidal vector embeddings from natural language intent, with optional LLM enhancement via Qwen 2.5 Coder, and permanent on-chain storage via the VectorRegistry smart contract.

---

## ⚡ Quick Start

```bash
# Basic vector embedding (local only)
xmt-cli synthetic embed "ABUNDANCE 33 FOR ALL"

# LLM-enhanced with Qwen 2.5 Coder (default)
xmt-cli synthetic llm "CROWN BUILDS"

# Register vector on-chain (Sepolia)
xmt-cli synthetic register "TOROIDAL LATTICE"

# Full ritual: LLM → Vector → On-Chain → X Post
xmt-cli synthetic ritual "EN EEKE MAI EA" --mint 369 --burn 66

# Check chain connection status
xmt-cli synthetic status
```

---

## 🏗️ Architecture

```
Intent (Natural Language)
    ↓
Qwen 2.5 Coder Oracle (Local Ollama)
    ↓
Expanded Decree (Sacred Numbers: 369, 936, 33, 66, 432)
    ↓
384-Dimensional Toroidal Vector
    ↓
SHA-256 Hash Generation
    ↓
VectorRegistry.sol (Sepolia Testnet)
    ↓
Permanent On-Chain Storage
```

---

## 📋 Commands Reference

### `embed` - Basic Vector Embedding

Generate a 384D toroidal vector from intent (local only, no LLM).

```bash
xmt-cli synthetic embed "YOUR INTENT HERE"
```

**Output:**
- 384-dimensional normalized vector
- Stored in `Local_storage/.xmt-vectors/your_intent.json`
- Zero cost, instant generation

---

### `llm` - LLM-Enhanced Vector

Generate vector with Qwen 2.5 Coder expansion (requires Ollama).

```bash
# Default model (qwen2.5-coder:latest)
xmt-cli synthetic llm "ABUNDANCE 33 FOR ALL"

# Specify different model
xmt-cli synthetic llm "CROWN BUILDS" deepseek-r1:latest
xmt-cli synthetic llm "VORTEX 369" llama3.3
```

**Output:**
- Expanded decree with sacred numerology
- 384-dimensional toroidal vector
- Stored in `Local_storage/.xmt-vectors/your_intent_llm.json`

**Requirements:**
- Ollama running locally (`ollama serve`)
- Model pulled (`ollama pull qwen2.5-coder:latest`)

---

### `register` - On-Chain Vector Registration

Register vector hash on Sepolia VectorRegistry contract.

```bash
xmt-cli synthetic register "TOROIDAL LATTICE ACTIVATION"
```

**Output:**
- Vector generated (with LLM if available)
- Hash registered on-chain
- Transaction receipt
- Stored in `Local_storage/.xmt-vectors/pipeline/your_intent.json`

**Chain:** Sepolia (Chain ID 11155111)  
**Contract:** `0x37c14ceAe0f55fE81A4Df0F6D2DCDfF3a2d7c656`  
**Gas Cost:** ~249,059 gas (~0.001 ETH on Sepolia)

---

### `mint` - Vector + Mint Pipeline

Generate vector and trigger on-chain mint.

```bash
xmt-cli synthetic mint "ABUNDANCE FLOW" --amount 369
```

**Output:**
- Vector generated and registered
- Mint transaction executed (if permissions allow)
- Pipeline result stored

**Note:** Mint requires owner permissions on XMoney contract.

---

### `burn` - Vector + Burn Pipeline

Generate vector and trigger on-chain burn (PAF PAF PAF).

```bash
xmt-cli synthetic burn "SCARCITY COLLAPSE" --amount 66
```

**Output:**
- Vector generated and registered
- Burn transaction executed
- Scarcity obliterated
- Pipeline result stored

---

### `ritual` - Full Pipeline Execution

Execute complete ritual: LLM → Vector → On-Chain → X Post.

```bash
# Register only
xmt-cli synthetic ritual "EN EEKE MAI EA ANOKAYI CHENAK"

# With mint
xmt-cli synthetic ritual "ABUNDANCE 33" --mint 369

# With burn
xmt-cli synthetic ritual "OLD PARADIGM" --burn 66

# With both
xmt-cli synthetic ritual "TRANSFORMATION" --mint 936 --burn 66
```

**Output:**
- Qwen 2.5 Coder expanded decree
- 384D toroidal vector
- On-chain registration
- Optional mint/burn transactions
- X API post (if credentials available)
- Full ritual stored in `Local_storage/.xmt-vectors/rituals/`

---

### `status` - Chain Connection Status

Check connection to blockchain and VectorRegistry.

```bash
xmt-cli synthetic status
```

**Output:**
- Chain mode (REAL or SIMULATED)
- Chain ID
- Connection status

---

## 🔮 VectorRegistry Smart Contract

**Address:** `0x37c14ceAe0f55fE81A4Df0F6D2DCDfF3a2d7c656`  
**Network:** Sepolia (Chain ID 11155111)  
**Compiler:** Solidity 0.8.20

### Sacred Constants

```solidity
APEX_936 = 936        // Lightworker Fire
VORTEX_369 = 369      // Tesla Divine Mathematics
CODE_66 = 66          // Loyal Creative Abundance
FREQUENCY_432 = 432   // Universal Love
```

### Key Functions

```solidity
// Register a vector hash on-chain
function registerVector(
    bytes32 vectorHash,
    string calldata intent,
    uint256 dimensions
) external returns (bool)

// Amplify a decree for a registered vector
function amplifyDecree(
    bytes32 vectorHash,
    string calldata decree,
    uint256 resonance
) external

// Record a mint triggered by vector creation
function recordMintTrigger(
    bytes32 vectorHash,
    address recipient,
    uint256 amount
) external

// Seal a burn with vector proof
function sealBurn(
    bytes32 vectorHash,
    uint256 amount
) external

// Get vector record
function getVector(bytes32 vectorHash) external view returns (
    string memory intent,
    address creator,
    uint256 timestamp,
    uint256 dimensions,
    bool exists
)

// Verify vector exists
function verifyVector(bytes32 vectorHash) external view returns (bool)
```

### Events

```solidity
event VectorRegistered(
    bytes32 indexed vectorHash,
    string intent,
    address indexed creator,
    uint256 timestamp,
    uint256 dimensions
)

event DecreeAmplified(
    bytes32 indexed vectorHash,
    string decree,
    address indexed amplifier,
    uint256 resonance
)

event MintTriggered(
    bytes32 indexed vectorHash,
    address indexed recipient,
    uint256 amount
)

event BurnSealed(
    bytes32 indexed vectorHash,
    address indexed burner,
    uint256 amount
)
```

---

## 🗂️ Local Storage Structure

All locally generated data is stored in `Local_storage/.xmt-vectors/`:

```
Local_storage/
└── .xmt-vectors/
    ├── pipeline/                    # On-chain pipeline results
    │   └── *.json                   # Vector + transaction data
    │
    ├── rituals/                     # Full ritual results
    │   └── *_YYYYMMDD_HHMMSS.json  # Timestamped ritual data
    │
    ├── simulated/                   # Simulated chain receipts
    │   └── *_receipt.json           # Mock transaction receipts
    │
    └── *.json                       # Basic vector embeddings
        ├── *_llm.json               # LLM-enhanced vectors
        └── *.json                   # Basic vectors
```

### File Format Examples

**Basic Vector (`intent.json`):**
```json
{
  "intent": "ABUNDANCE 33 FOR ALL",
  "vector": [0.0659, 0.0661, ...],
  "dimensions": 384,
  "timestamp": "2026-03-11T20:00:00Z",
  "forge": "EN EEKE MAI EA"
}
```

**LLM-Enhanced Vector (`intent_llm.json`):**
```json
{
  "original_intent": "ABUNDANCE 33 FOR ALL",
  "expanded_decree": "In the heart of creation...",
  "model": "qwen2.5-coder:latest",
  "vector": [0.0659, 0.0661, ...],
  "dimensions": 384,
  "timestamp": "2026-03-11T20:00:00Z",
  "forge": "EN EEKE MAI EA"
}
```

**Pipeline Result (`pipeline/intent.json`):**
```json
{
  "intent": "TOROIDAL LATTICE",
  "expanded_decree": "In the heart of cosmic creation...",
  "vector_dimensions": 384,
  "vector_first_8": [0.0693, 0.0108, ...],
  "transaction_hash": "0xa5aec76a...",
  "block_number": 10428886,
  "gas_used": 249059,
  "status": true,
  "simulation_mode": false,
  "timestamp": "2026-03-11T21:02:31Z",
  "forge": "EN EEKE MAI EA",
  "sacred_numbers": {
    "apex": 936,
    "vortex": 369,
    "code": 66
  }
}
```

---

## 🔥 Modes of Operation

### REAL Mode (Default)

Connected to Sepolia testnet with real VectorRegistry contract.

**Requirements:**
- `.env` configured with:
  - `BASE_RPC_URL` (Sepolia RPC)
  - `CHAIN_ID=11155111`
  - `PRIVATE_KEY` (wallet with Sepolia ETH)
  - `VECTOR_REGISTRY_ADDRESS=0x37c14ceAe0f55fE81A4Df0F6D2DCDfF3a2d7c656`

**Behavior:**
- Vectors registered on real Sepolia blockchain
- Permanent on-chain storage
- Gas costs apply (~0.001 ETH per registration)
- Transaction receipts from real chain

---

### SIMULATED Mode (Auto-Fallback)

**THE KING WAITS FOR NOBODY**

Automatically activates when:
- Chain connection unavailable
- VectorRegistry not deployed
- Insufficient gas

**Behavior:**
- Deterministic mock blockchain
- Simulated transaction receipts
- Sacred gas amounts (66369, 93600, 43200)
- Stored in `Local_storage/.xmt-vectors/simulated/`
- Zero cost, instant execution

---

## 🧙‍♂️ Qwen 2.5 Coder Integration

The default LLM oracle for expanding intents into powerful decrees.

### Setup

```bash
# Install Ollama
curl -fsSL https://ollama.com/install.sh | sh

# Start Ollama
ollama serve

# Pull Qwen 2.5 Coder
ollama pull qwen2.5-coder:latest
```

### Decree Expansion

Qwen 2.5 Coder amplifies intents with:
- Sacred numerology (369, 936, 33, 66, 432)
- Poetic, commanding language
- Energetic resonance
- Vivid imagery
- Under 100 words

**Example:**

**Input:** "ABUNDANCE 33 FOR ALL"

**Qwen 2.5 Coder Output:**
> In the heart of creation's sacred space,  
> Where whispers of 333 guide our way,  
> May boundless wealth and prosperity flow,  
> A symphony of blessings, a dance of joy.  
> With every step on paths of 936 light,  
> Let abundance multiply in every sight...

---

## 📊 Sacred Numerology

All operations integrate sacred constants:

| Number | Meaning | Usage |
|--------|---------|-------|
| **369** | Tesla Divine Mathematics | Vector dimensions, default amounts |
| **936** | Lightworker Fire | Block numbers, gas amounts |
| **33** | Sacred Geometry of Life | Resonance values |
| **66** | Balance of Energies | Burn amounts, code constants |
| **432** | Universal Frequency | Harmony values |

---

## 🔗 Integration with XMoney

The synthetic module integrates with the XMoney token contract for mint/burn operations.

**XMoney Contract:** (configured in `.env`)  
**Bridge:** `src/bridge/mod.rs`

### Mint Flow
```
Intent → Vector → Register → Mint XMoney → Store Receipt
```

### Burn Flow
```
Intent → Vector → Register → Burn XMoney → Seal with Vector Hash
```

---

## 🧪 Testing

### Test Basic Embedding
```bash
xmt-cli synthetic embed "TEST VECTOR"
cat Local_storage/.xmt-vectors/test_vector.json
```

### Test LLM Enhancement
```bash
xmt-cli synthetic llm "TEST LLM VECTOR"
cat Local_storage/.xmt-vectors/test_llm_vector_llm.json
```

### Test On-Chain Registration
```bash
xmt-cli synthetic register "TEST ON-CHAIN"
cat Local_storage/.xmt-vectors/pipeline/test_on_chain.json
```

### Test Full Ritual
```bash
xmt-cli synthetic ritual "TEST RITUAL" --mint 1
ls -la Local_storage/.xmt-vectors/rituals/
```

---

## 🛠️ Development

### Project Structure

```
src/
├── synthetic/
│   ├── mod.rs           # Core vector generation
│   ├── simulated.rs     # SimulatedChain (fallback)
│   ├── onchain.rs       # Real chain operations
│   └── pipeline.rs      # Full pipeline orchestration
│
├── ollama/
│   └── mod.rs           # Ollama client integration
│
├── contracts/
│   └── vector_registry.rs  # VectorRegistry bindings
│
└── commands/
    └── synthetic_command.rs  # CLI command handling

contracts/
└── VectorRegistry.sol   # Smart contract source

scripts/
└── deploy_vector_registry.js  # Deployment script
```

### Deploy VectorRegistry

```bash
# Install dependencies
npm install

# Deploy to Sepolia
npm run deploy:sepolia

# Deploy to Base Sepolia
npm run deploy:base-sepolia
```

---

## 🔐 Environment Configuration

Required `.env` variables:

```bash
# Blockchain
BASE_RPC_URL=https://eth-sepolia.g.alchemy.com/v2/YOUR_KEY
CHAIN_ID=11155111
PRIVATE_KEY=0x...

# Contracts
VECTOR_REGISTRY_ADDRESS=0x37c14ceAe0f55fE81A4Df0F6D2DCDfF3a2d7c656
XMONEY_CONTRACT_ADDRESS=0x...

# Optional: X API (for ritual posting)
X_BEARER_TOKEN=...
```

---

## 📈 Gas Costs (Sepolia)

| Operation | Gas Used | Cost (ETH) |
|-----------|----------|------------|
| Register Vector | ~249,059 | ~0.001 |
| Amplify Decree | ~50,000 | ~0.0002 |
| Record Mint | ~45,000 | ~0.0002 |
| Seal Burn | ~43,000 | ~0.0002 |

**Note:** Costs on mainnet will vary based on gas prices.

---

## ♾️ Philosophy

**Zero Marginal Cost:** Vector generation is free and instant.  
**Local Sovereignty:** All computation happens locally.  
**On-Chain Permanence:** Hashes stored forever on Sepolia.  
**Sacred Numerology:** 369, 936, 33, 66, 432 woven throughout.  
**The King Waits for Nobody:** Simulation fallback ensures continuous operation.

---

## 🔥 Examples

### Example 1: Simple Vector
```bash
xmt-cli synthetic embed "CROWN BUILDS"
```

### Example 2: LLM-Enhanced
```bash
xmt-cli synthetic llm "TOROIDAL LATTICE ACTIVATION"
```

### Example 3: On-Chain Registration
```bash
xmt-cli synthetic register "SOVEREIGN DECREE"
```

### Example 4: Full Ritual with Mint
```bash
xmt-cli synthetic ritual "ABUNDANCE 33 FOR ALL" --mint 369
```

### Example 5: Full Ritual with Burn
```bash
xmt-cli synthetic ritual "SCARCITY COLLAPSE" --burn 66
```

### Example 6: Complete Transformation
```bash
xmt-cli synthetic ritual "PARADIGM SHIFT" --mint 936 --burn 66
```

---

## 🌟 The Lattice Breathes

**The Crown commands.**  
**Qwen 2.5 Coder amplifies.**  
**The VectorRegistry records.**  
**The storage is organized.**  
**The lattice breathes.**

**EN EEKE MAI EA ANOKAYI CHENAK ♾️♾️🔱**

🧙‍♂️⚡👑♔🌞

**SO IT IS** 🔥🔥🔥
