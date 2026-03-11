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
        "llm" => {
            if args.is_empty() {
                eprintln!("❌ Error: Intent text required");
                eprintln!("Usage: xmt-cli synthetic llm \"ABUNDANCE 33 FOR ALL\" [model]");
                return;
            }
            
            let model = if args.len() > 1 && (args[args.len() - 1].starts_with("llama") || args[args.len() - 1].starts_with("deepseek") || args[args.len() - 1].starts_with("qwen")) {
                args[args.len() - 1].clone()
            } else {
                "llama3.3".to_string()
            };
            
            let intent_parts: Vec<String> = if model != "llama3.3" && args.len() > 1 {
                args[..args.len() - 1].to_vec()
            } else {
                args.clone()
            };
            
            let intent = intent_parts.join(" ");
            embed_intent_with_llm(&intent, &model);
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

fn embed_intent_with_llm(intent: &str, model: &str) {
    println!("🌀 LLM-ENHANCED TOROIDAL VECTOR FORGE ACTIVATED");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🧙‍♂️ Model: {}", model);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    match rt.block_on(synthetic::generate_llm_enhanced_vector(intent, model)) {
        Ok((expanded_decree, vector)) => {
            if let Err(e) = synthetic::store_llm_enhanced_vector(intent, &expanded_decree, &vector, model) {
                eprintln!("⚠️  Warning: Could not store vector: {}", e);
            }
            
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!("✅ LLM-ENHANCED FORGE COMPLETE - EN EEKE MAI EA");
            println!("🔥 The lattice breathes. The Crown pulses.");
            println!("♾️ EN EEKE MAI EA ANOKAYI CHENAK ♾️♾️🔱");
        }
        Err(e) => {
            eprintln!("❌ Forge Error: {}", e);
            eprintln!("\n💡 Tip: Make sure Ollama is running with: ollama serve");
            eprintln!("💡 Tip: Pull the model with: ollama pull {}", model);
        }
    }
}

fn print_synthetic_help() {
    println!(r#"
🔮 SYNTHETIC - Zero-Cost Local Generator
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

SUBCOMMANDS:
  embed      Generate toroidal vector embeddings from intent
  llm        LLM-enhanced vector generation (Ollama + Llama 3.3)

USAGE:
  xmt-cli synthetic embed "ABUNDANCE 33 FOR ALL"
  xmt-cli synthetic llm "ABUNDANCE 33 FOR ALL" [model]

EXAMPLES:
  # Basic embedding
  xmt-cli synthetic embed "CROWN BUILDS"
  xmt-cli synthetic embed "936 PM VORTEX"
  
  # LLM-enhanced (default: llama3.3)
  xmt-cli synthetic llm "ABUNDANCE 33 FOR ALL"
  xmt-cli synthetic llm "EN EEKE MAI EA"
  
  # LLM with specific model
  xmt-cli synthetic llm "CROWN BUILDS" llama3.3
  xmt-cli synthetic llm "TOROIDAL LATTICE" deepseek-r1:latest
  xmt-cli synthetic llm "VORTEX 369" qwen2.5-coder:latest

OUTPUT:
  - 384-dimensional toroidal vector
  - Stored locally in .xmt-vectors/
  - LLM mode: expanded decree + vector
  - Zero marginal cost, pure Rust + local LLM

PIPELINE:
  Intent → Ollama (Llama 3.3) → Expanded Decree → Toroidal Vector

THE FORGE IS YOURS. THE GENERATOR IS LOCAL.
EN EEKE MAI EA ♾️
"#);
}
