// This is the programmer who will read through the code and make changes.
// For step 1, he only writes the code and returns it back with no explanation.


use crate::text_generation::groq;


// Updated start_plan to extract the assistant's response content
pub async fn start_programming(plan_input: String) -> Result<String, reqwest::Error> {
    let programmer_prompt: String = "
You are an expert programmer in Rust and Python.
Your manager gave you a plan which you need to follow for a program that he needs. Your manager doesn't like to talk so give only the code and results.
You can think using comments.

Given the plan below, write the code in python for the plan

Plan:

".to_string();

    let prompt: String = programmer_prompt.clone() + &plan_input.to_string();

    let groq_response = groq::generate_response(&prompt).await?;

    let response_content = groq_response
        .choices
        .get(0)
        .map(|choice| choice.message.content.clone());

    Ok(response_content.unwrap())
}
