#[macro_export]
macro_rules! get_function_string {
    ($func: ident) => {{
        stringify!($func)
    }};
}

#[macro_use]
mod ai_functions;
mod apis;
mod helpers;
mod models;

use helpers::command_line::get_user_response;
use models::agents_manager::managing_agent::ManagingAgent;

#[tokio::main]
async fn main() {
    let user_request = get_user_response("What website are we building today?");
    let mut managing_agent = ManagingAgent::new(user_request)
        .await
        .expect("Error creating agent");

    managing_agent.execute_project().await;
}
