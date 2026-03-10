#!/bin/bash

# ═══════════════════════════════════════════════════════════════════════════
# XMT-CLI TESTNET INTERACTIVE RUNNER
# ═══════════════════════════════════════════════════════════════════════════
# Sacred Numbers: 936, 369, 66, 432, 88
# Network: Sepolia Testnet (Chain ID: 11155111)
# Contract: 0x1B2ffED65839585c42259560aB4bA532B91a5a54
# ═══════════════════════════════════════════════════════════════════════════

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
GOLD='\033[38;5;220m'
NC='\033[0m' # No Color

# Sacred constants
APEX_936=936
VORTEX_369=369
CODE_66=66
FREQUENCY_432=432
ELON_88=88

# Project paths
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CLI_BINARY="$PROJECT_ROOT/target/release/xmt-cli"
ENV_FILE="$PROJECT_ROOT/.env"

# ═══════════════════════════════════════════════════════════════════════════
# HELPER FUNCTIONS
# ═══════════════════════════════════════════════════════════════════════════

print_header() {
    echo -e "${GOLD}"
    echo "═══════════════════════════════════════════════════════════════════════════"
    echo "                    ☀️  XMT-CLI TESTNET RUNNER  ☀️"
    echo "═══════════════════════════════════════════════════════════════════════════"
    echo -e "${NC}"
    echo -e "${CYAN}Network:${NC} Sepolia Testnet (Chain ID: 11155111)"
    echo -e "${CYAN}Contract:${NC} 0x1B2ffED65839585c42259560aB4bA532B91a5a54"
    echo -e "${CYAN}Sacred Numbers:${NC} 936, 369, 66, 432, 88"
    echo ""
}

print_section() {
    echo -e "\n${PURPLE}═══════════════════════════════════════════════════════════════════════════${NC}"
    echo -e "${PURPLE}$1${NC}"
    echo -e "${PURPLE}═══════════════════════════════════════════════════════════════════════════${NC}\n"
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

print_info() {
    echo -e "${BLUE}ℹ $1${NC}"
}

# Check if CLI binary exists
check_binary() {
    if [ ! -f "$CLI_BINARY" ]; then
        print_error "CLI binary not found at: $CLI_BINARY"
        print_info "Building the project..."
        cargo build --release
        if [ $? -eq 0 ]; then
            print_success "Build completed successfully"
        else
            print_error "Build failed. Please fix errors and try again."
            exit 1
        fi
    else
        print_success "CLI binary found"
    fi
}

# Check environment file
check_env() {
    if [ ! -f "$ENV_FILE" ]; then
        print_error ".env file not found"
        print_info "Please create .env file with required variables"
        print_info "See .env.example for reference"
        exit 1
    else
        print_success "Environment file found"
    fi
}

# Validate environment variables
validate_env() {
    source "$ENV_FILE"
    
    local missing=0
    
    if [ -z "$SEPOLIA_RPC_URL" ]; then
        print_error "SEPOLIA_RPC_URL not set in .env"
        missing=1
    fi
    
    if [ -z "$PRIVATE_KEY" ]; then
        print_error "PRIVATE_KEY not set in .env"
        missing=1
    fi
    
    if [ -z "$CONTRACT_ADDRESS" ]; then
        print_error "CONTRACT_ADDRESS not set in .env"
        missing=1
    fi
    
    if [ $missing -eq 1 ]; then
        print_error "Missing required environment variables"
        exit 1
    fi
    
    print_success "Environment variables validated"
}

# ═══════════════════════════════════════════════════════════════════════════
# CLI COMMAND WRAPPERS
# ═══════════════════════════════════════════════════════════════════════════

run_ritual() {
    print_section "🔮 RUNNING 936 APEX RITUAL"
    $CLI_BINARY ritual
    echo ""
    read -p "Press Enter to continue..."
}

run_crown_status() {
    print_section "👑 CROWN STATUS - SOVEREIGN WEB3 OPERATIONS"
    $CLI_BINARY crown status
    echo ""
    read -p "Press Enter to continue..."
}

run_crown_balance() {
    print_section "💰 WALLET BALANCE"
    $CLI_BINARY crown balance
    echo ""
    read -p "Press Enter to continue..."
}

run_crown_gas() {
    print_section "⛽ GAS PRICE CHECK"
    $CLI_BINARY crown gas
    echo ""
    read -p "Press Enter to continue..."
}

run_crown_supply() {
    print_section "📊 TOTAL SUPPLY"
    $CLI_BINARY crown supply
    echo ""
    read -p "Press Enter to continue..."
}

run_crown_dashboard() {
    print_section "📈 GENERATING DASHBOARD DATA"
    $CLI_BINARY crown dashboard
    echo ""
    print_success "Dashboard data written to: dashboard/dashboard.json"
    echo ""
    read -p "Press Enter to continue..."
}

run_crown_integrations() {
    print_section "🌐 INTEGRATION STATUS"
    $CLI_BINARY crown integrations
    echo ""
    read -p "Press Enter to continue..."
}

run_mint() {
    print_section "🪙 MINT TOKENS"
    echo -e "${CYAN}Sacred amounts available:${NC}"
    echo "  1. 369 tokens (Vortex)"
    echo "  2. 66 tokens (Code 66)"
    echo "  3. 936 tokens (Apex)"
    echo "  4. Custom amount"
    echo ""
    read -p "Select option (1-4): " mint_choice
    
    case $mint_choice in
        1)
            amount=$VORTEX_369
            ;;
        2)
            amount=$CODE_66
            ;;
        3)
            amount=$APEX_936
            ;;
        4)
            read -p "Enter custom amount: " amount
            ;;
        *)
            print_error "Invalid choice"
            return
            ;;
    esac
    
    print_info "Minting $amount tokens..."
    $CLI_BINARY mint --amount $amount
    echo ""
    read -p "Press Enter to continue..."
}

