use crate::models::{
    agent_basic::basic_agent::BasicAgent,
    agents::agent_traits::{FactSheet, SpecifalFunctions},
};

#[derive(Debug)]
pub struct ManagingAgent {
    attributes: BasicAgent,
    fact_sheet: FactSheet,
    agents: Vec<Box<dyn SpecifalFunctions>>,
}
