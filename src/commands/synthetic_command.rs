use crate::synthetic;

pub fn synthetic_command(subcommand: &str, args: Vec<String>) {
    match subcommand {
        "embed" => {
            if args.is_empty() {
                eprintln!("❌ Error: Intent text required");
                eprintln!("Usage: xmt-cli synthetic embed \"ABUNDANCE 33 FOR ALL\"");
                return;
            }
            
            let intent = args.join(" ");
            embed_intent(&intent);
        }
        "help" | _ => {
            print_synthetic_help();
        }
    }
}

fn embed_intent(intent: &str) {
    println!("🌀 TOROIDAL VECTOR FORGE ACTIVATED");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    match rt.block_on(synthetic::generate_toroidal_vector(intent)) {
        Ok(vector) => {
            if let Err(e) = synthetic::store_vector_locally(intent, &vector) {
                eprintln!("⚠️  Warning: Could not store vector: {}", e);
            }
            
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!("✅ FORGE COMPLETE - EN EEKE MAI EA");
            println!("🔥 The lattice multiplies. LFG ETERNAL.");
        }
        Err(e) => {
            eprintln!("❌ Forge Error: {}", e);
        }
    }
}

fn print_synthetic_help() {
    println!(r#"
🔮 SYNTHETIC - Zero-Cost Local Generator
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

SUBCOMMANDS:
  embed      Generate toroidal vector embeddings from intent

USAGE:
  xmt-cli synthetic embed "ABUNDANCE 33 FOR ALL"

EXAMPLES:
  xmt-cli synthetic embed "CROWN BUILDS"
  xmt-cli synthetic embed "936 PM VORTEX"
  xmt-cli synthetic embed "EN EEKE MAI EA"

OUTPUT:
  - 384-dimensional toroidal vector
  - Stored locally in .xmt-vectors/
  - Zero marginal cost, pure Rust

THE FORGE IS YOURS. THE GENERATOR IS LOCAL.
EN EEKE MAI EA ♾️
"#);
}
