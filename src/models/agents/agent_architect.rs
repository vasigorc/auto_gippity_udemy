use crate::{
    ai_functions::aifunc_architect::{print_project_scope, print_site_urls},
    helpers::general::ai_task_request_decoded,
    models::agent_basic::basic_agent::{AgentState, BasicAgent},
};

use super::agent_traits::{FactSheet, ProjectScope};

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
            objective: "Gathers information and design solutions for website development"
                .to_string(),
            position: "Solutions architect".to_string(),
            state: AgentState::Discovering,
            memory: vec![],
        };

        Self { attributes }
    }

    // Retrieve project scope
    async fn call_project_scope(&mut self, fact_sheet: &mut FactSheet) -> ProjectScope {
        let msg_context = fact_sheet.project_description.to_string();

        let ai_response = ai_task_request_decoded::<ProjectScope>(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_project_scope),
            print_project_scope,
        )
        .await;

        fact_sheet.project_scope = Some(ai_response);
        self.attributes.state = AgentState::Finished;
        ai_response
    }

    async fn call_determine_external_urls(
        &mut self,
        fact_sheet: &mut FactSheet,
        msg_context: String,
    ) {
        let ai_response: Vec<String> = ai_task_request_decoded::<Vec<String>>(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_site_urls),
            print_site_urls,
        )
        .await;

        fact_sheet.external_urls = ai_response;
        self.attributes.state = AgentState::UnitTesting;
    }
}
