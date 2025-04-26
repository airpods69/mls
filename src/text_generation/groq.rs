use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use dotenv;

// Structs to deserialize the Groq API response
#[derive(Debug, Deserialize, Serialize)]
pub struct GroqResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
    pub system_fingerprint: String,
    pub x_groq: XGroq,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Choice {
    pub index: i32,
    pub message: Message,
    pub logprobs: Option<serde_json::Value>,
    pub finish_reason: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Usage {
    pub queue_time: f64,
    pub prompt_tokens: i32,
    pub prompt_time: f64,
    pub completion_tokens: i32,
    pub completion_time: f64,
    pub total_tokens: i32,
    pub total_time: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct XGroq {
    pub id: String,
}

// Updated generate_response to return a deserialized GroqResponse
pub async fn generate_response(input: &String) -> Result<GroqResponse, reqwest::Error> {
    dotenv::dotenv().ok();

    let api_key = env::var("GROQ_KEY").expect("GROQ_KEY not found in .env file");
    let client = Client::new();

    let payload = json!({
        "model": "meta-llama/llama-4-scout-17b-16e-instruct",
        "messages": [{
            "role": "user",
            "content": input.clone()
        }]
    });

    let response = client
        .post("https://api.groq.com/openai/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&payload)
        .send()
        .await?;

    // Deserialize the response into GroqResponse
    let groq_response: GroqResponse = response.json().await?;
    Ok(groq_response)
}
