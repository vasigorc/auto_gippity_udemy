use std::time::Duration;

use crate::{
    ai_functions::aifunc_architect::{print_project_scope, print_site_urls},
    helpers::{
        command_line::PrintCommand,
        general::{ai_task_request_decoded, check_status_code},
    },
    models::agent_basic::basic_agent::{AgentState, BasicAgent},
};
use async_trait::async_trait;
use reqwest::Client;

use super::agent_traits::{FactSheet, ProjectScope, SpecialFunctions};

#[derive(Debug)]
pub struct AgentSolutionArchitect {
    attributes: BasicAgent,
}

impl AgentSolutionArchitect {
    #[allow(dead_code)]
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
        self.attributes.state = AgentState::Validation;
    }
}

#[async_trait]
impl SpecialFunctions for AgentSolutionArchitect {
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
                    let project_scope = self.call_project_scope(fact_sheet).await;
                    if project_scope.is_external_urls_required {
                        self.call_determine_external_urls(
                            fact_sheet,
                            fact_sheet.project_description.clone(),
                        )
                        .await;
                        self.attributes.state = AgentState::Validation;
                    }
                }
                AgentState::Validation => {
                    let mut exclude_urls: Vec<String> = vec![];
                    let client = Client::builder()
                        .timeout(Duration::from_secs(5))
                        .build()
                        .unwrap();

                    // Find faulty urls
                    let urls: &Vec<String> = fact_sheet.external_urls.as_ref();

                    for url in urls {
                        let endpoint_str = format!("Testing URL endpoint: {}", url);
                        PrintCommand::UnitTest.print_agent_message(
                            self.attributes.position.as_str(),
                            endpoint_str.as_str(),
                        );

                        // Perform the actual URL test
                        match check_status_code(&client, url).await {
                            Ok(status_code) => {
                                if status_code != 200 {
                                    exclude_urls.push(url.clone());
                                }
                            }
                            Err(e) => {
                                eprintln!("Error checking {}: {}", url, e);
                            }
                        }
                    }

                    if !exclude_urls.is_empty() {
                        let new_urls: Vec<String> = fact_sheet
                            .external_urls
                            .iter()
                            .filter(|url| !exclude_urls.contains(url))
                            .cloned()
                            .collect();
                        fact_sheet.external_urls = new_urls;
                    }
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
    async fn test_soltuion_architect() {
        let mut agent = AgentSolutionArchitect::new();

        let mut dummy_factsheet = FactSheet {
            project_description:
                "Build a full stack website with user login that shows latest Forex prices"
                    .to_string(),
            project_scope: None,
            external_urls: vec![],
            backend_code: None,
            api_endpoint_schema: vec![],
        };

        agent
            .execute(&mut dummy_factsheet)
            .await
            .expect("Unable to execute Solutions Arhitect Agent");

        assert!(dummy_factsheet.project_scope.is_some());
        assert!(!dummy_factsheet.external_urls.is_empty());
        dbg!(agent);
    }
}
