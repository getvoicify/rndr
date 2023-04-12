use aws_sdk_cloudformation::{Client, Error};
use tauri::State;

async fn _list_stacks(client: &Client) -> Result<Vec<String>, Error> {
    let stacks = client.list_stacks().send().await?;
    let mut stack_names: Vec<String> = Vec::new();

    for stack in stacks.stack_summaries().unwrap_or_default() {
        if let Some(stack_status) = stack.stack_status() {
            let is_complete = stack_status == &aws_sdk_cloudformation::model::StackStatus::CreateComplete
                || stack_status == &aws_sdk_cloudformation::model::StackStatus::UpdateComplete
                || stack_status == &aws_sdk_cloudformation::model::StackStatus::RollbackComplete;

            let is_rndr_stack = stack.stack_name().unwrap_or_default().starts_with("rndr-stack");

            if is_rndr_stack && is_complete {
                stack_names.push(stack.stack_name().unwrap_or_default().to_string());
            }
        }
    }

    Ok(stack_names)
}

#[tauri::command]
pub async fn get_stack_list(client: State<'_, Client>) -> Result<Vec<String>, String> {
    let _client = client.inner();
    let stacks = _list_stacks(_client).await;

    match stacks {
        Ok(stacks) => {
            Ok(stacks)
        },
        Err(err) => {
            println!("Error: {}", err);
            Err("Error".to_string())
        }
    }
}