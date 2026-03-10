# 🔥 XMT-CLI TESTNET INTERACTIVE RUNNER 🔥

**Interactive menu-driven interface for all XMT-CLI commands on Sepolia testnet**

---

## 🚀 QUICK START

```bash
./run_testnet_cli.sh
```

This will:
1. ✅ Check if CLI binary exists (builds if needed)
2. ✅ Validate `.env` file
3. ✅ Verify environment variables
4. ✅ Launch interactive menu

---

## 📋 MENU STRUCTURE

### 🔮 RITUAL & CORE
- **Run 936 Apex Ritual** - Execute the sacred DTQPE + PQC + Toroidal ritual
- **Timeline Sync** - Build project + run ritual (calls `sync.sh`)

### 👑 CROWN COMMANDS (Status & Info)
- **Crown Status** - Network status, block number, chain ID
- **Wallet Balance** - Check your XMT token balance
- **Gas Price** - Current gas price in gwei
- **Total Supply** - Total XMT token supply
- **Generate Dashboard Data** - Create `dashboard/dashboard.json`
- **Integration Status** - View all integration layers

### ⚡ BLOCKCHAIN OPERATIONS
- **Mint Tokens** - Mint XMT tokens (369, 66, 936, or custom)
- **Burn Tokens** - Burn XMT tokens (369, 66, 936, or custom)
- **Abundance Drop** - Automated abundance distribution

### 𝕏 X API & GROK ORACLE
- **X API Operations** - Post tweets, search, get user info, find triggers
- **Grok Oracle** - Chat, verify rituals, calculate prices, list models

### 🧪 TESTING & DASHBOARD
- **Run Tests** - Quick smoke test, full suite, or specific test categories
- **Open Dashboard** - Start HTTP server and open dashboard in browser

### 🔧 SYSTEM
- **Rebuild CLI** - Run `cargo build --release`
- **View Logs** - Show last 50 lines of logs
- **Exit** - Quit the interactive runner

---

## 🎯 FEATURES

### ✅ **Auto-Build**
If the CLI binary doesn't exist, it automatically runs `cargo build --release`

### ✅ **Environment Validation**
Checks for required environment variables:
- `SEPOLIA_RPC_URL`
- `PRIVATE_KEY`
- `CONTRACT_ADDRESS`

### ✅ **Sacred Number Presets**
Quick access to sacred amounts:
- **369** - Vortex alignment
- **66** - Code 66 harmonic
- **936** - Apex power

### ✅ **Color-Coded Output**
- 🟢 Green - Success messages
- 🔴 Red - Errors
- 🟡 Yellow - Warnings
- 🔵 Blue - Info
- 🟣 Purple - Section headers
- 🟡 Gold - Main headers

### ✅ **Dashboard Integration**
Automatically starts HTTP server on port 8936 and opens browser

### ✅ **Test Suite Access**
Run individual test categories or full suite:
- Smoke tests
- Mint tests
- Burn tests
- Integration tests
- Full Sepolia suite

---

## 📖 USAGE EXAMPLES

### Example 1: Mint 369 Tokens
```
1. Launch: ./run_testnet_cli.sh
2. Select: 9 (Mint Tokens)
3. Choose: 1 (369 tokens - Vortex)
4. Confirm transaction
```

### Example 2: Check Status & Balance
```
1. Launch: ./run_testnet_cli.sh
2. Select: 3 (Crown Status)
3. View network info
4. Select: 4 (Wallet Balance)
5. View XMT balance
```

### Example 3: Run Ritual & Generate Dashboard
```
1. Launch: ./run_testnet_cli.sh
2. Select: 1 (Run 936 Apex Ritual)
3. Watch DTQPE + PQC + Toroidal execution
4. Select: 7 (Generate Dashboard Data)
5. Select: 15 (Open Dashboard)
```

### Example 4: X API Abundance Search
```
1. Launch: ./run_testnet_cli.sh
2. Select: 12 (X API Operations)
3. Choose: 4 (Search abundance triggers)
4. View "EN EEKE MAI EA" mentions
```

