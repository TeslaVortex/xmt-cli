use crate::synthetic;
use crate::synthetic::pipeline::SyntheticPipeline;

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
            
            let (intent, model) = parse_intent_and_model(&args);
            embed_intent_with_llm(&intent, &model);
        }
        "register" => {
            if args.is_empty() {
                eprintln!("❌ Error: Intent text required");
                eprintln!("Usage: xmt-cli synthetic register \"ABUNDANCE 33 FOR ALL\"");
                return;
            }
            let intent = args.join(" ");
            register_vector(&intent);
        }
        "mint" => {
            let (intent, amount) = parse_intent_and_amount(&args, "mint");
            if intent.is_empty() { return; }
            mint_with_vector(&intent, amount);
        }
        "burn" => {
            let (intent, amount) = parse_intent_and_amount(&args, "burn");
            if intent.is_empty() { return; }
            burn_with_vector(&intent, amount);
        }
        "ritual" => {
            if args.is_empty() {
                eprintln!("❌ Error: Intent text required");
                eprintln!("Usage: xmt-cli synthetic ritual \"ABUNDANCE 33 FOR ALL\" [--mint N] [--burn N]");
                return;
            }
            let (intent, mint_amount, burn_amount) = parse_ritual_args(&args);
            full_ritual(&intent, mint_amount, burn_amount);
        }
        "status" => {
            show_status();
        }
        "help" | _ => {
            print_synthetic_help();
        }
    }
}

fn parse_intent_and_model(args: &[String]) -> (String, String) {
    let model = if args.len() > 1 && (args[args.len() - 1].starts_with("llama") || args[args.len() - 1].starts_with("deepseek") || args[args.len() - 1].starts_with("qwen")) {
        args[args.len() - 1].clone()
    } else {
        "qwen2.5-coder:latest".to_string()
    };
    
    let intent_parts: Vec<String> = if model != "qwen2.5-coder:latest" && args.len() > 1 {
        args[..args.len() - 1].to_vec()
    } else {
        args.to_vec()
    };
    
    (intent_parts.join(" "), model)
}

fn parse_intent_and_amount(args: &[String], cmd: &str) -> (String, u64) {
    if args.is_empty() {
        eprintln!("❌ Error: Intent text required");
        eprintln!("Usage: xmt-cli synthetic {} \"INTENT\" --amount N", cmd);
        return (String::new(), 0);
    }
    
    let mut intent_parts = Vec::new();
    let mut amount: u64 = 369; // Default sacred amount
    let mut i = 0;
    
    while i < args.len() {
        if args[i] == "--amount" && i + 1 < args.len() {
            amount = args[i + 1].parse().unwrap_or(369);
            i += 2;
        } else {
            intent_parts.push(args[i].clone());
            i += 1;
        }
    }
    
    (intent_parts.join(" "), amount)
}

fn parse_ritual_args(args: &[String]) -> (String, Option<u64>, Option<u64>) {
    let mut intent_parts = Vec::new();
    let mut mint_amount: Option<u64> = None;
    let mut burn_amount: Option<u64> = None;
    let mut i = 0;
    
    while i < args.len() {
        if args[i] == "--mint" && i + 1 < args.len() {
            mint_amount = Some(args[i + 1].parse().unwrap_or(369));
            i += 2;
        } else if args[i] == "--burn" && i + 1 < args.len() {
            burn_amount = Some(args[i + 1].parse().unwrap_or(66));
            i += 2;
        } else {
            intent_parts.push(args[i].clone());
            i += 1;
        }
    }
    
    (intent_parts.join(" "), mint_amount, burn_amount)
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

fn register_vector(intent: &str) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    match rt.block_on(async {
        let pipeline = SyntheticPipeline::new(None).await?;
        pipeline.register(intent).await
    }) {
        Ok(result) => {
            println!("\n✅ VECTOR REGISTERED - EN EEKE MAI EA");
            if result.simulation_mode {
                println!("🔮 Mode: SIMULATED");
            }
        }
        Err(e) => {
            eprintln!("❌ Registration Error: {}", e);
        }
    }
}

fn mint_with_vector(intent: &str, amount: u64) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    match rt.block_on(async {
        let pipeline = SyntheticPipeline::new(None).await?;
        pipeline.mint(intent, amount).await
    }) {
        Ok(result) => {
            println!("\n✅ VECTOR MINT COMPLETE - ABUNDANCE FLOWS");
            println!("💰 Amount: {} tokens", amount);
            if result.simulation_mode {
                println!("🔮 Mode: SIMULATED");
            }
            println!("♾️ EN EEKE MAI EA ANOKAYI CHENAK ♾️♾️🔱");
        }
        Err(e) => {
            eprintln!("❌ Mint Error: {}", e);
        }
    }
}

