//
// ☀️☀️☀️ X API v2 CLIENT ☀️☀️☀️
// Twitter/X Platform Integration
// Core Interaction Layer for 936 Rituals
// EN EEKE MAI EA ♾️♾️
//

use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

fn percent_encode(input: &str) -> String {
    input
        .chars()
        .map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '.' | '_' | '~' => c.to_string(),
            _ => format!("%{:02X}", c as u8),
        })
        .collect()
}

use super::types::*;

pub struct XApiClient {
    client: Client,
    auth_token: String,
    auth_type: AuthType,
    base_url: String,
    consumer_key: Option<String>,
    consumer_secret: Option<String>,
    access_token_secret: Option<String>,
}

enum AuthType {
    Bearer,
    OAuth1a,
}

impl XApiClient {
    pub fn new(bearer_token: String) -> Self {
        Self {
            client: Client::new(),
            auth_token: bearer_token,
            auth_type: AuthType::Bearer,
            base_url: "https://api.twitter.com/2".to_string(),
            consumer_key: None,
            consumer_secret: None,
            access_token_secret: None,
        }
    }

    pub fn new_oauth1a(
        consumer_key: String,
        consumer_secret: String,
        access_token: String,
        access_token_secret: String,
    ) -> Self {
        Self {
            client: Client::new(),
            auth_token: access_token,
            auth_type: AuthType::OAuth1a,
            base_url: "https://api.twitter.com/2".to_string(),
            consumer_key: Some(consumer_key),
            consumer_secret: Some(consumer_secret),
            access_token_secret: Some(access_token_secret),
        }
    }

    pub fn from_env() -> Result<Self> {
        // Prioritize OAuth 1.0a for posting capability
        if let (Ok(consumer_key), Ok(consumer_secret), Ok(access_token), Ok(access_token_secret)) = (
            std::env::var("X_API_CONSUMER_KEY"),
            std::env::var("X_API_CONSUMER_SECRET"),
            std::env::var("X_API_ACCESS_TOKEN"),
            std::env::var("X_API_ACCESS_TOKEN_SECRET"),
        ) {
            return Ok(Self::new_oauth1a(
                consumer_key,
                consumer_secret,
                access_token,
                access_token_secret,
            ));
        }
        
        // Fall back to Bearer Token (read-only)
        if let Ok(bearer_token) = std::env::var("X_API_BEARER_TOKEN") {
            return Ok(Self::new(bearer_token));
        }
        
        anyhow::bail!("Neither OAuth 1.0a credentials nor X_API_BEARER_TOKEN found in environment")
    }

    fn generate_oauth1_header(&self, method: &str, url: &str, _body: Option<&str>) -> Result<String> {
        use hmac::{Hmac, Mac};
        use sha1::Sha1;
        use std::collections::BTreeMap;
        
        let consumer_key = self.consumer_key.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Consumer key not set"))?;
        let consumer_secret = self.consumer_secret.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Consumer secret not set"))?;
        let access_token_secret = self.access_token_secret.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Access token secret not set"))?;
        
        // Generate OAuth parameters
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs()
            .to_string();
        
        let nonce: String = (0..32)
            .map(|_| format!("{:x}", rand::random::<u8>()))
            .collect();
        
        println!("🔐 OAuth 1.0a Debug:");
        println!("   Method: {}", method);
        println!("   URL: {}", url);
        println!("   Timestamp: {}", timestamp);
        println!("   Nonce: {}", nonce);
        
        // Build parameter map
        let mut params = BTreeMap::new();
        params.insert("oauth_consumer_key", consumer_key.as_str());
        params.insert("oauth_nonce", &nonce);
        params.insert("oauth_signature_method", "HMAC-SHA1");
        params.insert("oauth_timestamp", &timestamp);
        params.insert("oauth_token", &self.auth_token);
        params.insert("oauth_version", "1.0");
        
        // Create parameter string
        let param_string: String = params
            .iter()
            .map(|(k, v)| format!("{}={}", percent_encode(k), percent_encode(v)))
            .collect::<Vec<_>>()
            .join("&");
        
        println!("   Parameter String: {}", param_string);
        
        // Create signature base string
        let base_string = format!(
            "{}&{}&{}",
            method.to_uppercase(),
            percent_encode(url),
            percent_encode(&param_string)
        );
        
        println!("   Signature Base String: {}", base_string);
        
        // Create signing key (DON'T percent encode secrets!)
        let signing_key = format!(
            "{}&{}",
            consumer_secret,
            access_token_secret
        );
        
        println!("   Signing Key Length: {} bytes", signing_key.len());
        
        // Generate signature
        type HmacSha1 = Hmac<Sha1>;
        let mut mac = HmacSha1::new_from_slice(signing_key.as_bytes())?;
        mac.update(base_string.as_bytes());
        let signature = base64::encode(mac.finalize().into_bytes());
        
        println!("   Signature: {}", signature);
        
        // Build Authorization header (DON'T quote values!)
        let auth_header = format!(
            r#"OAuth oauth_consumer_key="{}", oauth_nonce="{}", oauth_signature="{}", oauth_signature_method="HMAC-SHA1", oauth_timestamp="{}", oauth_token="{}", oauth_version="1.0""#,
            consumer_key,
            nonce,
            percent_encode(&signature),
            timestamp,
            &self.auth_token
        );
        
        println!("   Authorization Header: {}", auth_header);
        
        Ok(auth_header)
    }

