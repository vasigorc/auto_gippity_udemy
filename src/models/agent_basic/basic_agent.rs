use crate::models::general::llm::Message;

use super::basic_traits::BasicTraits;

#[derive(Debug, PartialEq)]
pub enum AgentState {
    Discovering,
    Working,
    Validation,
    Finished,
}

#[derive(Debug)]
pub struct BasicAgent {
    pub objective: String,
    pub position: String,
    pub state: AgentState,
    pub memory: Vec<Message>,
}

impl BasicTraits for BasicAgent {
    fn new(objective: String, position: String) -> Self {
        Self {
            objective,
            position,
            state: AgentState::Discovering,
            memory: vec![],
        }
    }

    fn update_state(&mut self, new_state: AgentState) {
        self.state = new_state;
    }

    fn get_objective(&self) -> &String {
        &self.objective
    }

    fn get_position(&self) -> &String {
        &self.position
    }

    fn get_state(&self) -> &AgentState {
        &self.state
    }

    fn get_memory(&self) -> &Vec<Message> {
        &self.memory
    }
}
