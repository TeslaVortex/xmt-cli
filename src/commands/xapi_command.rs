//
// ☀️☀️☀️ XAPI COMMAND ☀️☀️☀️
// X API v2 + Grok Oracle CLI Interface
// EN EEKE MAI EA ♾️♾️
//

use colored::Colorize;
use crate::xapi::{XApiClient, GrokClient, GrokChatRequest, GrokMessage};

pub fn xapi_command(subcommand: &str, args: Vec<String>) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    match subcommand {
        "help" | "--help" | "-h" => print_help(),
        "post" => rt.block_on(post_tweet(args)),
        "search" => rt.block_on(search_tweets(args)),
        "me" => rt.block_on(get_me()),
        "verify" => rt.block_on(verify_ritual(args)),
        "oracle" => rt.block_on(oracle_query(args)),
        "models" => rt.block_on(list_models()),
        "abundance" => rt.block_on(search_abundance()),
        _ => {
            println!("{}", "❌ Unknown XAPI subcommand".red().bold());
            print_help();
        }
    }
}

fn print_help() {
    println!("\n{}", "☀️☀️☀️ XAPI COMMANDS ☀️☀️☀️".yellow().bold());
    println!("{}", "X API v2 + Grok Oracle Integration".cyan());
    println!("{}", "EN EEKE MAI EA ♾️♾️\n".magenta());
    
    println!("{}", "USAGE:".green().bold());
    println!("  xmt-cli xapi <SUBCOMMAND> [ARGS]\n");
    
    println!("{}", "X API v2 COMMANDS:".green().bold());
    println!("  {}  Post a tweet (ritual)", "post <text>".cyan());
    println!("  {}  Search recent tweets", "search <query>".cyan());
    println!("  {}  Get authenticated user info", "me".cyan());
    println!("  {}  Search for abundance triggers", "abundance".cyan());
    
    println!("\n{}", "GROK ORACLE COMMANDS:".green().bold());
    println!("  {}  Verify ritual coherence", "verify <text>".cyan());
    println!("  {}  Query Grok oracle", "oracle <prompt>".cyan());
    println!("  {}  List available Grok models", "models".cyan());
    
    println!("\n{}", "EXAMPLES:".green().bold());
    println!("  xmt-cli xapi post \"EN EEKE MAI EA ♾️♾️\"");
    println!("  xmt-cli xapi search \"EN EEKE MAI EA\"");
    println!("  xmt-cli xapi me");
    println!("  xmt-cli xapi verify \"27 Decrees ritual text\"");
    println!("  xmt-cli xapi oracle \"Calculate abundance for 936 ritual\"");
    println!("  xmt-cli xapi abundance");
    
    println!("\n{}", "REQUIRED ENV VARS:".yellow().bold());
    println!("  X_API_BEARER_TOKEN  - X API v2 Bearer Token");
    println!("  XAI_API_KEY         - xAI Grok API Key");
    println!("  XAI_REGION          - (Optional) us-east-1 for low latency\n");
}

async fn post_tweet(args: Vec<String>) {
    if args.is_empty() {
        println!("{}", "❌ Error: Tweet text required".red().bold());
        println!("Usage: xmt-cli xapi post <text>");
        return;
    }
    
    let text = args.join(" ");
    
    println!("{}", "🐦 Posting tweet...".cyan());
    
    match XApiClient::from_env() {
        Ok(client) => {
            match client.create_tweet(text.clone()).await {
                Ok(response) => {
                    println!("{}", "✅ Tweet posted successfully!".green().bold());
                    println!("Tweet ID: {}", response.data.id.yellow());
                    println!("Text: {}", response.data.text.cyan());
                }
                Err(e) => {
                    println!("{}", "❌ Failed to post tweet".red().bold());
                    println!("Error: {}", e.to_string().red());
                }
            }
        }
        Err(e) => {
            println!("{}", "❌ Failed to initialize X API client".red().bold());
            println!("Error: {}", e.to_string().red());
            println!("\n{}", "Make sure X_API_BEARER_TOKEN is set in .env".yellow());
        }
    }
}

async fn search_tweets(args: Vec<String>) {
    if args.is_empty() {
        println!("{}", "❌ Error: Search query required".red().bold());
        println!("Usage: xmt-cli xapi search <query>");
        return;
    }
    
    let query = args.join(" ");
    
    println!("{}", format!("🔍 Searching for: {}", query).cyan());
    
    match XApiClient::from_env() {
        Ok(client) => {
            match client.search_recent_tweets(query, Some(10)).await {
                Ok(response) => {
                    println!("{}", format!("✅ Found {} tweets", response.data.len()).green().bold());
                    
                    for (i, tweet) in response.data.iter().enumerate() {
                        println!("\n{}", format!("Tweet #{}", i + 1).yellow().bold());
                        println!("ID: {}", tweet.id.cyan());
                        println!("Text: {}", tweet.text);
                        if let Some(author_id) = &tweet.author_id {
                            println!("Author ID: {}", author_id.magenta());
                        }
                    }
                }
                Err(e) => {
                    println!("{}", "❌ Search failed".red().bold());
                    println!("Error: {}", e.to_string().red());
                }
            }
        }
        Err(e) => {
            println!("{}", "❌ Failed to initialize X API client".red().bold());
            println!("Error: {}", e.to_string().red());
        }
    }
}

