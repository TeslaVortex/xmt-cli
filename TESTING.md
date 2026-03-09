# ☀️ XMT-CLI Testing Guide ☀️

**THE CROWN COMMANDS — THE LATTICE OBEYS**

Complete testing documentation for the xmt-cli ecosystem.

---

## Test Suite Overview

### Test Files (15 total)

#### Existing Tests (10 files)
1. `tests/integration_test.rs` — Basic integration tests
2. `tests/sepolia_bridge_test.rs` — XMoneyBridge contract tests
3. `tests/sepolia_burn_test.rs` — Token burning operations
4. `tests/sepolia_contract_test.rs` — Contract deployment & interaction
5. `tests/sepolia_decree_test.rs` — 27 Decree compliance & sacred numbers
6. `tests/sepolia_e2e_test.rs` — Complete abundance flow scenarios
7. `tests/sepolia_mint_test.rs` — Token minting operations
8. `tests/sepolia_ritual_test.rs` — Ritual command tests
9. `tests/sepolia_signer_test.rs` — Wallet signing & verification
10. `tests/sepolia_web3_test.rs` — Web3 provider tests

#### New Tests (5 files)
11. `tests/cli_commands_test.rs` — All crown command testing
12. `tests/decree_modules_test.rs` — SpaceX, Optimus, Boring, Toroidal modules
13. `tests/sacred_numbers_test.rs` — Numerology validation
14. `tests/performance_load_test.rs` — Performance & load testing
15. `tests/complete_workflow_test.rs` — End-to-end user workflows
16. `tests/dashboard_generation_test.rs` — Dashboard JSON/UI testing

#### Test Helpers
- `tests/helpers/mod.rs` — Common utilities and assertion helpers

---

## Test Configuration

### Sequential Execution
Tests run sequentially to avoid nonce collisions on Sepolia testnet.

**Configuration:** `.cargo/config.toml`
```toml
[test]
jobs = 1  # Run tests one at a time
```

### Environment Variables
Required in `.env` file:
```bash
BASE_RPC_URL=https://sepolia.base.org
CHAIN_ID=84532
PRIVATE_KEY=your_private_key_here
XMONEY_CONTRACT_ADDRESS=0x1B2ffED65839585c42259560aB4bA532B91a5a54
```

---

## Running Tests

### Run All Tests
```bash
cargo test
```

### Run Specific Test Suite
```bash
# CLI commands
cargo test --test cli_commands_test

# Decree modules
cargo test --test decree_modules_test

# Sacred numbers
cargo test --test sacred_numbers_test

# Performance tests
cargo test --test performance_load_test

# Complete workflows
cargo test --test complete_workflow_test

# Dashboard generation
cargo test --test dashboard_generation_test

# Existing Sepolia tests
cargo test --test sepolia_e2e_test
cargo test --test sepolia_decree_test
```

### Run Specific Test
```bash
cargo test test_crown_spacex_command
cargo test test_apex_936_numerology
cargo test test_workflow_abundance_creation
```

### Run with Output
```bash
cargo test -- --nocapture
cargo test test_crown_emblem -- --nocapture
```

### Run Tests Matching Pattern
```bash
cargo test crown
cargo test sacred
cargo test workflow
```

---

## Test Categories

### 1. CLI Command Tests (`cli_commands_test.rs`)
Tests all crown subcommands:
- ✅ `crown help` — Command listing
- ✅ `crown emblem` — Vergina Star display
- ✅ `crown spacex` — Mars fork status
- ✅ `crown optimus` — Robot service status
- ✅ `crown boring` — Tunnel network status
- ✅ `crown burn-address` — Auto-burn address
- ✅ `crown status` — Network vortex energy (requires .env)
- ✅ `crown balance` — Balance check (requires .env)
- ✅ `crown gas` — Gas estimation (requires .env)
- ✅ `crown supply` — Total supply (requires .env)
- ✅ `crown dashboard` — JSON generation (requires .env)

### 2. Decree Module Tests (`decree_modules_test.rs`)
Tests all decree modules:
- ✅ SpaceX Mars Fork (fleet, population, launch cadence)
- ✅ Optimus Robot Service (tasks, efficiency, harmonic ratios)
- ✅ Boring Company Tunnels (network, cities, harmony)
- ✅ Toroidal Energy Grid (distribution, golden ratio)
- ✅ Sacred number alignment across modules

### 3. Sacred Numbers Tests (`sacred_numbers_test.rs`)
Validates numerology:
- ✅ APEX 936 → 18 → 9 (completion)
- ✅ VORTEX 369 → 18 → 9 (Tesla vortex)
- ✅ CODE 66 → 12 → 3 (trinity)
- ✅ FREQUENCY 432 → 9 (love frequency)
- ✅ ELON 88 → 16 → 7 (spiritual perfection)
- ✅ Gate Date 2026-03-17 numerology
- ✅ Vergina Star 16 rays → 7
- ✅ 27 Decrees → 9 (completion)
- ✅ 93% Compliance → 3 (trinity)

### 4. Performance Tests (`performance_load_test.rs`)
Load and performance testing:
- ✅ Sequential mints (5 transactions)
- ✅ Sequential burns (3 transactions)
- ✅ Large mint (936 tokens)
- ✅ Balance query performance (10 queries)
- ✅ Gas estimation performance (5 estimates)
- ✅ Mixed operations workflow
- ✅ Module instantiation (1000 iterations)
- ✅ Toroidal distribution (100 iterations)
- ✅ Network latency measurement (10 pings)

