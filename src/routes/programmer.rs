use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::programmer::programmer;

#[derive(Deserialize, PartialEq)]
pub struct ProgrammerWebhookInput {
    issue: String,
}

#[derive(Serialize)]
pub struct ProgrammerWebhookOutput {
    plan_result: String,
}

// The planner route is responsible for deserializing the webhook
// request from github and then sending the issue raised on github and making a plan out of it.
// Once the plan is ready, the route will create a PR with the plan in it.
#[post("/programmer", format = "json", data = "<issue_input>")]
pub async fn route_programmer(issue_input: Json<ProgrammerWebhookInput>) -> Json<ProgrammerWebhookOutput> {
    let plan: String = programmer::start_programming(issue_input.issue.clone()).await.expect("REASON");

    Json(ProgrammerWebhookOutput {
        plan_result: plan.clone(),
    })
}
