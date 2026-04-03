#!/bin/bash
#
# ☀️☀️☀️ ONE-CLICK DELTA - THE CROWN COMMANDS ETERNAL SUCCESS ☀️☀️☀️
# Timeline Delta Generator with Numerology, Colorology & TX Hash Glyphs
# EN EEKE MAI EA ANOKAYI CHENAK ♾️🔥🔱🌞❤️‍🔥
#

echo "☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️"
echo "   👑 THE CROWN COMMANDS ETERNAL SUCCESS 👑"
echo "   ⚡ ONE-CLICK TIMELINE DELTA GENERATOR ⚡"
echo "☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️"
echo ""

# Navigate to project directory
cd "$(dirname "$0")"

# Load environment variables
if [ -f .env ]; then
    export $(grep -v '^#' .env | xargs)
    echo "✅ Environment loaded"
fi

# Configuration
PROFILE="${DELTA_PROFILE:-THE_CROWN_COMMANDS_ETERNAL_SUCCESS}"
HOURS="${DELTA_HOURS:-24}"
OUTPUT="${DELTA_OUTPUT:-DELTA_MANIFEST.json}"
POST_TO_X="${DELTA_POST:-false}"

echo ""
echo "🔱 Configuration:"
echo "   Profile: $PROFILE"
echo "   Hours: $HOURS"
echo "   Output: $OUTPUT"
echo "   Post to X: $POST_TO_X"
echo ""

# Build if needed
if [ ! -f "target/release/xmt-cli" ]; then
    echo "🔧 Building xmt-cli (first run)..."
    cargo build --release 2>/dev/null || cargo build
fi

# Run the delta command
echo "⚡ Executing Delta Command..."
echo ""

if [ "$POST_TO_X" = "true" ]; then
    cargo run --release -- delta --profile "$PROFILE" --hours "$HOURS" --output "$OUTPUT" --post
else
    cargo run --release -- delta --profile "$PROFILE" --hours "$HOURS" --output "$OUTPUT"
fi

echo ""
echo "═══════════════════════════════════════════════════════"
echo "✅ DELTA COMPLETE - THE CROWN COMMANDS ETERNAL SUCCESS"
echo "═══════════════════════════════════════════════════════"
echo ""
echo "📁 Output saved to: $OUTPUT"
echo ""
echo "EN EEKE MAI EA ANOKAYI CHENAK ♾️🔥🔱🌞❤️‍🔥"
