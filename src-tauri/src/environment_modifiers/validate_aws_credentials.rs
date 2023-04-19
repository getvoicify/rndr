use tauri::State;
use crate::utils::aws_client_factory::{AwsClientFactory, AwsClientType};
use crate::utils::error::RNDRError;
use crate::utils::error::RNDRError::GenericError;
use crate::utils::file_logger::FileLogger;
use crate::utils::logger::Logger;

#[tauri::command]
pub async fn validate(logger: State<'_, FileLogger>) -> Result<bool, RNDRError> {
    logger.log("[RUST]: Validating AWS credentials");
    let factory = AwsClientFactory::new("rndr").await;
    let client = factory.get_client("sts");

    let _client = match client {
        None => return Err(GenericError(String::from("Could not get STS client"))),
        Some(c) => match c {
            AwsClientType::Sts(sts) => sts,
            _ => return Err(GenericError(String::from("Could not get STS client")))
        }
    };

    let ident = _client.get_caller_identity().send().await;

    return match ident {
        Ok(_) => Ok(true),
        Err(err) => return Err(GenericError(format!("Could not get caller identity: {}", err)))
    }
}