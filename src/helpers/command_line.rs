use std::{
    fs,
    io::{stdin, stdout, Stdout},
};

use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};

pub const CODE_TEMPLATE_PATH: &str =
    "/home/vasilegorcinschi/repos/web_template_autogpt/src/code_template.rs";
pub const EXEC_MAIN_PATH: &str = "/home/vasilegorcinschi/repos/web_template_autogpt/src/main.rs";
const API_SCHEMA_PATH: &str =
    "/home/vasilegorcinschi/repos/auto_gippity_udemy/schemas/api_schema.json";

#[derive(PartialEq, Debug)]
pub enum PrintCommand {
    AICall,
    UnitTest,
    Issue,
}

impl PrintCommand {
    pub fn print_agent_message(&self, agent_pos: &str, agent_statement: &str) {
        let mut stdout = stdout();

        let statement_color = match self {
            Self::AICall => Color::Cyan,
            Self::UnitTest => Color::Magenta,
            Self::Issue => Color::Red,
        };

        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        print!("Agent {}:", agent_pos);
        stdout.execute(SetForegroundColor(statement_color)).unwrap();
        println!("{}", agent_statement);
        stdout.execute(ResetColor).unwrap();
    }
}

pub fn get_user_response(question: &str) -> String {
    let mut stdout: Stdout = stdout();

    // Print the question in a specific color
    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
    println!("\n{}", question);

    // Reset the color
    stdout.execute(ResetColor).unwrap();

    // Read user input
    let mut user_response = String::new();
    stdin()
        .read_line(&mut user_response)
        .expect("Failed to read response");
    user_response.trim().to_string()
}

// Get code template and provide it as a single string to ChatGPT
pub fn read_template_contents(template_path: &str) -> String {
    fs::read_to_string(template_path).expect("Error reading code template")
}

// Save new backend code
pub fn save_backend_code(contents: &String) {
    fs::write(EXEC_MAIN_PATH, contents).expect("Error writing backend code")
}
// Save JSON API Endpoint Schema
pub fn save_api_endpoints(api_endpoints: &String) {
    fs::write(API_SCHEMA_PATH, api_endpoints).expect("Failed to write API endpoints to file")
}

// Our flow involves allowing AI to execute code on our machine
// This can potentially harmful for any host running this
// As a safety measure we will want to review any code before allowing
// to execute it
pub fn is_code_safe() -> bool {
    let mut stdout = stdout();

    loop {
        stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
        println!("\nWARNING: You are about to run code written entirely by AI");
        println!("Review the code and confirm that you wish to continue");

        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        println!("[1] All good!");

        stdout.execute(SetForegroundColor(Color::DarkRed)).unwrap();
        println!("[2] Let's stop this project!");

        stdout.execute(ResetColor).unwrap();

        let mut human_response: String = String::new();
        stdin()
            .read_line(&mut human_response)
            .expect("Failed to read human response!");

        human_response = human_response.trim().to_lowercase();
        match human_response.as_str() {
            "1" | "ok" | "y" => return true,
            "2" | "no" | "n" => return false,
            _ => {
                println!("Invalid input, please select '1' or '2'")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_printing_agent_message() {
        PrintCommand::AICall.print_agent_message("agent_pos", "agent_statement");
    }
}
