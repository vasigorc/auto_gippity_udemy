use crate::models::agent_basic::basic_agent::BasicAgent;

#[derive(Debug)]
pub struct AgentSolutionArchitect {
    attributes: BasicAgent,
}

impl AgentSolutionArchitect {
  pub fn new() -> Self {
    let attributes = BasicAgent {
      // go to previous location g; go to next location g,
      // Ctr + o takes you to the previous locaiton in the jump list 
      // Ctr + i  takes you to the next location in the jump list
      objective: "Gathers information and design solutions for website development".to_string(),
      position: "Solutions architect".to_string(),
    }
  }
}