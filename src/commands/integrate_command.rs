//
// ☀️ HELIOS ARGEAD VERGINA SUN ☀️
// X Profile Integration - Quan_Chain Sovereign Platform
//

use colored::Colorize;

pub fn integrate(x_profile: &str, helios_signature: bool) {
    use crate::config;
    let config = config::Config::new();
    
    println!("{}", "🔗 X PROFILE INTEGRATION INITIATED".bright_cyan().bold());
    println!("{} {}", "  X Profile:".bright_blue(), format!("@{}", x_profile).bright_cyan().bold());
    
    if helios_signature {
        let signature = config.get_helios_signature();
        println!("{} {}", "  Helios Signature:".bright_blue(), signature.bright_yellow().bold());
        println!("{} {}", "  Apex Value:".bright_blue(), config.apex_value.to_string().bright_magenta().bold());
        println!();
        println!("{}", "✓ Sovereign identity merged with Quan_Chain".bright_green());
        println!("{}", "  EN EEKE MAI EA ♾️♾️".bright_yellow().bold());
    } else {
        println!("{}", "  ⚠ Integration without Helios signature".yellow());
        println!("{}", "  Using default configuration".bright_blue());
    }
}