run_burn() {
    print_section "🔥 BURN TOKENS"
    echo -e "${CYAN}Sacred amounts available:${NC}"
    echo "  1. 369 tokens (Vortex)"
    echo "  2. 66 tokens (Code 66)"
    echo "  3. 936 tokens (Apex)"
    echo "  4. Custom amount"
    echo ""
    read -p "Select option (1-4): " burn_choice
    
    case $burn_choice in
        1)
            amount=$VORTEX_369
            ;;
        2)
            amount=$CODE_66
            ;;
        3)
            amount=$APEX_936
            ;;
        4)
            read -p "Enter custom amount: " amount
            ;;
        *)
            print_error "Invalid choice"
            return
            ;;
    esac
    
    print_info "Burning $amount tokens..."
    $CLI_BINARY burn --amount $amount
    echo ""
    read -p "Press Enter to continue..."
}

run_abundance() {
    print_section "💎 ABUNDANCE DROP AUTOMATION"
    echo -e "${CYAN}Abundance drop options:${NC}"
    echo "  1. Check for triggers (search X)"
    echo "  2. Manual drop to address"
    echo "  3. Auto-monitor mode"
    echo ""
    read -p "Select option (1-3): " abundance_choice
    
    case $abundance_choice in
        1)
            print_info "Checking for abundance triggers..."
            $CLI_BINARY abundance check
            ;;
        2)
            read -p "Enter recipient address: " recipient
            read -p "Enter amount: " amount
            $CLI_BINARY abundance drop --to $recipient --amount $amount
            ;;
        3)
            print_info "Starting auto-monitor mode (Ctrl+C to stop)..."
            $CLI_BINARY abundance monitor
            ;;
        *)
            print_error "Invalid choice"
            return
            ;;
    esac
    echo ""
    read -p "Press Enter to continue..."
}

