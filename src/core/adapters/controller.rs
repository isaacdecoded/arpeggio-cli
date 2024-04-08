use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait Controller<RequestObject> {
    async fn execute(&self, request_object: RequestObject) -> Result<(), Box<dyn Error>>;
}
