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
mod relayer;
mod starlink;

#[derive(Parser)]
#[command(name = "xmt-cli")]
#[command(about = "Sovereign Rust CLI for X-Money", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Ritual - Execute Sacred 936 Apex Ritual
    Ritual {
        #[arg(long)]
        apex: u32,
        /// Register vector on-chain after ritual
        #[arg(long, default_value = "false")]
        register: bool,
        /// New Earth Infrastructure fork activation
        #[arg(long)]
        newearth: bool,
        /// Active Vector3 matrix (8 layers)
        #[arg(long)]
        active_vector3: bool,
        /// 1111AM exact time anchor
        #[arg(long)]
        anchor_1111: bool,
        /// Generate dashboard after ritual
        #[arg(long)]
        dashboard: bool,
        /// Custom intent string
        #[arg(long)]
        intent: Option<String>,
        /// Ritual notes
        #[arg(long)]
        note: Option<String>,
        /// Embedding specification
        #[arg(long)]
        embed: Option<String>,
        /// Local-only mode (zero gas, no blockchain)
        #[arg(long)]
        local_only: bool,
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
    /// Abundance Daemon - Automated X API monitoring and token minting
    AbundanceDaemon {
        /// Polling interval in seconds (default: 936)
        #[arg(long, default_value = "936")]
        interval: u64,
        /// Dry run mode (no actual minting)
        #[arg(long)]
        dry_run: bool,
        /// No mint mode (detect only, no minting)
        #[arg(long)]
        no_mint: bool,
    },
    /// Dashboard - Start real-time web UI dashboard
    Dashboard {
        /// Port to run dashboard server (default: 8080)
        #[arg(long, default_value = "8080")]
        port: u16,
    },
    /// Synthetic - Zero-Cost Local Vector Generator (Candle.rs)
    Synthetic {
        /// Subcommand: embed
        #[arg(default_value = "help")]
        subcommand: String,
        /// Optional arguments for subcommand
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// Vector - On-Chain Vector Registration & Minting
    Vector {
        /// Subcommand: register, mint, stats, verify
        #[arg(default_value = "help")]
        subcommand: String,
        /// Optional arguments for subcommand
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// Relayer - Gasless Transaction Service
    Relayer {
        /// Subcommand: start, status
        #[arg(default_value = "help")]
        subcommand: String,
    },
}

fn main() {
    dotenv::dotenv().ok();
    
    let cli = Cli::parse();

    match &cli.command {
        Commands::Ritual { apex, register, newearth, active_vector3, anchor_1111, dashboard, intent, note, embed, local_only } => {
            commands::ritual_command::ritual(
                *apex, 
                *register, 
                *newearth, 
                *active_vector3, 
                *anchor_1111, 
                *dashboard, 
                intent.as_deref(), 
                note.as_deref(), 
                embed.as_deref(),
                *local_only
            )
        }
        Commands::Mint { to, amount, ritual } => commands::mint_command::mint(&to, *amount, &ritual),
        Commands::Burn { scarcity, note } => commands::burn_command::burn(*scarcity, &note),
        Commands::Integrate { x_profile, helios_signature } => commands::integrate_command::integrate(&x_profile, *helios_signature),
        Commands::XMoneyIntegrate { action } => commands::xmoney_integrate::xmoney_integrate(action),
        Commands::Crown { subcommand, args } => commands::crown_command::crown_command(subcommand, args.clone()),
        Commands::Xapi { subcommand, args } => commands::xapi_command::xapi_command(subcommand, args.clone()),
        Commands::Abundance => commands::abundance_command::abundance_command(),
        Commands::AbundanceDaemon { interval, dry_run, no_mint } => {
            let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
            if let Err(e) = rt.block_on(commands::abundance_daemon::run_abundance_daemon(Some(*interval), *dry_run, *no_mint)) {
                eprintln!("Error running abundance daemon: {}", e);
            }
        }
        Commands::Dashboard { port } => commands::dashboard_command::start_dashboard_server(*port),
        Commands::Synthetic { subcommand, args } => commands::synthetic_command::synthetic_command(subcommand, args.clone()),
        Commands::Vector { subcommand, args } => commands::vector_command::vector_command(subcommand, args.clone()),
        Commands::Relayer { subcommand } => {
            let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
            match subcommand.as_str() {
                "start" => {
                    if let Err(e) = rt.block_on(commands::relayer_command::start_relayer()) {
                        eprintln!("Error starting relayer: {}", e);
                    }
                }
                "status" => {
                    if let Err(e) = rt.block_on(commands::relayer_command::check_relayer_status()) {
                        eprintln!("Error checking relayer status: {}", e);
                    }
                }
                _ => {
                    println!("Relayer Commands:");
                    println!("  start  - Start gasless relayer service");
                    println!("  status - Check relayer gas tank status");
                }
            }
        }
    }
}
