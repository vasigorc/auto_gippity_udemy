use crate::apis::constants::*;
use crate::models::general::llm::{APIResponse, ChatCompletion, Message};
use dotenv::dotenv;
use reqwest::header::InvalidHeaderValue;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use std::env;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CallGptError {
    #[error("Invalid header: {0}")]
    InvalidHeader(#[from] InvalidHeaderValue),
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}

// using dynamically dispatched trait object that implements Error for flexibility
// ownership of the type implementing Send can safely be transferred between threads
pub async fn call_gpt(messages: Vec<Message>) -> Result<String, CallGptError> {
    // enables us to get information from our envvars
    dotenv().ok();

    let api_key: String =
        env::var(OPEN_AI_KEY).expect("OPEN_AI_KEY not found among environment variables");
    let api_org: String =
        env::var(OPEN_AI_ORG).expect("OPEN_AI_ORG not found among environment variables");

    // Create API key header
    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key))?,
    );
    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(api_org.as_str())?,
    );

    // Create client
    let client = Client::builder().default_headers(headers).build()?;

    let chat_completion = ChatCompletion {
        model: "gpt-4".to_string(),
        messages,
        temperature: 0.1,
    };

    let response: APIResponse = client
        .post(OPENAI_API_URL)
        .json(&chat_completion)
        .send()
        .await?
        .json() // convert to APIResponse here
        .await?;

    Ok(response.choices[0].api_message.content.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[cfg(feature = "openai-coverage")]
    async fn test_call_to_openai() {
        let message = Message {
            role: "user".to_string(),
            content: "Hi there, this is a test. Give me a brief response.".to_string(),
        };

        let response = call_gpt(vec![message]).await;

        match response {
            Ok(content) => {
                assert!(true);
            }
            Err(err) => {
                assert!(false);
            }
        }
    }
}
