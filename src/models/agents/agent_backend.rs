use async_trait::async_trait;

use crate::{
    ai_functions::aifunc_backend::{
        print_backend_webserver_code, print_fixed_code, print_improved_webserver_code,
        print_rest_api_endpoints,
    },
    helpers::{
        command_line::{
            read_template_contents, save_backend_code, CODE_TEMPLATE_PATH, EXEC_MAIN_PATH,
        },
        general::{ai_task_request, ai_task_request_decoded},
    },
    models::agent_basic::basic_agent::{AgentState, BasicAgent},
};

use super::agent_traits::{FactSheet, SpecifalFunctions};

#[derive(Debug)]
pub struct AgentBackendDeveloper {
    attributes: BasicAgent,
    bug_errors: Option<String>,
    bug_count: i8,
}

impl AgentBackendDeveloper {
    pub fn new() -> Self {
        let attributes = BasicAgent {
            objective: "Develops backend code for webserver and the server database".to_string(),
            position: "Backend Developer".to_string(),
            state: AgentState::Discovering,
            memory: vec![],
        };

        Self {
            attributes,
            bug_errors: None,
            bug_count: 0,
        }
    }

    async fn call_initial_backend_code(&mut self, fact_sheet: &mut FactSheet) {
        // Read the code template contents
        let code_template_string = read_template_contents(CODE_TEMPLATE_PATH);

        // Concatenate instruction
        let msg_context = format!(
            "CODE TEMPLATE: {}\n PROJECT DESCRIPTION: {}\n",
            code_template_string, fact_sheet.project_description
        );

        let ai_response: String = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_backend_webserver_code),
            print_backend_webserver_code,
        )
        .await;

        // save code on disk in the other locally stored directory
        save_backend_code(&ai_response);
        // and also save this in memory
        fact_sheet.backend_code = Some(ai_response);
    }

    async fn call_improved_backend_code(&mut self, fact_sheet: &mut FactSheet) {
        let msg_context = format!(
            "CODE TEMPLATE: {:?}\n PROJECT_DESCRIPTION: {:?}\n",
            fact_sheet.backend_code, fact_sheet
        );

        let ai_response: String = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_improved_webserver_code),
            print_improved_webserver_code,
        )
        .await;

        // save code on disk in the other locally stored directory
        save_backend_code(&ai_response);
        // and also save this in memory
        fact_sheet.backend_code = Some(ai_response);
    }

    async fn call_fix_code_bugs(&mut self, fact_sheet: &mut FactSheet) {
        let msg_context = format!(
            "BROKEN CODE: {:?}\n ERROR_BUGS: {:?}\n
            THIS FUNCTION ONLY OUTPUTS CODE. JUST OUTPUT THE CODE.",
            fact_sheet.backend_code, self.bug_errors
        );

        let ai_response: String = ai_task_request_decoded(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_fixed_code),
            print_fixed_code,
        )
        .await;

        // save code on disk in the other locally stored directory
        save_backend_code(&ai_response);
        // and also save this in memoryprint_fixed_code
        fact_sheet.backend_code = Some(ai_response);
    }

    async fn call_extract_rest_api_endpoints(&self) -> String {
        // get stringified code from our main.rs template file
        // faster and cheaper to get it from the localfile than going
        // through asking LLM for code
        let backend_code = read_template_contents(EXEC_MAIN_PATH);
        let msg_context = format!("CODE_INPUT: {}", backend_code);
        let ai_response: String = ai_task_request_decoded(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_rest_api_endpoints),
            print_rest_api_endpoints,
        )
        .await;
        ai_response
    }
}

#[async_trait]
impl SpecifalFunctions for AgentBackendDeveloper {
    fn get_attributes_from_agent(&self) -> &BasicAgent {
        &self.attributes
    }

    async fn execute(
        &mut self,
        fact_sheet: &mut FactSheet,
    ) -> Result<(), Box<dyn std::error::Error>> {
        while self.attributes.state != AgentState::Finished {
            match self.attributes.state {
                AgentState::Discovering => {
                    self.call_initial_backend_code(fact_sheet).await;
                    self.attributes.state = AgentState::Working;
                    continue;
                }
                AgentState::Working => {
                    if self.bug_count == 0 {
                        self.call_improved_backend_code(fact_sheet).await;
                    } else {
                        self.call_fix_code_bugs(fact_sheet).await;
                    }
                    self.attributes.state = AgentState::Validation;
                    continue;
                }
                AgentState::Validation => {
                    // Temporary solution before properly implementing the Validation branch
                    self.attributes.state = AgentState::Finished;
                }
                _ => self.attributes.state = AgentState::Finished,
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[cfg(feature = "openai-coverage")]
    async fn test_writing_backend_code() {
        let mut agent = AgentBackendDeveloper::new();
        let factsheet_string: &str = r#"
      {
        "project_description": "build a website that fetches and tracks fitness progress with timezone information",
        "project_scope": {
          "is_crud_required": true,
          "is_user_login_and_logout": true,
          "is_external_urls_required": true
        },
        "external_urls": [
          "http://worldtimeapi.org/api/timezone"
        ],
        "backend_code": null,
        "api_endpoint_schema": []
      }"#;

        let mut fact_sheet: FactSheet = serde_json::from_str(factsheet_string).unwrap();

        agent.attributes.state = AgentState::Discovering;
        agent
            .execute(&mut fact_sheet)
            .await
            .expect("Failed to execut Backend Developer agent");
    }
}
