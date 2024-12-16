use crate::{
    ai_functions::aifunc_managing::convert_user_input_to_goal,
    helpers::general::ai_task_request,
    models::{
        agent_basic::basic_agent::{AgentState, BasicAgent},
        agents::agent_traits::{FactSheet, SpecifalFunctions},
    },
};

#[derive(Debug)]
pub struct ManagingAgent {
    attributes: BasicAgent,
    fact_sheet: FactSheet,
    agents: Vec<Box<dyn SpecifalFunctions>>,
}

impl ManagingAgent {
    pub async fn new(user_request: String) -> Result<Self, Box<dyn std::error::Error>> {
        let attributes = BasicAgent {
            objective: "Manage agents who are building excellent websites for the user".to_string(),
            position: "Project Manager".to_string(),
            state: AgentState::Discovering,
            memory: vec![],
        };

        let project_description = ai_task_request(
            user_request,
            &attributes.position,
            get_function_string!(convert_user_input_to_goal),
            convert_user_input_to_goal,
        )
        .await;

        let agents: Vec<Box<dyn SpecifalFunctions>> = vec![];

        let fact_sheet = FactSheet {
            project_description,
            project_scope: None,
            external_urls: vec![],
            backend_code: None,
            api_endpoint_schema: vec![],
        };

        Ok(Self {
            attributes,
            fact_sheet,
            agents,
        })
    }
}
