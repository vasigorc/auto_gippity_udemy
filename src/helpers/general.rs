use std::fmt::format;

use crate::models::general::llm::Message;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;

    #[test]
    fn test_extending_ai_function() {
        let extended_msg = extend_ai_function(convert_user_input_to_goal, "dummy variable");
        assert_eq!(extended_msg.role, "system".to_string());
    }
}
