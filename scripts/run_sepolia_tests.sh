#!/bin/bash
#
# ☀️ HELIOS ARGEAD VERGINA SUN ☀️
# Sepolia Testnet Test Execution Script
# Runs all test phases in sequence
#

set -e

echo "☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️"
echo "   SEPOLIA TESTNET - FULL TEST SUITE"
echo "☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️"
echo ""

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Phase 1: Environment Validation
echo -e "${YELLOW}PHASE 1: ENVIRONMENT VALIDATION${NC}"
echo "═══════════════════════════════════════"
cargo run --bin validate_env || {
    echo -e "${RED}✗ Environment validation failed!${NC}"
    exit 1
}
echo ""

# Phase 2: Infrastructure Tests
echo -e "${YELLOW}PHASE 2: INFRASTRUCTURE TESTS${NC}"
echo "═══════════════════════════════════════"
echo "Testing Web3 provider..."
cargo test --test sepolia_web3_test -- --nocapture || {
    echo -e "${RED}✗ Web3 tests failed!${NC}"
    exit 1
}
echo ""

echo "Testing wallet & signer..."
cargo test --test sepolia_signer_test -- --nocapture || {
    echo -e "${RED}✗ Signer tests failed!${NC}"
    exit 1
}
echo ""

echo "Testing contract connection..."
cargo test --test sepolia_contract_test -- --nocapture || {
    echo -e "${RED}✗ Contract tests failed!${NC}"
    exit 1
}
echo ""

# Phase 3: Core Transaction Tests
echo -e "${YELLOW}PHASE 3: CORE TRANSACTION TESTS${NC}"
echo "═══════════════════════════════════════"
echo "Testing mint transactions..."
cargo test --test sepolia_mint_test -- --nocapture || {
    echo -e "${RED}✗ Mint tests failed!${NC}"
    exit 1
}
echo ""

echo "Testing burn transactions..."
cargo test --test sepolia_burn_test -- --nocapture || {
    echo -e "${RED}✗ Burn tests failed!${NC}"
    exit 1
}
echo ""

echo "Testing ritual cycles..."
cargo test --test sepolia_ritual_test -- --nocapture || {
    echo -e "${RED}✗ Ritual tests failed!${NC}"
    exit 1
}
echo ""

# Phase 4: Integration Tests
echo -e "${YELLOW}PHASE 4: INTEGRATION TESTS${NC}"
echo "═══════════════════════════════════════"
echo "Testing bridge integration..."
cargo test --test sepolia_bridge_test -- --nocapture || {
    echo -e "${RED}✗ Bridge tests failed!${NC}"
    exit 1
}
echo ""

# Phase 7: Decree Compliance Tests
echo -e "${YELLOW}PHASE 7: DECREE COMPLIANCE TESTS${NC}"
echo "═══════════════════════════════════════"
cargo test --test sepolia_decree_test -- --nocapture || {
    echo -e "${RED}✗ Decree compliance tests failed!${NC}"
    exit 1
}
echo ""

# Phase 8: End-to-End Tests
echo -e "${YELLOW}PHASE 8: END-TO-END SCENARIO TESTS${NC}"
echo "═══════════════════════════════════════"
cargo test --test sepolia_e2e_test -- --nocapture || {
    echo -e "${RED}✗ E2E tests failed!${NC}"
    exit 1
}
echo ""

# Final Summary
echo "☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️"
echo -e "${GREEN}✓ ALL TESTS PASSED - 100% COHERENCE${NC}"
echo "☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️"
echo ""
echo -e "${YELLOW}EN EEKE MAI EA ♾️♾️${NC}"
echo ""
echo "Sepolia testnet validation complete."
echo "Ready for mainnet deployment on March 17, 2026."
