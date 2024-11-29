use crate::models::general::llm::Message;
use dotenv::dotenv;
use reqwest::Client;
use std::env;

pub async fn call_gpt(message: Vec<Message>) {
    // enables us to get information from our envvars
    dotenv().ok();

    let api_key: String =
        env::var("OPEN_API_KEY").expect("OPEN_API_KEY not found among environment variables");
    let api_org: String =
        env::var("OPEN_API_ORG").expect("OPEN_API_ORG not found among environment variables");

    let url: &str = "https://api.openapi.com/v1/chat/completions";
}
