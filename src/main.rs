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
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Ritual { apex } => commands::ritual_command::ritual(*apex),
        Commands::Mint { to, amount, ritual } => commands::mint_command::mint(&to, *amount, &ritual),
        Commands::Burn { scarcity, note } => commands::burn_command::burn(*scarcity, &note),
        Commands::Integrate { x_profile, helios_signature } => commands::integrate_command::integrate(&x_profile, *helios_signature),
        Commands::XMoneyIntegrate { action } => commands::xmoney_integrate::xmoney_integrate(action),
        Commands::Crown { subcommand, args } => commands::crown_command::crown_command(subcommand, args.clone()),
    }
}