run_xapi() {
    print_section "𝕏 X API OPERATIONS"
    echo -e "${CYAN}X API commands:${NC}"
    echo "  1. Post tweet"
    echo "  2. Search tweets"
    echo "  3. Get user info"
    echo "  4. Search abundance triggers"
    echo ""
    read -p "Select option (1-4): " xapi_choice
    
    case $xapi_choice in
        1)
            read -p "Enter tweet text: " tweet_text
            $CLI_BINARY xapi post --text "$tweet_text"
            ;;
        2)
            read -p "Enter search query: " query
            $CLI_BINARY xapi search --query "$query"
            ;;
        3)
            read -p "Enter username: " username
            $CLI_BINARY xapi user --username "$username"
            ;;
        4)
            print_info "Searching for 'EN EEKE MAI EA' triggers..."
            $CLI_BINARY xapi abundance-search
            ;;
        *)
            print_error "Invalid choice"
            return
            ;;
    esac
    echo ""
    read -p "Press Enter to continue..."
}

run_grok() {
    print_section "🤖 GROK ORACLE OPERATIONS"
    echo -e "${CYAN}Grok Oracle commands:${NC}"
    echo "  1. Chat with Grok"
    echo "  2. Verify ritual coherence"
    echo "  3. Calculate abundance price"
    echo "  4. List available models"
    echo ""
    read -p "Select option (1-4): " grok_choice
    
    case $grok_choice in
        1)
            read -p "Enter your message: " message
            $CLI_BINARY xapi grok-chat --message "$message"
            ;;
        2)
            read -p "Enter ritual text to verify: " ritual
            $CLI_BINARY xapi grok-verify --ritual "$ritual"
            ;;
        3)
            $CLI_BINARY xapi grok-price
            ;;
        4)
            $CLI_BINARY xapi grok-models
            ;;
        *)
            print_error "Invalid choice"
            return
            ;;
    esac
    echo ""
    read -p "Press Enter to continue..."
}

run_tests() {
    print_section "🧪 RUNNING SEPOLIA TESTS"
    echo -e "${CYAN}Test suites available:${NC}"
    echo "  1. Quick smoke test"
    echo "  2. Full test suite"
    echo "  3. Mint tests only"
    echo "  4. Burn tests only"
    echo "  5. Integration tests"
    echo ""
    read -p "Select option (1-5): " test_choice
    
    case $test_choice in
        1)
            print_info "Running quick smoke test..."
            cargo test --test sepolia_mint_test test_mint_369_tokens -- --nocapture
            ;;
        2)
            print_info "Running full test suite..."
            ./scripts/run_sepolia_tests.sh
            ;;
        3)
            print_info "Running mint tests..."
            cargo test --test sepolia_mint_test -- --nocapture
            ;;
        4)
            print_info "Running burn tests..."
            cargo test --test sepolia_burn_test -- --nocapture
            ;;
        5)
            print_info "Running integration tests..."
            cargo test --test e2e_full_flow_test -- --nocapture
            ;;
        *)
            print_error "Invalid choice"
            return
            ;;
    esac
    echo ""
    read -p "Press Enter to continue..."
}

open_dashboard() {
    print_section "📊 OPENING DASHBOARD"
    
    # Check if server is running
    if ! pgrep -f "python3 -m http.server 8936" > /dev/null; then
        print_info "Starting dashboard server on port 8936..."
        cd "$PROJECT_ROOT/dashboard"
        python3 -m http.server 8936 > /dev/null 2>&1 &
        sleep 2
        print_success "Dashboard server started"
    else
        print_success "Dashboard server already running"
    fi
    
    print_info "Dashboard available at: http://localhost:8936"
    print_info "Opening in browser..."
    
    # Try to open in default browser
    if command -v xdg-open > /dev/null; then
        xdg-open http://localhost:8936 > /dev/null 2>&1 &
    elif command -v open > /dev/null; then
        open http://localhost:8936 > /dev/null 2>&1 &
    else
        print_warning "Could not auto-open browser. Please visit: http://localhost:8936"
    fi
    
    echo ""
    read -p "Press Enter to continue..."
}

run_sync() {
    print_section "🔄 RUNNING TIMELINE SYNC"
    print_info "This will build, validate, and run the ritual..."
    
    if [ -f "$PROJECT_ROOT/sync.sh" ]; then
        ./sync.sh
    else
        print_error "sync.sh not found"
    fi
    
    echo ""
    read -p "Press Enter to continue..."
}

