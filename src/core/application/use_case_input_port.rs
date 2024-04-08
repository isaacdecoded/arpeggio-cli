use async_trait::async_trait;
use anyhow::Result;

#[async_trait]
pub trait UseCaseInputPort<RequestModel>: Send {
    async fn interact(&self, request_model: RequestModel) -> Result<()>;
}
