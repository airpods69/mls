use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::planner::planner;

#[derive(Deserialize, PartialEq)]
pub struct PlannerWebhookInput {
    issue: String,
}

#[derive(Serialize)]
pub struct PlannerWebhookOutput {
    plan_result: String,
}

fn escape_json_string(input: &str) -> String {
    let mut result = String::new();
    for c in input.chars() {
        match c {
            '"' => result.push_str("\\\""),
            '\\' => result.push_str("\\\\"),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            '\x08' => result.push_str("\\b"),
            '\x0c' => result.push_str("\\f"),
            c if c < ' ' || c > '\x7f' => {
                result.push_str(&format!("\\u{:04x}", c as u32));
            }
            _ => result.push(c),
        }
    }
    result
}

// The planner route is responsible for deserializing the webhook
// request from github and then sending the issue raised on github and making a plan out of it.
// Once the plan is ready, the route will create a PR with the plan in it.
#[post("/planner", format = "json", data = "<issue_input>")]
pub async fn route_planner(issue_input: Json<PlannerWebhookInput>) -> Json<PlannerWebhookOutput> {
    let mut plan: String = planner::start_plan(issue_input.issue.clone()).await.expect("REASON");

    plan = escape_json_string(&plan);

    Json(PlannerWebhookOutput {
        plan_result: plan.clone(),
    })
}
