use reqwest::Client;
use serde::de::DeserializeOwned;

use crate::{apis::call_requests::call_gpt, models::general::llm::Message};

use super::command_line::PrintCommand;

// encourage certain specific output
pub fn extend_ai_function(ai_func: fn(&str) -> &'static str, func_input: &str) -> Message {
    let ai_func_str = ai_func(func_input);
    dbg!(ai_func_str);

    // Extend the string to encourage only printing the output
    let msg: String = format!(
        "FUNCTION {}
    INSTRUCTION: You are a function printer. You only PRINT the results of functions.
    Nothing else. No commentary. Here is the input to the function: {}. Print out
    what the function will return.",
        ai_func_str, func_input
    );

    Message {
        role: "system".to_string(),
        content: msg,
    }
}

// Perform call to LLM GPT

pub async fn ai_task_request(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_passed: for<'a> fn(&'a str) -> &'static str,
) -> String {
    // Extend AI function
    let extended_message = extend_ai_function(function_passed, &msg_context);

    // Print current status
    PrintCommand::AICall.print_agent_message(agent_position, agent_operation);

    // Attempt first call
    match call_gpt(vec![extended_message.clone()]).await {
        Ok(response) => response,
        Err(_) => {
            // Retry if the first call fails
            call_gpt(vec![extended_message])
                .await
                .expect("Failed to call OpenAI twice")
        }
    }
}

pub async fn ai_task_request_decoded<T: DeserializeOwned>(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_passed: for<'a> fn(&'a str) -> &'static str,
) -> T {
    let llm_response = ai_task_request(
        msg_context,
        agent_position,
        agent_operation,
        function_passed,
    )
    .await;

    let decoded_response: T = serde_json::from_str(llm_response.as_str())
        .expect("Failed to decode AI response from serde_json");
    decoded_response
}

// Check whether request URL is valid
pub async fn check_status_code(client: &Client, url: &str) -> Result<u16, reqwest::Error> {
    let response = client.get(url).send().await?;
    Ok(response.status().as_u16())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;

    #[test]
    fn test_extending_ai_function() {
        let extended_msg = extend_ai_function(convert_user_input_to_goal, "dummy variable");
        assert_eq!(extended_msg.role, "system".to_string());
    }

    #[tokio::test]
    #[cfg(feature = "openai-coverage")]
    async fn test_ai_task_request() {
        let ai_func_param = "Build me a webserver for making stock price api requests".to_string();

        let result = ai_task_request(
            ai_func_param,
            "Managing agent",
            "Defining user requirements",
            convert_user_input_to_goal,
        )
        .await;

        assert!(result.len() > 20);
    }
}
