use aws_sdk_cloudformation::{Client as CFClient, output::DescribeStacksOutput};

pub async fn get_stack(client: &CFClient, stack_name: &str) -> Result<DescribeStacksOutput, String> {
    let output = client.describe_stacks()
        .stack_name(stack_name)
        .send()
        .await;

    return match output {
        Ok(output) => Ok(output),
        Err(err) => Err(err.to_string())
    };
}