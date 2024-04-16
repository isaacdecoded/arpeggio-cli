use async_trait::async_trait;

#[async_trait]
pub trait UseCaseInputPort<RequestModel>: Send {
    async fn interact(&self, request_model: RequestModel);
}
