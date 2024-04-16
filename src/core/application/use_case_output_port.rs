use std::error::Error;
use async_trait::async_trait;

#[async_trait]
pub trait UseCaseOutputPort<ResponseModel>: Send + Sync {
    async fn success(&self, response_model: ResponseModel);
    async fn failure(&self, error: Box<dyn Error + Send>);
}
