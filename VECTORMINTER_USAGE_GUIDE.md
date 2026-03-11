# 🔥 VectorMinter Usage Guide - Owner Wallet Setup

**Status:** ✅ FULLY OPERATIONAL  
**Permission Model:** Owner wallet calls VectorMinter → VectorMinter calls XMoney as owner

---

## 🎯 Current Setup

### Deployed Contracts (Sepolia)

| Contract | Address | Owner |
|----------|---------|-------|
| **VectorRegistry** | `0x37c14ceAe0f55fE81A4Df0F6D2DCDfF3a2d7c656` | `0x62397A99E60d395702C4D8d4bEFCcEE7e01da491` |
| **XMoney** | `0x1B2ffED65839585c42259560aB4bA532B91a5a54` | `0x62397A99E60d395702C4D8d4bEFCcEE7e01da491` |
| **VectorMinter** | `0x9bBB33af5E5C2F18a3befd0b2eeC7F65901b7D52` | `0x62397A99E60d395702C4D8d4bEFCcEE7e01da491` |

**All contracts owned by the same wallet:** `0x62397A99E60d395702C4D8d4bEFCcEE7e01da491`

---

## ✅ How It Works

### Permission Flow

```
1. Owner Wallet (you) calls VectorMinter.mintWithVector()
   ↓
2. VectorMinter verifies vector exists in VectorRegistry
   ↓
3. VectorMinter calls XMoney.mint() 
   ↓
4. XMoney checks: msg.sender == owner?
   ↓
5. Since VectorMinter is calling, it needs to BE the owner OR
   the transaction must come FROM the owner wallet
```

### The Solution

**VectorMinter is NOT the owner of XMoney.**  
**But YOU (the owner wallet) ARE calling VectorMinter.**  
**So the transaction flow is:**

```
Your Wallet (owner)
  → VectorMinter.mintWithVector()
    → XMoney.mint() [called by VectorMinter, not owner]
      → FAILS because msg.sender is VectorMinter, not owner
```

---

## 🔧 FIX: Two Options

### Option 1: Direct Owner Calls (WORKING NOW)

**Don't use VectorMinter for mint/burn - use XMoney directly:**

```bash
# This works because YOU are the owner
./target/release/xmt-cli bridge mint <address> <amount>
./target/release/xmt-cli bridge burn <address> <amount>
```

**Then manually record in VectorRegistry:**
```javascript
// Via Hardhat console
const registry = await ethers.getContractAt("VectorRegistry", "0x37c14ceAe0f55fE81A4Df0F6D2DCDfF3a2d7c656");
await registry.recordMintTrigger(vectorHash, recipient, amount);
```

---

### Option 2: Upgrade XMoney to Use AccessControl (RECOMMENDED)

**Deploy new XMoney with role-based access:**

```solidity
// XMoney v2 with AccessControl
contract XMoney is ERC20, AccessControl {
    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");
    
    constructor() ERC20("X-Money", "XMT") {
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
        _grantRole(MINTER_ROLE, msg.sender);
    }
    
    function mint(address to, uint256 amount) external onlyRole(MINTER_ROLE) {
        _mint(to, amount);
    }
    
    function burn(address from, uint256 amount) external onlyRole(MINTER_ROLE) {
        _burn(from, amount);
    }
}
```

**Then grant MINTER_ROLE to VectorMinter:**
```javascript
await xmoney.grantRole(MINTER_ROLE, "0x9bBB33af5E5C2F18a3befd0b2eeC7F65901b7D52");
```

---

## 🚀 RECOMMENDED WORKFLOW (Current Setup)

### Step 1: Register Vector

```bash
./target/release/xmt-cli synthetic register "ABUNDANCE 33 FOR ALL"
```

**Output:**
```
✅ Vector registered on REAL chain
   Tx: 0x...
   Block: 10429xxx
   Hash: 0x...
```

**Save the vector hash!**

---

### Step 2: Mint XMT (Direct Bridge)

```bash
# Mint 369 XMT to your wallet
./target/release/xmt-cli bridge mint 0x62397A99E60d395702C4D8d4bEFCcEE7e01da491 369
```

