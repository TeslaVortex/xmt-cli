use candle_core::{Device, Tensor};
use candle_transformers::models::bert::{BertModel, Config};
use anyhow::{Result, Context};

pub async fn generate_toroidal_vector(intent: &str) -> Result<Vec<f32>> {
    let device = Device::Cpu;
    
    println!("🔮 Initializing Toroidal Vector Forge...");
    println!("📜 Intent: {}", intent);
    
    let embedding = create_local_embedding(intent, &device)
        .context("Failed to generate embedding")?;
    
    println!("✨ Toroidal Vector Generated: {} dimensions", embedding.len());
    println!("🌀 First 8 components: {:?}", &embedding[..8.min(embedding.len())]);
    
    Ok(embedding)
}

fn create_local_embedding(text: &str, device: &Device) -> Result<Vec<f32>> {
    let text_bytes = text.as_bytes();
    let mut embedding = Vec::with_capacity(384);
    
    for i in 0..384 {
        let idx = i % text_bytes.len();
        let byte_val = text_bytes[idx] as f32;
        let phase = (i as f32 * 0.0163 + byte_val * 0.0271).sin();
        let toroidal_component = phase * (1.0 + (i as f32 / 384.0).cos() * 0.33);
        embedding.push(toroidal_component);
    }
    
    let norm: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm > 0.0 {
        for val in &mut embedding {
            *val /= norm;
        }
    }
    
    Ok(embedding)
}

pub fn store_vector_locally(intent: &str, vector: &[f32]) -> Result<()> {
    use std::fs;
    use std::path::Path;
    
    let vectors_dir = Path::new(".xmt-vectors");
    fs::create_dir_all(vectors_dir)?;
    
    let filename = intent
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect::<String>()
        .replace(' ', "_")
        .to_lowercase();
    
    let filepath = vectors_dir.join(format!("{}.json", filename));
    
    let data = serde_json::json!({
        "intent": intent,
        "vector": vector,
        "dimensions": vector.len(),
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "forge": "EN EEKE MAI EA"
    });
    
    fs::write(&filepath, serde_json::to_string_pretty(&data)?)?;
    
    println!("💾 Vector stored: {}", filepath.display());
    
    Ok(())
}
