use std::collections::HashMap;
use aws_config::profile::ProfileFileCredentialsProvider;
use aws_config::provider_config::ProviderConfig;
use aws_sdk_cloudformation as cloudformation;
use aws_sdk_s3 as s3;
use aws_sdk_cloudformation::{Client as CFClient};
use aws_sdk_sts::Client as StsClient;

pub enum AwsClientType {
    S3(s3::Client),
    CloudFormation(CFClient),
    Sts(StsClient),
}

pub struct AwsClientFactory {
    client_map: HashMap<String, AwsClientType>,
}

impl AwsClientFactory {
    pub(crate) async fn new(profile: &str) -> Self {
        let mut client_map = HashMap::new();

        let provider = ProfileFileCredentialsProvider::builder()
            .profile_name(profile)
            .configure(&ProviderConfig::with_default_region().await)
            .build();


        let config = aws_config::from_env().credentials_provider(provider).load().await;
        let client = cloudformation::Client::new(&config);
        let s3_client = s3::Client::new(&config);
        let sts_client = StsClient::new(&config);

        client_map.insert("s3".to_string(), AwsClientType::S3(s3_client));
        client_map.insert("cloudformation".to_string(), AwsClientType::CloudFormation(client));
        client_map.insert("sts".to_string(), AwsClientType::Sts(sts_client));
        Self { client_map }
    }

    pub(crate) fn get_client(&self, key: &str) -> Option<&AwsClientType> {
        self.client_map.get(key)
    }
}