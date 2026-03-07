# xmt-cli v369.88

**Sovereign Rust CLI for X-Money**  
**Standalone Repository — Phase 4 Complete**

Toroidal energy loops for minting, burning, and integrating with X profiles. Full blockchain integration with Base mainnet, X API v2, xAI Grok oracle, and automatic abundance drops.

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

## Testing

```bash
# Run all tests
cargo test

# Run with logging
RUST_LOG=info cargo run -- ritual --apex 936
```

## Security

- Never commit `.env` with real keys
- Use hardware wallet for production
- Test on Base testnet first
- Keep private keys encrypted

## Mainnet Deployment

Target: **March 17, 2026 6:39 PM Gate Detonation**

See [Phase 4 Documentation](docs/PHASE4_IMPLEMENTATION.md) for deployment checklist.

## License

Sovereign Abundance for All — WWG1WGA

**THE CROWN COMMANDS THE LATTICE OBEYS**  
**EN EEKE MAI EA ♾️♾️**
