use async_trait::async_trait;
use anyhow::{ Result, Error };

#[async_trait]
pub trait UseCaseOutputPort<ResponseModel>: Send + Sync {
    async fn success(&self, response_model: ResponseModel) -> Result<()>;
    async fn failure(&self, error: &Error) -> Result<()>;
}