### Example 5: Run Full Test Suite
```
1. Launch: ./run_testnet_cli.sh
2. Select: 14 (Run Tests)
3. Choose: 2 (Full test suite)
4. Watch all Sepolia tests execute
```

---

## 🔧 REQUIREMENTS

### Environment Variables (.env)
```bash
# Network
SEPOLIA_RPC_URL=https://sepolia.infura.io/v3/YOUR_KEY
CHAIN_ID=11155111

# Wallet
PRIVATE_KEY=0x...

# Contract
CONTRACT_ADDRESS=0x1B2ffED65839585c42259560aB4bA532B91a5a54

# X API (optional)
X_API_KEY=...
X_API_SECRET=...
X_ACCESS_TOKEN=...
X_ACCESS_SECRET=...

# Grok Oracle (optional)
XAI_API_KEY=...
```

### System Dependencies
- Rust toolchain (cargo)
- Python 3 (for dashboard server)
- curl (for testing)
- Standard Unix tools (grep, awk, etc.)

---

## 🎨 MENU NAVIGATION

**Number Selection:**
- Type the number of your choice
- Press Enter to execute

**Sacred Number Quick Access:**
- Mint/Burn menus offer 369, 66, 936 presets
- Option 4 allows custom amounts

**Continuous Operation:**
- After each command, press Enter to return to menu
- Menu loops until you select 0 (Exit)

---

## 🔍 TROUBLESHOOTING

### "CLI binary not found"
**Solution:** Script will auto-build. If build fails, check Rust installation.

### ".env file not found"
**Solution:** Copy `.env.example` to `.env` and fill in your values.

### "Missing required environment variables"
**Solution:** Check that SEPOLIA_RPC_URL, PRIVATE_KEY, and CONTRACT_ADDRESS are set.

### "Build failed"
**Solution:** Run `cargo build --release` manually to see detailed errors.

### Dashboard won't open
**Solution:** Manually visit `http://localhost:8936` in your browser.

---

## 📊 COMMAND REFERENCE

| Menu # | Command | Description |
|--------|---------|-------------|
| 1 | `ritual` | Run 936 Apex Ritual |
| 2 | `sync.sh` | Timeline sync (build + ritual) |
| 3 | `crown status` | Network status |
| 4 | `crown balance` | Wallet balance |
| 5 | `crown gas` | Gas price |
| 6 | `crown supply` | Total supply |
| 7 | `crown dashboard` | Generate dashboard.json |
| 8 | `crown integrations` | Integration status |
| 9 | `mint --amount <N>` | Mint tokens |
| 10 | `burn --amount <N>` | Burn tokens |
| 11 | `abundance <cmd>` | Abundance operations |
| 12 | `xapi <cmd>` | X API operations |
| 13 | `xapi grok-<cmd>` | Grok Oracle |
| 14 | `cargo test` | Run tests |
| 15 | Dashboard server | Open dashboard |
| 16 | `cargo build --release` | Rebuild CLI |
| 17 | View logs | Show recent logs |

---

## 🌟 SACRED NUMBERS

The script provides quick access to sacred numbers:

- **936** - APEX_936 - Apex power, maximum alignment
- **369** - VORTEX_369 - Tesla vortex mathematics
- **66** - CODE_66 - Harmonic blessing code
- **432** - FREQUENCY_432 - Sacred frequency
- **88** - ELON_88 - Infinite power fuel

---

## 🔐 SECURITY NOTES

- Never commit `.env` file to git
- Keep `PRIVATE_KEY` secure
- Use testnet only for development
- Verify contract address before transactions

---

## 📝 LOGS

Logs are written to `xmt-cli.log` (if configured).

View recent logs via menu option 17 or:
```bash
tail -f xmt-cli.log
```

---

## 🎯 EXIT

To exit the interactive runner:
- Select option **0**
- Or press **Ctrl+C**

---

**EN EEKE MAI EA ♾️♾️**  
**THE CROWN COMMANDS - THE LATTICE OBEYS**  
**SO IT IS 🔥🔥🔥**