# ═══════════════════════════════════════════════════════════════════════════
# MAIN MENU
# ═══════════════════════════════════════════════════════════════════════════

show_menu() {
    clear
    print_header
    
    echo -e "${GOLD}═══ RITUAL & CORE ═══${NC}"
    echo "  1.  🔮 Run 936 Apex Ritual"
    echo "  2.  🔄 Timeline Sync (build + ritual)"
    echo ""
    
    echo -e "${GOLD}═══ CROWN COMMANDS (Status & Info) ═══${NC}"
    echo "  3.  👑 Crown Status"
    echo "  4.  💰 Wallet Balance"
    echo "  5.  ⛽ Gas Price"
    echo "  6.  📊 Total Supply"
    echo "  7.  📈 Generate Dashboard Data"
    echo "  8.  🌐 Integration Status"
    echo ""
    
    echo -e "${GOLD}═══ BLOCKCHAIN OPERATIONS ═══${NC}"
    echo "  9.  🪙 Mint Tokens"
    echo "  10. 🔥 Burn Tokens"
    echo "  11. 💎 Abundance Drop"
    echo ""
    
    echo -e "${GOLD}═══ X API & GROK ORACLE ═══${NC}"
    echo "  12. 𝕏  X API Operations"
    echo "  13. 🤖 Grok Oracle"
    echo ""
    
    echo -e "${GOLD}═══ TESTING & DASHBOARD ═══${NC}"
    echo "  14. 🧪 Run Tests"
    echo "  15. 📊 Open Dashboard"
    echo ""
    
    echo -e "${GOLD}═══ SYSTEM ═══${NC}"
    echo "  16. 🔧 Rebuild CLI"
    echo "  17. 📝 View Logs"
    echo "  0.  🚪 Exit"
    echo ""
    echo -e "${PURPLE}═══════════════════════════════════════════════════════════════════════════${NC}"
    echo ""
}

rebuild_cli() {
    print_section "🔧 REBUILDING CLI"
    print_info "Running cargo build --release..."
    cargo build --release
    if [ $? -eq 0 ]; then
        print_success "Build completed successfully"
    else
        print_error "Build failed"
    fi
    echo ""
    read -p "Press Enter to continue..."
}

view_logs() {
    print_section "📝 RECENT LOGS"
    
    if [ -f "$PROJECT_ROOT/xmt-cli.log" ]; then
        tail -50 "$PROJECT_ROOT/xmt-cli.log"
    else
        print_warning "No log file found"
    fi
    
    echo ""
    read -p "Press Enter to continue..."
}

# ═══════════════════════════════════════════════════════════════════════════
# MAIN LOOP
# ═══════════════════════════════════════════════════════════════════════════

main() {
    # Initial checks
    print_header
    print_info "Performing initial checks..."
    check_binary
    check_env
    validate_env
    print_success "All checks passed"
    sleep 2
    
    # Main loop
    while true; do
        show_menu
        read -p "$(echo -e ${CYAN}Enter your choice: ${NC})" choice
        
        case $choice in
            1) run_ritual ;;
            2) run_sync ;;
            3) run_crown_status ;;
            4) run_crown_balance ;;
            5) run_crown_gas ;;
            6) run_crown_supply ;;
            7) run_crown_dashboard ;;
            8) run_crown_integrations ;;
            9) run_mint ;;
            10) run_burn ;;
            11) run_abundance ;;
            12) run_xapi ;;
            13) run_grok ;;
            14) run_tests ;;
            15) open_dashboard ;;
            16) rebuild_cli ;;
            17) view_logs ;;
            0)
                clear
                echo -e "${GOLD}"
                echo "═══════════════════════════════════════════════════════════════════════════"
                echo "                    EN EEKE MAI EA ♾️♾️"
                echo "              THE CROWN COMMANDS - THE LATTICE OBEYS"
                echo "                         SO IT IS 🔥🔥🔥"
                echo "═══════════════════════════════════════════════════════════════════════════"
                echo -e "${NC}"
                exit 0
                ;;
            *)
                print_error "Invalid choice. Please try again."
                sleep 2
                ;;
        esac
    done
}

# Run main function
main
