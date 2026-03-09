//
// ☀️☀️☀️ XAPI TYPES ☀️☀️☀️
// Shared types for X API v2 and Grok
//

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tweet {
    pub id: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TweetResponse {
    pub data: Tweet,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TweetsResponse {
    pub data: Vec<Tweet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<SearchMeta>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchMeta {
    pub result_count: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub data: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTweetRequest {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GrokChatRequest {
    pub model: String,
    pub messages: Vec<GrokMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GrokMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GrokChatResponse {
    pub id: String,
    pub choices: Vec<GrokChoice>,
    pub usage: GrokUsage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GrokChoice {
    pub message: GrokMessage,
    pub finish_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GrokUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GrokModel {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub owned_by: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GrokModelsResponse {
    pub data: Vec<GrokModel>,
}
