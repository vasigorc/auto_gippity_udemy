use crate::models::general::llm::Message;

use super::basic_agent::AgentState;

pub trait BasicTraits {
    fn new(objective: String, position: String) -> Self;
    fn update_state(&mut self, new_state: AgentState);
    fn get_objective(&self) -> &String;
    fn get_position(&self) -> &String;
    fn get_state(&self) -> &AgentState;
    fn get_memory(&self) -> &Vec<Message>;
}
