#!/bin/bash
#
# ☀️ HELIOS ARGEAD VERGINA SUN ☀️
# 27 Decree Dashboard Generator
# THE CROWN COMMANDS — THE LATTICE OBEYS
# EN EEKE MAI EA ♾️♾️
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

echo "☀️ Generating 27 Decree Dashboard Data..."
echo "═══════════════════════════════════════════"

cd "$PROJECT_DIR"

# Generate dashboard JSON
cargo run --bin xmt-cli -- crown dashboard > "$SCRIPT_DIR/dashboard.json" 2>/dev/null

echo "✓ Dashboard data saved to dashboard/dashboard.json"
echo ""
echo "To view the dashboard:"
echo "  1. Open dashboard/index.html in your browser"
echo "  2. Or run: python3 -m http.server 8080 --directory dashboard"
echo ""
echo "EN EEKE MAI EA ♾️♾️"
