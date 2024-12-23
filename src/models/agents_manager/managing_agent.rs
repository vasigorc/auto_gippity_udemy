use crate::{
    ai_functions::aifunc_managing::convert_user_input_to_goal,
    helpers::general::ai_task_request,
    models::{
        agent_basic::basic_agent::{AgentState, BasicAgent},
        agents::{
            agent_architect::AgentSolutionArchitect,
            agent_traits::{FactSheet, SpecialFunctions},
        },
    },
};

#[derive(Debug)]
pub struct ManagingAgent {
    attributes: BasicAgent,
    fact_sheet: FactSheet,
    agents: Vec<Box<dyn SpecialFunctions>>,
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

        let agents: Vec<Box<dyn SpecialFunctions>> = vec![];

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

    fn add_agent(&mut self, agent: Box<dyn SpecialFunctions>) {
        self.agents.push(agent);
    }

    fn create_agents(&mut self) {
        self.add_agent(Box::new(AgentSolutionArchitect::new()));
    }

    pub async fn execute_project(&mut self) {
        self.create_agents();

        for agent in &mut self.agents {
            let agent_response = agent.execute(&mut self.fact_sheet).await;

            let agent_info = agent.get_attributes_from_agent();
            dbg!(agent_info);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[cfg(feature = "openai-coverage")]
    async fn test_managing_agent() {
        let user_request: &str = r#"I need a full-stack app that fetches and tracks my fitness 
          progress. It needs to include timezone info from the web."#;

        let mut managing_agent = ManagingAgent::new(user_request.to_string())
            .await
            .expect("Error creating Managing Agent");

        managing_agent.execute_project().await;

        dbg!(managing_agent.fact_sheet);
    }
}
