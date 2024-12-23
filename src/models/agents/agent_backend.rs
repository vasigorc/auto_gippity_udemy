use std::{
    process::{Command, Stdio},
    time::Duration,
};

use async_trait::async_trait;
use reqwest::Client;
use tokio::time;

use crate::{
    ai_functions::aifunc_backend::{
        print_backend_webserver_code, print_fixed_code, print_improved_webserver_code,
        print_rest_api_endpoints,
    },
    helpers::{
        command_line::{
            is_code_safe, read_template_contents, save_api_endpoints, save_backend_code,
            PrintCommand, CODE_TEMPLATE_PATH, EXEC_MAIN_PATH, WS_PROJECT_PATH,
        },
        general::{ai_task_request, ai_task_request_decoded, check_status_code},
    },
    models::agent_basic::basic_agent::{AgentState, BasicAgent},
};

use super::agent_traits::{FactSheet, RouteObject, SpecialFunctions};

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
            "BROKEN_CODE: {:?}\n ERROR_BUGS: {:?}\n
            THIS FUNCTION ONLY OUTPUTS CODE. JUST OUTPUT THE CODE.",
            fact_sheet.backend_code, self.bug_errors
        );

        let ai_response: String = ai_task_request(
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
        let ai_response: String = ai_task_request(
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
impl SpecialFunctions for AgentBackendDeveloper {
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
                    // Guard: ensure AI safety
                    PrintCommand::UnitTest.print_agent_message(
                        self.attributes.position.as_str(),
                        "Backend Code Unit Testing: Requesting user input",
                    );

                    if !is_code_safe() {
                        println!("Exeting because AI generated code was deemed not safe.");
                    }

                    PrintCommand::UnitTest.print_agent_message(
                        self.attributes.position.as_str(),
                        "Backend Code Unit Testing: building project...",
                    );

                    let build_backend_server = Command::new("cargo")
                        .arg("build")
                        .current_dir(WS_PROJECT_PATH)
                        // return information back to us
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .output()
                        .expect("Failed to build backend application");

                    if build_backend_server.status.success() {
                        self.bug_count = 0;

                        PrintCommand::UnitTest.print_agent_message(
                            self.attributes.position.as_str(),
                            "Backend Code Unit Testing: Test server build succcessful...",
                        );
                    } else {
                        let error_string = String::from_utf8(build_backend_server.stderr).unwrap();
                        self.bug_count += 1;
                        self.bug_errors = Some(error_string);

                        // Exit if too many bugs
                        if self.bug_count > 2 {
                            PrintCommand::Issue.print_agent_message(
                                self.attributes.position.as_str(),
                                "Backend Code Unit Testing: Too many bugs found in code...",
                            );
                            panic!("Error: too many bugs...");
                        }

                        // Pass back for rework
                        self.attributes.state = AgentState::Working;
                        continue;
                    }

                    let api_endpoints_str = self.call_extract_rest_api_endpoints().await;
                    // Extract and test API endpoints
                    let api_endpoints: Vec<RouteObject> =
                        serde_json::from_str(api_endpoints_str.as_str())
                            .expect("Failed to deserialize endpoints into RouteObjects");

                    let static_endpoints = api_endpoints
                        .iter()
                        .filter(|&route_object| {
                            route_object.method == "get" && route_object.is_route_dynamic == "false"
                        })
                        .cloned()
                        .collect::<Vec<RouteObject>>();

                    fact_sheet.api_endpoint_schema.clone_from(&static_endpoints);

                    PrintCommand::UnitTest.print_agent_message(
                        self.attributes.position.as_str(),
                        "Backend Code Unit Testing: Starting Web Server...",
                    );

                    let mut run_backend_server = Command::new("cargo")
                        .arg("run")
                        .current_dir(WS_PROJECT_PATH)
                        // return information back to us
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .spawn()
                        .expect("Failed to run backend application");

                    PrintCommand::UnitTest.print_agent_message(
                        self.attributes.position.as_str(),
                        "Backend Code Unit Testing: Launching server in 5 seconds...",
                    );

                    time::sleep(Duration::from_secs(5)).await;

                    for endpoint in static_endpoints {
                        let testing_msg = format!("Testing endpoint '{}'...", endpoint.route);

                        PrintCommand::UnitTest.print_agent_message(
                            self.attributes.position.as_str(),
                            testing_msg.as_str(),
                        );

                        let url = format!("http://localhost:8080{}", endpoint.route);

                        let client = Client::builder()
                            .timeout(Duration::from_secs(5))
                            .build()
                            .unwrap();

                        // Print out the result of testing
                        match check_status_code(&client, &url).await {
                            Ok(status_code) => {
                                if status_code != 200 {
                                    PrintCommand::Issue.print_agent_message(
                                        self.attributes.position.as_str(),
                                        format!(
                                            "WARNING: Failed to call backend url endpoint {} ",
                                            endpoint.route
                                        )
                                        .as_str(),
                                    );
                                }
                            }
                            Err(e) => {
                                PrintCommand::Issue.print_agent_message(
                                    self.attributes.position.as_str(),
                                    format!("Error checking backend {}", e).as_str(),
                                );
                            }
                        }
                    }
                    save_api_endpoints(&api_endpoints_str);

                    PrintCommand::UnitTest.print_agent_message(
                        self.attributes.position.as_str(),
                        "Backend testing complete...",
                    );

                    run_backend_server
                        .kill()
                        .expect("Failed to kill backend web server");

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
    #[cfg(feature = "openai-coverage")]
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

        agent.attributes.state = AgentState::Validation;
        agent
            .execute(&mut fact_sheet)
            .await
            .expect("Failed to execut Backend Developer agent");
    }
}
