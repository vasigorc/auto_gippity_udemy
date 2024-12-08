use serde::{Deserialize, Serialize};

use crate::models::agent_basic::basic_agent::BasicAgent;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RouteObject {
    pub is_route_dynamic: String,
    pub method: String,
    pub request_body: serde_json::Value,
    pub response: serde_json::Value,
    pub route: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct ProjectScope {
    pub is_crud_required: bool,
    pub is_user_login_and_logout: bool,
    pub is_external_urls_required: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FactSheet {
    pub project_description: String,
    pub project_scope: Option<ProjectScope>,
    pub external_urls: Vec<String>,
    pub backend_code: Option<String>,
    pub api_endpoint_schema: Vec<RouteObject>,
}

// uncomment the line below if async_trait is not natively supported by Rust yet
// #[async_trait]
pub trait SpecifalFunctions: Debug {
    // Used by the manager to get agents' attributes
    fn get_attributes_from_agent(&self) -> &BasicAgent;

    // managing agent can call agents to execute whatever task
    async fn execute(
        &mut self,
        fact_sheet: &mut FactSheet,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
