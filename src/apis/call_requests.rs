use crate::models::general::llm::Message;
use dotenv::dotenv;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use std::env;

pub async fn call_gpt(message: Vec<Message>) {
    // enables us to get information from our envvars
    dotenv().ok();

    let api_key: String =
        env::var("OPEN_API_KEY").expect("OPEN_API_KEY not found among environment variables");
    let api_org: String =
        env::var("OPEN_API_ORG").expect("OPEN_API_ORG not found among environment variables");

    let url: &str = "https://api.openapi.com/v1/chat/completions";

    // Create API key header
    let mut headers = HeaderMap::new();
    headers.insert(
        "authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
    );
    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(api_org.as_str()).unwrap(),
    );

    // Create client
    let client = Client::builder().default_headers(headers).build().unwrap();
}
