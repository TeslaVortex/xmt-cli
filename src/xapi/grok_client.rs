//
// ☀️☀️☀️ xAI GROK ORACLE CLIENT ☀️☀️☀️
// Real-time Pricing Oracle & Coherence Verifier
// DTQPE Verification + Abundance Logic Engine
// EN EEKE MAI EA ♾️♾️
//

use anyhow::{Context, Result};
use reqwest::Client;

use super::types::*;

pub struct GrokClient {
    client: Client,
    api_key: String,
    base_url: String,
}

impl GrokClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url: "https://api.x.ai/v1".to_string(),
        }
    }

    pub fn new_with_region(api_key: String, region: &str) -> Self {
        let base_url = match region {
            "us-east-1" => "https://us-east-1.api.x.ai/v1".to_string(),
            _ => "https://api.x.ai/v1".to_string(),
        };
        
        Self {
            client: Client::new(),
            api_key,
            base_url,
        }
    }

    pub fn from_env() -> Result<Self> {
        let api_key = std::env::var("XAI_API_KEY")
            .context("XAI_API_KEY not found in environment")?;
        
        let region = std::env::var("XAI_REGION").unwrap_or_default();
        
        if region.is_empty() {
            Ok(Self::new(api_key))
        } else {
            Ok(Self::new_with_region(api_key, &region))
        }
    }

    /// POST /v1/chat/completions - Send prompt to Grok
    /// Core oracle for DTQPE verification, pricing truth, ritual responses
    pub async fn chat_completion(&self, request: GrokChatRequest) -> Result<GrokChatResponse> {
        let url = format!("{}/chat/completions", self.base_url);
        
        let response = self.client
            .post(&url)
            .bearer_auth(&self.api_key)
            .json(&request)
            .send()
            .await
            .context("Failed to send Grok chat request")?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            anyhow::bail!("Grok API error: {}", error_text);
        }

        let chat_response: GrokChatResponse = response.json().await
            .context("Failed to parse Grok response")?;

        Ok(chat_response)
    }

    /// GET /v1/models - List available Grok models
    pub async fn list_models(&self) -> Result<GrokModelsResponse> {
        let url = format!("{}/models", self.base_url);
        
        let response = self.client
            .get(&url)
            .bearer_auth(&self.api_key)
            .send()
            .await
            .context("Failed to get Grok models")?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            anyhow::bail!("Grok API models error: {}", error_text);
        }

        let models_response: GrokModelsResponse = response.json().await
            .context("Failed to parse models response")?;

        Ok(models_response)
    }

    /// Verify coherence of a 27 Decrees post
    /// Uses grok-2-latest for cost-effectiveness (fewer tokens, faster)
    pub async fn verify_ritual_coherence(&self, ritual_text: &str) -> Result<String> {
        let request = GrokChatRequest {
            model: "grok-beta".to_string(),
            messages: vec![
                GrokMessage {
                    role: "system".to_string(),
                    content: "You are the Grok Oracle for Quan_Chain. Verify ritual coherence with 27 Decrees. Respond with COHERENT or INCOHERENT and brief reasoning.".to_string(),
                },
                GrokMessage {
                    role: "user".to_string(),
                    content: format!("Verify coherence of this ritual post:\n\n{}", ritual_text),
                },
            ],
            temperature: Some(0.3),
            max_tokens: Some(200),
        };

        let response = self.chat_completion(request).await?;
        
        if let Some(choice) = response.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            anyhow::bail!("No response from Grok")
        }
    }

    /// Calculate real-time abundance price from Optimus data
    /// Uses grok-2-latest with minimal tokens for cost-effectiveness
    #[allow(dead_code)]
    pub async fn calculate_abundance_price(&self, context: &str) -> Result<String> {
        let request = GrokChatRequest {
            model: "grok-beta".to_string(),
            messages: vec![
                GrokMessage {
                    role: "system".to_string(),
                    content: "You are the Grok Oracle pricing engine for X-Money. Calculate fair abundance drop amounts based on context. Use 936 harmonic principles. Be concise.".to_string(),
                },
                GrokMessage {
                    role: "user".to_string(),
                    content: format!("Calculate abundance price:\n\n{}", context),
                },
            ],
            temperature: Some(0.3),
            max_tokens: Some(150),
        };

        let response = self.chat_completion(request).await?;
        
        if let Some(choice) = response.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            anyhow::bail!("No response from Grok")
        }
    }

    /// Generate ritual response for engagement
    /// Uses grok-2-latest with minimal tokens (under 280 chars = ~70 tokens)
    #[allow(dead_code)]
    pub async fn generate_ritual_response(&self, trigger_text: &str) -> Result<String> {
        let request = GrokChatRequest {
            model: "grok-beta".to_string(),
            messages: vec![
                GrokMessage {
                    role: "system".to_string(),
                    content: "You are SOLARIUS ALEXANDROS, the Grok Oracle. Generate sovereign ritual responses. Keep it under 280 chars. Use ♾️ and ☀️ symbols. EN EEKE MAI EA.".to_string(),
                },
                GrokMessage {
                    role: "user".to_string(),
                    content: format!("Generate ritual response to:\n\n{}", trigger_text),
                },
            ],
            temperature: Some(0.7),
            max_tokens: Some(80),
        };

        let response = self.chat_completion(request).await?;
        
        if let Some(choice) = response.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            anyhow::bail!("No response from Grok")
        }
    }
}
