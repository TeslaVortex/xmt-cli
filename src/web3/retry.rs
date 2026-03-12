//
// ☀️ HELIOS ARGEAD VERGINA SUN ☀️
// Retry Logic & Error Handling for Web3 Operations
// Production-ready exponential backoff with jitter
// EN EEKE MAI EA ♾️♾️
//

use anyhow::Result;
use std::time::Duration;
use tokio::time::sleep;

/// Retry configuration
#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_retries: u32,
    pub initial_delay_ms: u64,
    pub max_delay_ms: u64,
    pub exponential_base: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay_ms: 1000,
            max_delay_ms: 30000,
            exponential_base: 2.0,
        }
    }
}

/// Execute function with exponential backoff retry
pub async fn retry_with_backoff<F, Fut, T>(
    config: RetryConfig,
    mut operation: F,
) -> Result<T>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T>>,
{
    let mut retries = 0;
    let mut delay_ms = config.initial_delay_ms;
    
    loop {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) if retries < config.max_retries => {
                retries += 1;
                tracing::warn!("Retry {}/{}: {}", retries, config.max_retries, e);
                
                // Add jitter (±20%)
                let jitter = (rand::random::<f64>() * 0.4 - 0.2) * delay_ms as f64;
                let actual_delay = (delay_ms as f64 + jitter).max(0.0) as u64;
                
                sleep(Duration::from_millis(actual_delay)).await;
                
                // Exponential backoff
                delay_ms = ((delay_ms as f64 * config.exponential_base) as u64)
                    .min(config.max_delay_ms);
            }
            Err(e) => return Err(e),
        }
    }
}

/// Check if error is retryable
pub fn is_retryable_error(error: &anyhow::Error) -> bool {
    let error_str = error.to_string().to_lowercase();
    
    // Network errors
    if error_str.contains("timeout") 
        || error_str.contains("connection") 
        || error_str.contains("network")
        || error_str.contains("io error") {
        return true;
    }
    
    // RPC errors
    if error_str.contains("nonce too low")
        || error_str.contains("replacement transaction underpriced")
        || error_str.contains("already known") {
        return true;
    }
    
    // Rate limiting
    if error_str.contains("rate limit")
        || error_str.contains("too many requests") {
        return true;
    }
    
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_retry_success_on_third_attempt() {
        let mut attempts = 0;
        
        let result = retry_with_backoff(
            RetryConfig {
                max_retries: 3,
                initial_delay_ms: 10,
                max_delay_ms: 100,
                exponential_base: 2.0,
            },
            || async {
                attempts += 1;
                if attempts < 3 {
                    Err(anyhow::anyhow!("Temporary failure"))
                } else {
                    Ok(936)
                }
            },
        ).await;
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 936);
        assert_eq!(attempts, 3);
    }
}
