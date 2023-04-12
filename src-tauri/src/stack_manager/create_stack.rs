use std::path::PathBuf;
use std::thread::sleep;
use aws_sdk_cloudformation::{Client as CFClient};
use aws_sdk_cloudformation::model::Capability::CapabilityIam;
use aws_sdk_cloudformation::model::StackStatus::CreateComplete;
use aws_sdk_cloudformation::output::CreateStackOutput;
use tauri::{AppHandle, State, Wry};
use crate::stack_manager::describe_stack::get_stack;
use crate::utils::file_logger::FileLogger;
use crate::utils::logger::Logger;
use crate::utils::read_file_to_text_string::read_file_to_string;

#[tauri::command]
pub async fn create_aws_stack(_client: State<'_, CFClient>, _logger: State<'_, FileLogger>, handle: AppHandle<Wry>) -> Result<String, String> {
    let logger = _logger.inner();
    let client = _client.inner();
    let _stack_name = "rndr-stack";
    let path_to_stack_file = handle.path_resolver().app_data_dir();
    if let Some(path_buf) = path_to_stack_file {
        let path = path_buf.join(".config").join(".dep_repo").join("aws").join("cloud-render-cloudformation.yml");

        if !path.exists() {
            logger.log("[RUST]: Could not find path to stack file");
            return Err("Could not find path to stack file".to_string());
        }

        logger.log(&format!("[RUST]: Path to stack file: {}", path.to_str().unwrap()));

        match _create_stack(client, logger, _stack_name, path).await {
            Ok(_) => {
                logger.log("[RUST]: Stack created successfully");
                if is_stack_complete(client, _stack_name).await? {
                    logger.log("[RUST]: Stack is complete");
                    let stack = get_stack(client, _stack_name).await;
                    match stack {
                        Ok(stack) => {
                            logger.log(&format!("[RUST]: Stack: {:?}", stack));
                        },
                        Err(err) => {
                            logger.log(&format!("[RUST]: Error getting stack: {}", err));
                        }
                    }
                } else {
                    logger.log("[RUST]: Stack is not complete");
                }

                Ok("Stack created successfully".to_string())
            },
            Err(err) => {
                logger.log(&format!("[RUST]: Error creating stack: {}", err));
                Err(err)
            }
        }
    } else {
        logger.log("[RUST]: Could not find path to stack file");
        Err("Could not find path to stack file".to_string())
    }
}

async fn _create_stack(client: &CFClient, logger: &FileLogger, stack_name: &str, path: PathBuf) -> Result<CreateStackOutput, String> {
    logger.log(&*format!("[RUST]: Creating stack - {}", stack_name));

    let path_string = path.to_str().unwrap_or_default();
    let stack_template = match read_file_to_string(path_string) {
        Ok(template) => template,
        Err(err) => {
            logger.log(&format!("[RUST]: Error reading stack file: {}", err));
            return Err("Error reading stack file".to_string());
        }
    };

    let output = client.create_stack()
        .stack_name(stack_name)
        .template_body(stack_template)
        .capabilities(CapabilityIam)
        .send()
        .await;

    return match output {
        Ok(o) => {
            logger.log(&format!("[RUST]: Stack created: {}", o.stack_id.as_ref().unwrap()));
            Ok(o)
        }
        Err(err) => {
            Err(err.to_string())
        }
    }
}

async fn is_stack_complete(client: &CFClient, stack_name: &str) -> Result<bool, String> {

    loop {
        let stack = get_stack(client, stack_name).await?;
        let stack_status = stack.stacks()
            .unwrap_or_default()
            .first()
            .unwrap()
            .stack_status();
        sleep(std::time::Duration::from_secs(30));
        if stack_status.unwrap().as_str() == CreateComplete.as_str() {
            break;
        }
    }

    Ok(true)
}