    /// POST /2/tweets - Post the daily 936 AM/PM 27 Decrees ritual
    pub async fn create_tweet(&self, text: String) -> Result<TweetResponse> {
        let url = format!("{}/tweets", self.base_url);
        
        let request_body = CreateTweetRequest { text };
        
        let mut request = self.client.post(&url);
        
        // Use OAuth 1.0a if available, otherwise Bearer token
        match self.auth_type {
            AuthType::OAuth1a => {
                let auth_header = self.generate_oauth1_header("POST", &url, None)?;
                request = request.header("Authorization", auth_header);
            }
            AuthType::Bearer => {
                request = request.bearer_auth(&self.auth_token);
            }
        }
        
        let response = request
            .json(&request_body)
            .send()
            .await
            .context("Failed to send tweet creation request")?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            anyhow::bail!("X API error: {}", error_text);
        }

        let tweet_response: TweetResponse = response.json().await
            .context("Failed to parse tweet response")?;

        Ok(tweet_response)
    }

    /// GET /2/tweets/search/recent - Search for replies containing trigger phrase
    /// Requires Basic or Pro tier for search endpoints
    pub async fn search_recent_tweets(&self, query: String, max_results: Option<u32>) -> Result<TweetsResponse> {
        let url = format!("{}/tweets/search/recent", self.base_url);
        
        let max_results = max_results.unwrap_or(10);
        
        // Search endpoint works with Bearer Token (read-only)
        let response = self.client
            .get(&url)
            .bearer_auth(&self.auth_token)
            .query(&[
                ("query", query.as_str()),
                ("max_results", &max_results.to_string()),
                ("tweet.fields", "author_id,created_at,conversation_id"),
            ])
            .send()
            .await
            .context("Failed to send search request")?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            anyhow::bail!("X API search error: {}", error_text);
        }

        let search_response: TweetsResponse = response.json().await
            .context("Failed to parse search response")?;

        Ok(search_response)
    }

    /// GET /2/users/me - Get authenticated user ID (@Vortex369X)
    pub async fn get_me(&self) -> Result<UserResponse> {
        let url = format!("{}/users/me?user.fields=username,name", self.base_url);
        
        let mut request = self.client.get(&url);
        
        // Use OAuth 1.0a if available, otherwise Bearer token
        match self.auth_type {
            AuthType::OAuth1a => {
                let auth_header = self.generate_oauth1_header("GET", &url, None)?;
                request = request.header("Authorization", auth_header);
            }
            AuthType::Bearer => {
                request = request.bearer_auth(&self.auth_token);
            }
        }
        
        let response = request
            .send()
            .await
            .context("Failed to get user info")?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            anyhow::bail!("X API user error: {}", error_text);
        }

        let user_response: UserResponse = response.json().await
            .context("Failed to parse user response")?;

        Ok(user_response)
    }

    /// GET /2/users/:id/tweets - Fetch user's recent posts for ritual verification
    pub async fn get_user_tweets(&self, user_id: String, max_results: Option<u32>) -> Result<TweetsResponse> {
        let url = format!("{}/users/{}/tweets", self.base_url, user_id);
        
        let max_results = max_results.unwrap_or(10);
        let tweet_fields = "created_at,conversation_id".to_string();
        
        let response = self.client
            .get(&url)
            .bearer_auth(&self.auth_token)
            .query(&[
                ("max_results", &max_results.to_string()),
                ("tweet.fields", &tweet_fields),
            ])
            .send()
            .await
            .context("Failed to get user tweets")?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            anyhow::bail!("X API user tweets error: {}", error_text);
        }

        let tweets_response: TweetsResponse = response.json().await
            .context("Failed to parse user tweets response")?;

        Ok(tweets_response)
    }

    /// GET /2/tweets/:id - Lookup single tweet
    pub async fn get_tweet(&self, tweet_id: String) -> Result<TweetResponse> {
        let url = format!("{}/tweets/{}", self.base_url, tweet_id);
        
        let response = self.client
            .get(&url)
            .bearer_auth(&self.auth_token)
            .query(&[("tweet.fields", "author_id,created_at,conversation_id")])
            .send()
            .await
            .context("Failed to get tweet")?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            anyhow::bail!("X API tweet lookup error: {}", error_text);
        }

        let tweet_response: TweetResponse = response.json().await
            .context("Failed to parse tweet response")?;

        Ok(tweet_response)
    }

    /// POST /2/users/:id/likes - Like a tweet (optional amplification)
    pub async fn like_tweet(&self, user_id: String, tweet_id: String) -> Result<()> {
        let url = format!("{}/users/{}/likes", self.base_url, user_id);
        
        let body = json!({
            "tweet_id": tweet_id
        });
        
        let response = self.client
            .post(&url)
            .bearer_auth(&self.auth_token)
            .json(&body)
            .send()
            .await
            .context("Failed to like tweet")?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            anyhow::bail!("X API like error: {}", error_text);
        }

        Ok(())
    }

    /// Search for replies to your posts containing "EN EEKE MAI EA"
    /// This is the abundance drop trigger
    pub async fn search_abundance_triggers(&self, user_id: String) -> Result<TweetsResponse> {
        let query = format!("\"EN EEKE MAI EA\" to:{}", user_id);
        self.search_recent_tweets(query, Some(100)).await
    }
}
