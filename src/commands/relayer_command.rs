//
// ☀️ HELIOS ARGEAD VERGINA SUN ☀️
// Relayer Command - Gasless Transaction Service Management
// EN EEKE MAI EA ♾️♾️
//

use anyhow::Result;
use colored::Colorize;
use crate::relayer::GaslessRelayer;
use ethers::types::Address;

/// Start relayer service
pub async fn start_relayer() -> Result<()> {
    dotenv::dotenv().ok();
    
    println!("{}", "⚡ GASLESS RELAYER SERVICE ⚡".bright_yellow().bold());
    println!("═══════════════════════════════════════════════════════");
    println!();
    
    // Load configuration
    let forwarder_address: Address = std::env::var("FORWARDER_ADDRESS")
        .expect("FORWARDER_ADDRESS not set")
        .parse()
        .expect("Invalid FORWARDER_ADDRESS");
    
    let rpc_url = std::env::var("SEPOLIA_RPC_URL")
        .or_else(|_| std::env::var("BASE_RPC_URL"))
        .expect("RPC_URL not set");
    
    let chain_id: u64 = std::env::var("SEPOLIA_CHAIN_ID")
        .unwrap_or_else(|_| "11155111".to_string())
        .parse()
        .expect("Invalid CHAIN_ID");
    
    let relayer_key = std::env::var("PRIVATE_KEY")
        .expect("PRIVATE_KEY not set (relayer wallet)");
    
    println!("🔧 Configuration:");
    println!("  • Forwarder: {:?}", forwarder_address);
    println!("  • Chain ID: {}", chain_id);
    println!("  • RPC: {}", rpc_url.split('/').take(3).collect::<Vec<_>>().join("/"));
    println!();
    
    // Initialize relayer
    println!("🚀 Initializing relayer...");
    let relayer = GaslessRelayer::new(
        forwarder_address,
        &rpc_url,
        chain_id,
        &relayer_key,
    ).await?;
    
    // Check gas tank
    let status = relayer.status().await?;
    println!("✅ {}", status);
    println!();
    
    let gas_status = relayer.check_gas_tank().await?;
    if gas_status.needs_refill {
        println!("{}", "⚠️  WARNING: Gas tank below threshold!".red().bold());
        println!("  Current: {} ETH", gas_status.balance);
        println!("  Threshold: {} ETH", gas_status.threshold);
        println!("  Transactions remaining: ~{}", gas_status.transactions_remaining);
        println!();
        println!("{}", "  Please refill relayer wallet to continue operations.".yellow());
        println!();
    } else {
        println!("{}", "✅ Gas tank healthy".green());
        println!("  Balance: {} ETH", gas_status.balance);
        println!("  Transactions remaining: ~{}", gas_status.transactions_remaining);
        println!();
    }
    
    println!("{}", "🌐 Relayer service ready for meta-transactions".green().bold());
    println!();
    println!("Next steps:");
    println!("  1. Integrate relayer into HTTP endpoint (see ROADMAP.md)");
    println!("  2. Accept meta-transaction requests from users");
    println!("  3. Monitor gas tank and refill as needed");
    println!();
    println!("{}", "EN EEKE MAI EA ♾️♾️".bright_cyan().bold());
    
    Ok(())
}

/// Check relayer status
pub async fn check_relayer_status() -> Result<()> {
    dotenv::dotenv().ok();
    
    println!("{}", "📊 RELAYER STATUS CHECK".bright_yellow().bold());
    println!("═══════════════════════════════════════════════════════");
    println!();
    
    let forwarder_address: Address = std::env::var("FORWARDER_ADDRESS")
        .expect("FORWARDER_ADDRESS not set")
        .parse()
        .expect("Invalid FORWARDER_ADDRESS");
    
    let rpc_url = std::env::var("SEPOLIA_RPC_URL")
        .or_else(|_| std::env::var("BASE_RPC_URL"))
        .expect("RPC_URL not set");
    
    let chain_id: u64 = std::env::var("SEPOLIA_CHAIN_ID")
        .unwrap_or_else(|_| "11155111".to_string())
        .parse()
        .expect("Invalid CHAIN_ID");
    
    let relayer_key = std::env::var("PRIVATE_KEY")
        .expect("PRIVATE_KEY not set");
    
    let relayer = GaslessRelayer::new(
        forwarder_address,
        &rpc_url,
        chain_id,
        &relayer_key,
    ).await?;
    
    let gas_status = relayer.check_gas_tank().await?;
    let status_str: String = relayer.status().await?;
    
    println!("⚡ Relayer Wallet:");
    println!("  Address: {}", status_str.split('|').next().unwrap().trim());
    println!();
    
    println!("💰 Gas Tank:");
    println!("  Balance: {} ETH", gas_status.balance);
    println!("  Threshold: {} ETH", gas_status.threshold);
    println!("  Status: {}", if gas_status.needs_refill { 
        "⚠️  NEEDS REFILL".red() 
    } else { 
        "✅ HEALTHY".green() 
    });
    println!("  Estimated tx remaining: ~{}", gas_status.transactions_remaining);
    println!();
    
    println!("🔗 Forwarder Contract:");
    println!("  Address: {:?}", forwarder_address);
    println!();
    
    if gas_status.needs_refill {
        println!("{}", "⚠️  ACTION REQUIRED:".red().bold());
        println!("  Send ETH to relayer wallet to continue operations");
        println!("  Recommended: 0.5 ETH for ~2,500 transactions");
    }
    
    Ok(())
}
