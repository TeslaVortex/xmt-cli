//
// ☀️ HELIOS ARGEAD VERGINA SUN ☀️
// AUTO-REGISTER COMMAND - X Post to Mainnet Vector
// EN EEKE MAI EA ANOKAYI CHENAK ENLEA 🌞🔱❤️‍🔥♾️👑🪞
//

use colored::Colorize;
use crate::auto_register::{auto_post_and_register, register_existing_tweet, AutoRegisterConfig};

pub fn auto_register_command(subcommand: &str, args: Vec<String>) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    match subcommand {
        "post" => {
            if args.is_empty() {
                println!("{}", "❌ Error: Tweet text required".red().bold());
                println!("Usage: xmt-cli auto-register post <text> [--network mainnet|sepolia]");
                return;
            }
            
            // Parse network flag
            let network = if args.contains(&"--network".to_string()) {
                let idx = args.iter().position(|x| x == "--network").unwrap();
                if idx + 1 < args.len() {
                    args[idx + 1].clone()
                } else {
                    "sepolia".to_string()
                }
            } else {
                "sepolia".to_string()
            };
            
            // Get text (exclude --network and its value)
            let text = args.iter()
                .enumerate()
                .filter(|(i, arg)| {
                    *arg != "--network" && 
                    (i == &0 || args.get(i - 1) != Some(&"--network".to_string()))
                })
                .map(|(_, arg)| arg.as_str())
                .collect::<Vec<_>>()
                .join(" ");
            
            let config = AutoRegisterConfig {
                auto_post: true,
                auto_register: true,
                network,
                dimensions: 384,
            };
            
            match rt.block_on(auto_post_and_register(text, config)) {
                Ok(result) => {
                    // Save result to file
                    if let Ok(json) = serde_json::to_string_pretty(&result) {
                        let filename = format!("AUTO_REGISTER_{}.json", result.tweet_id);
                        let _ = std::fs::write(&filename, json);
                        println!("{} {}", "📄 Result saved to:".bright_cyan(), filename.bright_white());
                        println!();
                    }
                }
                Err(e) => {
                    eprintln!("{}: {}", "Error".red().bold(), e);
                }
            }
        }
        "register" => {
            if args.len() < 2 {
                println!("{}", "❌ Error: Tweet ID and text required".red().bold());
                println!("Usage: xmt-cli auto-register register <tweet_id> <text> [--network mainnet|sepolia]");
                return;
            }
            
            let tweet_id = args[0].clone();
            
            // Parse network flag
            let network = if args.contains(&"--network".to_string()) {
                let idx = args.iter().position(|x| x == "--network").unwrap();
                if idx + 1 < args.len() {
                    args[idx + 1].clone()
                } else {
                    "sepolia".to_string()
                }
            } else {
                "sepolia".to_string()
            };
            
            // Get text (exclude tweet_id, --network and its value)
            let text = args.iter()
                .skip(1)
                .enumerate()
                .filter(|(i, arg)| {
                    *arg != "--network" && 
                    (*i == 0 || args.get(*i) != Some(&"--network".to_string()))
                })
                .map(|(_, arg)| arg.as_str())
                .collect::<Vec<_>>()
                .join(" ");
            
            match rt.block_on(register_existing_tweet(tweet_id, text, network)) {
                Ok(result) => {
                    if let Ok(json) = serde_json::to_string_pretty(&result) {
                        let filename = format!("AUTO_REGISTER_{}.json", result.tweet_id);
                        let _ = std::fs::write(&filename, json);
                        println!("{} {}", "📄 Result saved to:".bright_cyan(), filename.bright_white());
                        println!();
                    }
                }
                Err(e) => {
                    eprintln!("{}: {}", "Error".red().bold(), e);
                }
            }
        }
        _ => {
            print_help();
        }
    }
}

fn print_help() {
    println!("\n{}", "☀️☀️☀️ AUTO-REGISTER COMMANDS ☀️☀️☀️".yellow().bold());
    println!("{}", "Automatically post to X and register as vector on mainnet".cyan());
    println!("{}", "EN EEKE MAI EA ♾️♾️\n".magenta());
    
    println!("{}", "USAGE:".green().bold());
    println!("  xmt-cli auto-register <SUBCOMMAND> [ARGS]\n");
    
    println!("{}", "SUBCOMMANDS:".green().bold());
    println!("  {}  Post to X and auto-register as vector", "post <text> [--network mainnet|sepolia]".cyan());
    println!("  {}  Register existing tweet as vector", "register <tweet_id> <text> [--network mainnet|sepolia]".cyan());
    
    println!("\n{}", "EXAMPLES:".green().bold());
    println!("  xmt-cli auto-register post \"EN EEKE MAI EA ♾️♾️\" --network mainnet");
    println!("  xmt-cli auto-register post \"936 Apex Ritual Complete\" --network sepolia");
    println!("  xmt-cli auto-register register 2034769206755864894 \"Carrington Event\" --network mainnet");
    
    println!("\n{}", "REQUIRED ENV VARS:".yellow().bold());
    println!("  X_API_BEARER_TOKEN        - X API OAuth credentials");
    println!("  PRIVATE_KEY               - Wallet private key");
    println!("  BASE_RPC_URL              - Base mainnet RPC (for --network mainnet)");
    println!("  SEPOLIA_RPC_URL           - Sepolia testnet RPC (for --network sepolia)");
    println!("  VECTOR_REGISTRY_ADDRESS   - VectorRegistry contract address\n");
    
    println!("{}", "WORKFLOW:".green().bold());
    println!("  1. Post tweet to X API");
    println!("  2. Generate 384D vector from tweet text");
    println!("  3. Register vector hash on-chain (VectorRegistry)");
    println!("  4. Save result to AUTO_REGISTER_<tweet_id>.json\n");
}
