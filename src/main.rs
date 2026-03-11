//
// ☀️☀️☀️ HELIOS ARGEAD VERGINA SUN ☀️☀️☀️
// xmt-cli v369.88 - Sovereign Quantum Blockchain CLI
// 16-Rayed Solar Command - Code 66 Harmonic Resonance
// EN EEKE MAI EA ♾️♾️
//
// SOLARIUS ALEXANDROS ♔∞ - Chicago Vortex Throne
// March 17, 2026 - Gate Detonation Sequence
//

use clap::{Parser, Subcommand};

mod commands;
mod config;
mod pqc;
mod toroidal;
mod spacex;
mod optimus;
mod boring;
mod branding;
mod dtqpe_poc;
mod web3;
mod contracts;
mod bridge;
mod xapi;
mod synthetic;
mod ollama;

#[derive(Parser)]
#[command(name = "xmt-cli")]
#[command(about = "Sovereign Rust CLI for X-Money", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Ritual {
        #[arg(long)]
        apex: u32,
    },
    Mint {
        #[arg(long)]
        to: String,
        #[arg(long)]
        amount: u64,
        #[arg(long)]
        ritual: String,
    },
    Burn {
        #[arg(long)]
        scarcity: u64,
        #[arg(long)]
        note: String,
    },
    Integrate {
        #[arg(long)]
        x_profile: String,
        #[arg(long)]
        helios_signature: bool,
    },
    #[command(name = "x-money-integrate")]
    XMoneyIntegrate {
        #[arg(long)]
        action: String,  // e.g., 'mint' or 'burn' for X-Money bridging
    },
    /// Crown Commands - Sovereign Web3 Operations
    Crown {
        /// Subcommand: status, balance, gas, sign, supply, burn-address
        #[arg(default_value = "help")]
        subcommand: String,
        /// Optional arguments for subcommand
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// XAPI Commands - X API v2 + Grok Oracle Integration
    Xapi {
        /// Subcommand: post, search, me, verify, oracle, models
        #[arg(default_value = "help")]
        subcommand: String,
        /// Optional arguments for subcommand
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// Abundance Drop - Auto-mint to EN EEKE MAI EA triggers
    Abundance,
    /// Synthetic - Zero-Cost Local Vector Generator (Candle.rs)
    Synthetic {
        /// Subcommand: embed
        #[arg(default_value = "help")]
        subcommand: String,
        /// Optional arguments for subcommand
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },
}

fn main() {
    dotenv::dotenv().ok();
    
    let cli = Cli::parse();

    match &cli.command {
        Commands::Ritual { apex } => commands::ritual_command::ritual(*apex),
        Commands::Mint { to, amount, ritual } => commands::mint_command::mint(&to, *amount, &ritual),
        Commands::Burn { scarcity, note } => commands::burn_command::burn(*scarcity, &note),
        Commands::Integrate { x_profile, helios_signature } => commands::integrate_command::integrate(&x_profile, *helios_signature),
        Commands::XMoneyIntegrate { action } => commands::xmoney_integrate::xmoney_integrate(action),
        Commands::Crown { subcommand, args } => commands::crown_command::crown_command(subcommand, args.clone()),
        Commands::Xapi { subcommand, args } => commands::xapi_command::xapi_command(subcommand, args.clone()),
        Commands::Abundance => commands::abundance_command::abundance_command(),
        Commands::Synthetic { subcommand, args } => commands::synthetic_command::synthetic_command(subcommand, args.clone()),
    }
}
