# xmt-cli v369.88

**Sovereign Rust CLI for X-Money**  
**Standalone Repository — Phase 4 Complete**  
**🌟 Repository Live: https://github.com/TeslaVortex/xmt-cli 🌟**

Toroidal energy loops for minting, burning, and integrating with X profiles. Full blockchain integration with Base mainnet, X API v2, xAI Grok oracle, and automatic abundance drops.

**✅ Ritual Complete — EN EEKE MAI EA ♾️♾️**  
**Repository Status:** Cleansed, optimized, and deployed to the digital aether.

## Features

✓ **Phase 1-2**: Core engine with DTQPE, PQC modules, toroidal ledger  
✓ **Phase 3**: X-Money integration subcommand  
✓ **Phase 4**: Real blockchain integration (Base mainnet), X API, Grok oracle, auto-drops, Layer-0 settlement

## Installation

```bash
# Clone repository
git clone https://github.com/TeslaVortex/xmt-cli
cd xmt-cli

# Build release binary
cargo build --release

# Binary location
./target/release/xmt-cli
```

## Configuration

1. Copy environment template:
```bash
cp .env.example .env
```

2. Edit `.env` with your credentials:
```bash
BASE_RPC_URL=https://mainnet.base.org
CHAIN_ID=8453
PRIVATE_KEY=your_private_key_here
X_API_BEARER_TOKEN=your_x_api_token_here
XAI_API_KEY=your_xai_key_here
XMONEY_CONTRACT_ADDRESS=0x...
```

## Usage

### Core Commands

```bash
# Daily ritual (936 apex)
xmt-cli ritual --apex 936

# Mint tokens
xmt-cli mint --to @Vortex369X --amount 369 --ritual "Abundance drop"

# Burn tokens
xmt-cli burn --scarcity 66 --note "Old fiat obliterated"

# Integrate X profile
xmt-cli integrate --x-profile Vortex369X --helios-signature
```

### Phase 4: Blockchain Integration

```bash
# Real blockchain mint (Base mainnet)
xmt-cli x-money-integrate --action mint

# Real blockchain burn
xmt-cli x-money-integrate --action burn
```

## Architecture

- **Web3 Provider**: Base mainnet (Chain ID: 8453)
- **Smart Contracts**: X-Money ERC-20 with mint/burn
- **X API v2**: Ritual posting, reply monitoring
- **xAI Grok Oracle**: Coherence verification, pricing
- **Auto-drop Engine**: 369 tokens per "EN EEKE MAI EA ♾️♾️" reply
- **Layer-0 Settlement**: Boring Company underground routing

## Documentation

- [Implementation Plan](IMPLEMENTATION_PLAN.md) - Overall vision and phases
- [API Endpoints](API_ENDPOINTS_IMPLEMENTATION_PLAN.md) - X API and Grok integration
- [Phase 4 Details](docs/PHASE4_IMPLEMENTATION.md) - Complete Phase 4 documentation
- [QFS 369 Toroidal Infrastructure](docs/QFS_369_TOROIDAL_INFRASTRUCTURE.md) - Quantum Financial System architecture
- [27 Decrees](docs/27_decrees.md) - Sovereign abundance protocols

## Testing

```bash
# Run all tests
cargo test

# Run with logging
RUST_LOG=info cargo run -- ritual --apex 936
```

## Security

- ✅ `.env` excluded from repository (never commit real keys)
- ✅ Build artifacts (`target/`) properly gitignored
- Use hardware wallet for production
- Test on Base testnet first
- Keep private keys encrypted
- Repository optimized: 96.5% size reduction (1.6GB → 111KB)

## Repository Setup

This repository has been cleansed and optimized for the quantum financial system:

- **Git History Cleaned**: Removed 1.6GB of build artifacts from history
- **Secrets Protected**: `.env` files excluded, GitHub push protection active
- **Optimized Size**: 96.5% reduction (1.6GB → 111KB pack size)
- **Ready for Deployment**: Clean commit history, proper `.gitignore` configuration

### Contributing

```bash
# Always ensure .env is not tracked
git status  # Verify .env is not staged

# Build artifacts are auto-ignored
cargo build  # target/ directory excluded automatically
```

## Mainnet Deployment

Target: **March 17, 2026 6:39 PM Gate Detonation**

See [Phase 4 Documentation](docs/PHASE4_IMPLEMENTATION.md) for deployment checklist.

## License

Sovereign Abundance for All — WWG1WGA

---

**THE CROWN COMMANDS THE LATTICE OBEYS**  
**EN EEKE MAI EA ♾️♾️**

*Repository established March 7, 2026 — Ritual Complete*  
*The toroidal infrastructure awaits activation*
