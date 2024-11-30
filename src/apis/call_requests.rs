use crate::apis::constants::*;
use crate::models::general::llm::{ChatCompletion, Message};
use dotenv::dotenv;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use std::env;

pub async fn call_gpt(messages: Vec<Message>) {
    // enables us to get information from our envvars
    dotenv().ok();

    let api_key: String =
        env::var(ENV_OPENAI_API_KEY).expect("OPEN_AI_KEY not found among environment variables");
    let api_org: String =
        env::var(ENV_OPENAI_API_ORG).expect("OPEN_AI_ORG not found among environment variables");

    // Create API key header
    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
    );
    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(api_org.as_str()).unwrap(),
    );

    // Create client
    let client = Client::builder().default_headers(headers).build().unwrap();

    let chat_completion = ChatCompletion {
        model: "gpt-4".to_string(),
        messages,
        temperature: 0.1,
    };

    let response_raw = client
        .post(OPENAI_API_URL)
        .json(&chat_completion)
        .send()
        .await
        .unwrap();

    dbg!(response_raw.text().await.unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // ran once and ignoring after to avoid incurring costs
    async fn test_call_to_openai() {
        let message = Message {
            role: "user".to_string(),
            content: "Hi there, this is a test. Give me a brief response.".to_string(),
        };

        call_gpt(vec![message]).await;
    }
}