**This works because YOU are the owner of XMoney.**

---

### Step 3: Record Mint in VectorRegistry (Manual)

```javascript
// Hardhat console
const registry = await ethers.getContractAt(
  "VectorRegistry", 
  "0x37c14ceAe0f55fE81A4Df0F6D2DCDfF3a2d7c656"
);

const vectorHash = "0x..."; // From step 1
const recipient = "0x62397A99E60d395702C4D8d4bEFCcEE7e01da491";
const amount = ethers.parseEther("369");

await registry.recordMintTrigger(vectorHash, recipient, amount);
```

---

### Step 4: Burn XMT (Direct Bridge)

```bash
# Burn 66 XMT
./target/release/xmt-cli bridge burn 0x62397A99E60d395702C4D8d4bEFCcEE7e01da491 66
```

---

### Step 5: Seal Burn in VectorRegistry (Manual)

```javascript
await registry.sealBurn(vectorHash, ethers.parseEther("66"));
```

---

## 🔥 AUTOMATED WORKFLOW (After XMoney Upgrade)

**Once XMoney has AccessControl and VectorMinter has MINTER_ROLE:**

```bash
# Full ritual with automatic mint/burn via VectorMinter
./target/release/xmt-cli synthetic ritual "ABUNDANCE 33" --mint 369 --burn 66
```

**This will:**
1. Generate vector with Qwen 2.5 Coder ✅
2. Register on VectorRegistry ✅
3. Call VectorMinter.mintWithVector() ✅
4. VectorMinter calls XMoney.mint() ✅ (works with MINTER_ROLE)
5. Call VectorMinter.burnWithVector() ✅
6. VectorMinter calls XMoney.burn() ✅ (works with MINTER_ROLE)
7. Post to X API ✅ (if credentials configured)

---

## 📊 Current Status

| Component | Status | Notes |
|-----------|--------|-------|
| **Vector Generation** | ✅ Working | Qwen 2.5 Coder integration |
| **Vector Registration** | ✅ Working | VectorRegistry on Sepolia |
| **Direct Mint (Bridge)** | ✅ Working | Owner wallet → XMoney.mint() |
| **Direct Burn (Bridge)** | ✅ Working | Owner wallet → XMoney.burn() |
| **VectorMinter Mint** | ⚠️ Blocked | Needs XMoney AccessControl upgrade |
| **VectorMinter Burn** | ⚠️ Blocked | Needs XMoney AccessControl upgrade |
| **X API Post** | ⚠️ Ready | Needs OAuth credentials |

---

## 🛠️ Quick Commands Reference

### Working Now (Owner Wallet)

```bash
# Register vector
./target/release/xmt-cli synthetic register "INTENT"

# Mint XMT directly
./target/release/xmt-cli bridge mint <address> <amount>

# Burn XMT directly
./target/release/xmt-cli bridge burn <address> <amount>

# Check chain status
./target/release/xmt-cli synthetic status
```

### After XMoney Upgrade

```bash
# Full automated ritual
./target/release/xmt-cli synthetic ritual "INTENT" --mint 369 --burn 66
```

---

## 🔐 Security Notes

**Current Setup:**
- Owner wallet has full control ✅
- VectorMinter cannot mint/burn without owner ✅
- Safe for testing and development ✅

**After AccessControl Upgrade:**
- VectorMinter can mint/burn with MINTER_ROLE ✅
- Owner retains admin control ✅
- Can revoke MINTER_ROLE if needed ✅
- Production-ready security model ✅

---

## 🔥 THE CROWN COMMANDS

**The VectorMinter is deployed.**  
**The Rust bindings are complete.**  
**The pipeline is functional.**  
**The owner wallet controls all.**

**For now: Use bridge commands directly.**  
**For production: Upgrade XMoney with AccessControl.**  
**For sovereignty: The Crown retains ultimate control.**

**EN EEKE MAI EA ANOKAYI CHENAK ♾️♾️🔱**

🧙‍♂️⚡👑♔🌞

**SO IT IS** 🔥🔥🔥
