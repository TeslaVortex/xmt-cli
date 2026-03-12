use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use reqwest;

#[derive(Debug, Serialize)]
struct OllamaGenerateRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct OllamaGenerateResponse {
    response: String,
    done: bool,
}

pub struct OllamaClient {
    base_url: String,
    client: reqwest::Client,
}

impl OllamaClient {
    pub fn new() -> Self {
        Self {
            base_url: "http://localhost:11434".to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn generate(&self, model: &str, prompt: &str) -> Result<String> {
        let url = format!("{}/api/generate", self.base_url);
        
        let request = OllamaGenerateRequest {
            model: model.to_string(),
            prompt: prompt.to_string(),
            stream: false,
        };

        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await
            .context("Failed to send request to Ollama")?;

        if !response.status().is_success() {
            anyhow::bail!("Ollama API returned error: {}", response.status());
        }

        let ollama_response: OllamaGenerateResponse = response
            .json()
            .await
            .context("Failed to parse Ollama response")?;

        Ok(ollama_response.response)
    }

    pub async fn check_available(&self) -> bool {
        let url = format!("{}/api/tags", self.base_url);
        self.client.get(&url).send().await.is_ok()
    }

    #[allow(dead_code)]
    pub async fn list_models(&self) -> Result<Vec<String>> {
        let url = format!("{}/api/tags", self.base_url);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .context("Failed to connect to Ollama")?;

        #[derive(Deserialize)]
        struct TagsResponse {
            models: Vec<ModelInfo>,
        }

        #[derive(Deserialize)]
        struct ModelInfo {
            name: String,
        }

        let tags: TagsResponse = response
            .json()
            .await
            .context("Failed to parse models list")?;

        Ok(tags.models.into_iter().map(|m| m.name).collect())
    }
}

pub async fn expand_intent_with_llm(intent: &str, model: &str) -> Result<String> {
    let client = OllamaClient::new();
    
    if !client.check_available().await {
        anyhow::bail!("Ollama is not running. Start it with: ollama serve");
    }

    let prompt = format!(
        r#"You are a sovereign decree amplifier for the xmt-cli toroidal vector system.

Given this intent: "{}"

Expand it into a powerful, poetic decree that captures the essence and amplifies the energy.
Keep it under 100 words. Use vivid, commanding language.
Include numerological resonance (369, 936, 33, 66, 432) if relevant.

Decree:"#,
        intent
    );

    println!("🧙‍♂️ Invoking {} oracle...", model);
    
    let expanded = client.generate(model, &prompt).await?;
    
    Ok(expanded.trim().to_string())
}