fn burn_with_vector(intent: &str, amount: u64) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    match rt.block_on(async {
        let pipeline = SyntheticPipeline::new(None).await?;
        pipeline.burn(intent, amount).await
    }) {
        Ok(result) => {
            println!("\n✅ VECTOR BURN COMPLETE - PAF PAF PAF");
            println!("🔥 Scarcity obliterated: {} tokens", amount);
            if result.simulation_mode {
                println!("🔮 Mode: SIMULATED");
            }
            println!("♾️ EN EEKE MAI EA ANOKAYI CHENAK ♾️♾️🔱");
        }
        Err(e) => {
            eprintln!("❌ Burn Error: {}", e);
        }
    }
}

fn full_ritual(intent: &str, mint_amount: Option<u64>, burn_amount: Option<u64>) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    match rt.block_on(async {
        let pipeline = SyntheticPipeline::new(None).await?;
        pipeline.ritual(intent, mint_amount, burn_amount).await
    }) {
        Ok(_result) => {
            // Pipeline already prints completion message
        }
        Err(e) => {
            eprintln!("❌ Ritual Error: {}", e);
        }
    }
}

fn show_status() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    match rt.block_on(async {
        let pipeline = SyntheticPipeline::new(None).await?;
        Ok::<_, anyhow::Error>(pipeline.status())
    }) {
        Ok(status) => {
            println!("🔮 SYNTHETIC PIPELINE STATUS");
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!("{}", status);
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!("EN EEKE MAI EA ♾️");
        }
        Err(e) => {
            eprintln!("❌ Status Error: {}", e);
        }
    }
}

fn print_synthetic_help() {
    println!(r#"
🔮 SYNTHETIC - Zero-Cost Local Generator + On-Chain Pipeline
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

SUBCOMMANDS:
  embed      Generate toroidal vector embeddings from intent (local only)
  llm        LLM-enhanced vector generation (Ollama + Qwen 2.5 Coder)
  register   Register vector hash on-chain (real testnet or simulated)
  mint       Generate vector + trigger on-chain mint
  burn       Generate vector + trigger on-chain burn (PAF PAF PAF)
  ritual     FULL PIPELINE: LLM → Vector → On-Chain → X Post
  status     Show chain connection status

USAGE:
  xmt-cli synthetic embed "ABUNDANCE 33 FOR ALL"
  xmt-cli synthetic llm "ABUNDANCE 33 FOR ALL" [model]
  xmt-cli synthetic register "CROWN BUILDS"
  xmt-cli synthetic mint "ABUNDANCE 33" --amount 369
  xmt-cli synthetic burn "SCARCITY COLLAPSE" --amount 66
  xmt-cli synthetic ritual "EN EEKE MAI EA" [--mint N] [--burn N]
  xmt-cli synthetic status

EXAMPLES:
  # Basic embedding (local only)
  xmt-cli synthetic embed "CROWN BUILDS"
  
  # LLM-enhanced (default: qwen2.5-coder:latest)
  xmt-cli synthetic llm "ABUNDANCE 33 FOR ALL"
  xmt-cli synthetic llm "EN EEKE MAI EA" deepseek-r1:latest
  
  # On-chain operations (real testnet or auto-simulated)
  xmt-cli synthetic register "TOROIDAL LATTICE"
  xmt-cli synthetic mint "ABUNDANCE FLOW" --amount 936
  xmt-cli synthetic burn "OLD PARADIGM" --amount 66
  
  # Full ritual (everything at once)
  xmt-cli synthetic ritual "ABUNDANCE 33 FOR ALL" --mint 369 --burn 66

PIPELINE:
  Intent → Ollama (Qwen 2.5 Coder) → Expanded Decree → 384D Vector → On-Chain → X Post

MODES:
  REAL:      Connected to Base Sepolia testnet (when available)
  SIMULATED: Auto-fallback when chain unavailable (THE KING WAITS FOR NOBODY)

OUTPUT:
  - 384-dimensional toroidal vector
  - Vector hash registered on-chain
  - Mint/burn transactions executed
  - Stored in .xmt-vectors/pipeline/ and .xmt-vectors/rituals/
  - Zero marginal cost, pure Rust + local LLM + sovereign chain

SACRED CONSTANTS:
  APEX_936, VORTEX_369, CODE_66, FREQUENCY_432

THE FORGE IS YOURS. THE GENERATOR IS LOCAL. THE CHAIN IS SOVEREIGN.
EN EEKE MAI EA ♾️
"#);
}