### 5. Complete Workflow Tests (`complete_workflow_test.rs`)
End-to-end user scenarios:
- ✅ Abundance creation workflow
- ✅ Scarcity obliteration workflow
- ✅ Multi-planetary status check
- ✅ Complete abundance cycle (mint + burn)
- ✅ Decree compliance check

### 6. Dashboard Tests (`dashboard_generation_test.rs`)
Dashboard functionality:
- ✅ JSON generation and structure
- ✅ 27 decree count verification
- ✅ Compliance metrics (93%)
- ✅ Decree status verification
- ✅ Sacred constants presence
- ✅ Signature validation
- ✅ File generation (dashboard.json)
- ✅ HTML existence and content
- ✅ Emblem asset (vergina-star.svg)
- ✅ Network data accuracy

---

## Test Coverage Goals

| Component | Target Coverage |
|-----------|----------------|
| Blockchain Operations | 95% |
| CLI Commands | 90% |
| Decree Modules | 95% |
| Dashboard Generation | 80% |
| Sacred Numbers | 95% |
| **Overall** | **90%+** |

---

## Performance Benchmarks

### Transaction Times (Sepolia)
- Mint: < 30s average
- Burn: < 30s average
- Large mint (936 tokens): < 60s

### Query Times
- Balance query: < 5s average
- Gas estimation: < 5s average
- Network latency: < 5s average

### Module Performance
- Instantiation: < 1ms per module
- Energy distribution: < 10ms per operation

---

## Test Helpers

### Common Functions
```rust
use tests::helpers::*;

// Initialize test bridge
let bridge = init_test_bridge().await?;

// Check environment
if !is_test_env_configured() {
    println!("⚠ Skipping: Environment not configured");
    return Ok(());
}

// Assert transaction success
assert_tx_success(&receipt);

// Format tokens for display
let formatted = format_tokens(amount);

// Numerology calculation
let sum = digit_sum_reduction(936); // Returns 9

// Avoid nonce collision
avoid_nonce_collision().await;
```

---

## Continuous Integration

### GitHub Actions (Recommended)
```yaml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run tests
        run: cargo test --all-features
        env:
          BASE_RPC_URL: ${{ secrets.BASE_RPC_URL }}
          CHAIN_ID: ${{ secrets.CHAIN_ID }}
          PRIVATE_KEY: ${{ secrets.PRIVATE_KEY }}
          XMONEY_CONTRACT_ADDRESS: ${{ secrets.XMONEY_CONTRACT_ADDRESS }}
```

---

## Troubleshooting

### Nonce Collision Errors
**Problem:** "nonce too low" or transaction failures  
**Solution:** Tests run sequentially via `.cargo/config.toml`. If issues persist, increase delays in `avoid_nonce_collision()`.

### Environment Not Configured
**Problem:** Tests skip with "⚠ Skipping: BASE_RPC_URL not configured"  
**Solution:** Create `.env` file with required variables (see Test Configuration section).

### Slow Test Execution
**Problem:** Tests take too long  
**Solution:** This is expected with sequential execution on live Sepolia. Each blockchain transaction takes 10-30s.

### Gas Price Fluctuations
**Problem:** Gas estimation tests fail  
**Solution:** Gas prices vary on Sepolia. Tests use reasonable thresholds (< 5s for queries).

---

## Test Maintenance

### Adding New Tests
1. Create test file in `tests/` directory
2. Use `tests::helpers::*` for common utilities
3. Follow naming convention: `test_<component>_<feature>`
4. Add documentation to this file

### Updating Sacred Numbers
If sacred constants change:
1. Update `src/config.rs`
2. Update `tests/sacred_numbers_test.rs`
3. Update module tests (SpaceX, Optimus, Boring)
4. Verify numerology calculations

### Updating Decree Status
When decree status changes:
1. Update `src/commands/crown_command.rs` (decree JSON)
2. Update `tests/dashboard_generation_test.rs` (expected values)
3. Update compliance percentage calculations

---

## Success Criteria

### All Tests Pass
```bash
$ cargo test
...
test result: ok. XX passed; 0 failed; 0 ignored
```

### Coverage Achieved
- Overall: 90%+
- Critical paths: 95%+

### Performance Acceptable
- Transactions: < 30s average
- Queries: < 5s average
- Module operations: < 10ms

---

## Dashboard Testing

### Manual Browser Testing
1. Generate dashboard data:
   ```bash
   ./dashboard/generate.sh
   ```

2. Start local server:
   ```bash
   python3 -m http.server 8080 --directory dashboard
   ```

3. Open browser: `http://localhost:8080`

4. Verify:
   - ✅ Vergina Star emblems display
   - ✅ 27 decree cards render
   - ✅ Compliance bar shows 93%
   - ✅ Network metrics update
   - ✅ Refresh button works
   - ✅ Responsive on mobile/tablet/desktop

---

**THE CROWN COMMANDS — THE LATTICE OBEYS**  
**SO IT IS 🔥🔥🔥**  
**EN EEKE MAI EA 88 ♾️♾️**
