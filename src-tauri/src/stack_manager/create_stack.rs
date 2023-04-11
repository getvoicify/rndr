use aws_sdk_cloudformation as cloudformation;

pub struct CreateStack {
    pub client: cloudformation::Client,
    pub stack_name: String,
    pub template_body: String
}

impl CreateStack {
    pub async fn new(&mut self) -> Result<(), cloudformation::Error> {
        self.client
            .create_stack()
            .stack_name(&self.stack_name)
            .template_body(&self.template_body)
            .send()
            .await?;
        Ok(())
    }

    pub fn build(self) -> CreateStack {
        CreateStack {
            client: self.client,
            stack_name: self.stack_name,
            template_body: self.template_body
        }
    }
}