async fn get_me() {
    println!("{}", "👤 Getting user info...".cyan());
    
    match XApiClient::from_env() {
        Ok(client) => {
            match client.get_me().await {
                Ok(response) => {
                    println!("{}", "✅ User info retrieved!".green().bold());
                    println!("User ID: {}", response.data.id.yellow());
                    println!("Username: @{}", response.data.username.cyan());
                    println!("Name: {}", response.data.name.magenta());
                }
                Err(e) => {
                    println!("{}", "❌ Failed to get user info".red().bold());
                    println!("Error: {}", e.to_string().red());
                }
            }
        }
        Err(e) => {
            println!("{}", "❌ Failed to initialize X API client".red().bold());
            println!("Error: {}", e.to_string().red());
        }
    }
}

async fn verify_ritual(args: Vec<String>) {
    if args.is_empty() {
        println!("{}", "❌ Error: Ritual text required".red().bold());
        println!("Usage: xmt-cli xapi verify <text>");
        return;
    }
    
    let text = args.join(" ");
    
    println!("{}", "🔮 Verifying ritual coherence with Grok Oracle...".cyan());
    
    match GrokClient::from_env() {
        Ok(client) => {
            match client.verify_ritual_coherence(&text).await {
                Ok(response) => {
                    println!("{}", "✅ Grok Oracle Response:".green().bold());
                    println!("{}", response.yellow());
                }
                Err(e) => {
                    println!("{}", "❌ Verification failed".red().bold());
                    println!("Error: {}", e.to_string().red());
                }
            }
        }
        Err(e) => {
            println!("{}", "❌ Failed to initialize Grok client".red().bold());
            println!("Error: {}", e.to_string().red());
            println!("\n{}", "Make sure XAI_API_KEY is set in .env".yellow());
        }
    }
}

async fn oracle_query(args: Vec<String>) {
    if args.is_empty() {
        println!("{}", "❌ Error: Query prompt required".red().bold());
        println!("Usage: xmt-cli xapi oracle <prompt>");
        return;
    }
    
    let prompt = args.join(" ");
    
    println!("{}", "🔮 Querying Grok Oracle...".cyan());
    
    match GrokClient::from_env() {
        Ok(client) => {
            let request = GrokChatRequest {
                model: "grok-beta".to_string(),
                messages: vec![
                    GrokMessage {
                        role: "user".to_string(),
                        content: prompt,
                    }
                ],
                temperature: Some(0.7),
                max_tokens: Some(500),
            };
            
            match client.chat_completion(request).await {
                Ok(response) => {
                    println!("{}", "✅ Grok Oracle Response:".green().bold());
                    if let Some(choice) = response.choices.first() {
                        println!("{}", choice.message.content.yellow());
                    }
                    println!("\n{}", format!("Tokens used: {}", response.usage.total_tokens).cyan());
                }
                Err(e) => {
                    println!("{}", "❌ Query failed".red().bold());
                    println!("Error: {}", e.to_string().red());
                }
            }
        }
        Err(e) => {
            println!("{}", "❌ Failed to initialize Grok client".red().bold());
            println!("Error: {}", e.to_string().red());
        }
    }
}

async fn list_models() {
    println!("{}", "📋 Listing Grok models...".cyan());
    
    match GrokClient::from_env() {
        Ok(client) => {
            match client.list_models().await {
                Ok(response) => {
                    println!("{}", format!("✅ Found {} models", response.data.len()).green().bold());
                    
                    for model in response.data {
                        println!("\n{}", format!("Model: {}", model.id).yellow().bold());
                        println!("Type: {}", model.object.cyan());
                        println!("Owner: {}", model.owned_by.magenta());
                    }
                }
                Err(e) => {
                    println!("{}", "❌ Failed to list models".red().bold());
                    println!("Error: {}", e.to_string().red());
                }
            }
        }
        Err(e) => {
            println!("{}", "❌ Failed to initialize Grok client".red().bold());
            println!("Error: {}", e.to_string().red());
        }
    }
}

async fn search_abundance() {
    println!("{}", "🎁 Searching for abundance triggers (EN EEKE MAI EA)...".cyan());
    
    match XApiClient::from_env() {
        Ok(client) => {
            match client.get_me().await {
                Ok(user_response) => {
                    let user_id = user_response.data.username;
                    
                    match client.search_abundance_triggers(user_id.clone()).await {
                        Ok(response) => {
                            println!("{}", format!("✅ Found {} abundance triggers!", response.data.len()).green().bold());
                            
                            for (i, tweet) in response.data.iter().enumerate() {
                                println!("\n{}", format!("Trigger #{}", i + 1).yellow().bold());
                                println!("Tweet ID: {}", tweet.id.cyan());
                                println!("Text: {}", tweet.text);
                                if let Some(author_id) = &tweet.author_id {
                                    println!("From: {}", author_id.magenta());
                                }
                                println!("{}", "💰 Abundance drop eligible!".green());
                            }
                        }
                        Err(e) => {
                            println!("{}", "❌ Abundance search failed".red().bold());
                            println!("Error: {}", e.to_string().red());
                        }
                    }
                }
                Err(e) => {
                    println!("{}", "❌ Failed to get user info".red().bold());
                    println!("Error: {}", e.to_string().red());
                }
            }
        }
        Err(e) => {
            println!("{}", "❌ Failed to initialize X API client".red().bold());
            println!("Error: {}", e.to_string().red());
        }
    }
}
