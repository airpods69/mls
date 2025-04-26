use crate::text_generation::groq;


// Updated start_plan to extract the assistant's response content
pub async fn start_plan(issue_input: String) -> Result<String, reqwest::Error> {
    let planner_prompt: String = "
You are a senior programming manager.
Your job is to take the issue raised by the users and plan out on how to tackle the situation.
It could either be an issue or a feature request.
Your job is to tell your programmer the steps they have to follow so that they can resolve the situation.
Then a tester will come in and test out the programmer's code and make sure everything is okay.

Issue or Feature request raised on Github:
".to_string();

    let prompt: String = planner_prompt.clone() + &issue_input.to_string();

    let groq_response = groq::generate_response(&prompt).await?;

    let response_content = groq_response
        .choices
        .get(0)
        .map(|choice| choice.message.content.clone());

    Ok(response_content.unwrap())
}
