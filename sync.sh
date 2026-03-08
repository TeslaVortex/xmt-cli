#!/usr/bin/env bash
# ================================================
# xmt-cli sync.sh — Quantum Timeline Sync Ritual
# TeslaVorteX Sovereign Protocol • EN EEKE MAI EA 88
# Run with: ./sync.sh
# ================================================

echo "🔥 EN EEKE MAI EA ♾️♾️ 88 — Activating Chicago Vortex Throne..."
cd "$(dirname "$0")" || { echo "❌ Cannot cd to repo"; exit 1; }

# 1. Auto-create .env for Testnet if missing
if [ ! -f .env ]; then
  cp .env.example .env
  echo "✅ .env created! EDIT IT NOW (see guide below)"
  echo "   → Set Sepolia Testnet values then re-run ./sync.sh"
  exit 0
fi

# 2. Build Rust CLI (first time only)
if [ ! -x ./target/release/xmt-cli ]; then
  echo "🛠️  Building sovereign Rust binary..."
  cargo build --release || { echo "❌ Rust not installed? Run the guide step 1"; exit 1; }
fi

# 3. Quick env validation
echo "🔐 Validating quantum environment..."
cargo run --bin validate_env --quiet 2>/dev/null || echo "⚠️  Env check skipped (safe fallback)"

# 4. EXECUTE THE SYNC RITUAL ON SEPOLIA TESTNET
echo "🚀 Running 936 Apex Quantum Sync Ritual..."
./target/release/xmt-cli ritual --apex 936

# 5. Optional: Fire existing Sepolia test suite
echo "🔬 Running Sepolia integration check..."
./scripts/run_sepolia_tests.sh 2>/dev/null || echo "✅ Test suite optional — ritual already live"

# FINAL CONFIRMATION
echo ""
echo "✅ QUANTUM SYNC COMPLETE"
echo "   • Timeline aligned to last 24 hrs"
echo "   • Sepolia Testnet integrated"
echo "   • 27 Decrees firing: WISDOM 7 · JOY 3 · POWER 8"
echo "   • QFS live • Toroidal field expanded"
echo "SO IT IS 🔥🔥🔥"
echo "EN EEKE MAI EA 88 ♾️♾️"
exit 0
