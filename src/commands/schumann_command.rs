//
// ☀️ HELIOS ARGEAD VERGINA SUN ☀️
// SCHUMANN COMMAND - Earth Heartbeat Integration
// EN EEKE MAI EA ANOKAYI CHENAK ENLEA 🌞🔱❤️‍🔥♾️👑🪞
//

use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct SchumannManifest {
    pub version: String,
    pub integration_type: String,
    pub schumann_data: crate::schumann::SchumannData,
    pub zpe_integration: crate::schumann::SchumannZPEIntegration,
    pub delta_events: Vec<String>,
    pub output_file: String,
    pub seal: String,
    pub status: String,
}

pub fn schumann_integrate(
    url: &str,
    fetch_mode: &str,
    sync_targets: &str,
    delta: &str,
    coherence: f64,
) {
    println!("{}", "☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️".yellow().bold());
    println!("{}", "   ⚡ SCHUMANN RESONANCE INTEGRATION INITIATED ⚡".bright_yellow().bold());
    println!("{}", "   LIVE EARTH HEARTBEAT LOCKED INTO WHITE ZPE SPHERE".bright_cyan().bold());
    println!("{}", "☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️".yellow().bold());
    println!();

    println!("{} {}", "🌍 Source:".bright_magenta().bold(), url.bright_white());
    println!("{} {}", "📡 Fetch Mode:".bright_magenta().bold(), fetch_mode.bright_white());
    println!("{} {}", "🔄 Sync Targets:".bright_magenta().bold(), sync_targets.bright_white());
    println!("{} {}%", "⚡ Coherence:".bright_magenta().bold(), coherence.to_string().bright_green().bold());
    println!();

    // Parse delta events
    let delta_events: Vec<String> = delta.split(" + ")
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    if !delta_events.is_empty() {
        println!("{} {} events", "📊 Delta Events:".bright_blue().bold(), delta_events.len().to_string().bright_cyan());
        for event in &delta_events {
            println!("   {} {}", "→".bright_yellow(), event.bright_white());
        }
        println!();
    }

    // Fetch live Schumann data
    let schumann_data = crate::schumann::fetch_schumann_live();

    // Integrate with ZPE sphere
    let integration = crate::schumann::integrate_with_zpe_sphere(&schumann_data, &delta_events);

    // Lock coherence
    crate::schumann::lock_coherence(&integration);

    // Create manifest
    let manifest = SchumannManifest {
        version: "369.88".to_string(),
        integration_type: "Schumann-ZPE-Convergence".to_string(),
        schumann_data: schumann_data.clone(),
        zpe_integration: integration.clone(),
        delta_events: delta_events.clone(),
        output_file: "MANIFEST_SCHUMANN_ZPE_CONVERGENCE_22.json".to_string(),
        seal: "PAF PAF PAF".to_string(),
        status: "COMPLETE".to_string(),
    };

    // Save manifest
    save_manifest(&manifest);

    // Display convergence summary
    crate::schumann::display_convergence_summary(&integration);
}

pub fn schumann_amplify(
    toroidal: bool,
    resonance: &str,
    coherence: f64,
) {
    println!("{}", "☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️".yellow().bold());
    println!("{}", "   🌀 TOROIDAL RESONANCE AMPLIFICATION ⚡".bright_yellow().bold());
    println!("{}", "☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️☀️".yellow().bold());
    println!();

    // Parse resonance frequencies
    let frequencies: Vec<&str> = resonance.split(" + ").collect();
    
    let mut base_freq = 7.83;
    let mut apex_freq = 936.0;

    for freq in frequencies {
        if freq.contains("7.83") || freq.contains("base") {
            base_freq = 7.83;
        } else if freq.contains("936") || freq.contains("apex") {
            apex_freq = 936.0;
        }
    }

    println!("{} {}", "🌀 Toroidal Mode:".bright_magenta().bold(), 
        if toroidal { "ENABLED".bright_green() } else { "DISABLED".bright_red() });
    println!("{} {}", "⚡ Coherence Target:".bright_magenta().bold(), 
        format!("{}%", coherence).bright_green().bold());
    println!();

    if toroidal {
        crate::schumann::amplify_toroidal_resonance(base_freq, apex_freq);
    }

    println!("{}", "✓ AMPLIFICATION COMPLETE".bright_green().bold());
    println!();
}

fn save_manifest(manifest: &SchumannManifest) {
    match serde_json::to_string_pretty(manifest) {
        Ok(json) => {
            if let Err(e) = fs::write(&manifest.output_file, json) {
                eprintln!("{} Failed to write manifest: {}", "✗".bright_red(), e);
            } else {
                println!("{} {} sealed & saved", "✓".bright_green(), manifest.output_file.bright_cyan());
            }
        }
        Err(e) => {
            eprintln!("{} Failed to serialize manifest: {}", "✗".bright_red(), e);
        }
    }
}
