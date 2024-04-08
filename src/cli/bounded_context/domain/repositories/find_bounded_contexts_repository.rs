use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait FindBoundedContextsRepository<ReadModel>: Send + Sync {
    async fn list_bounded_contexts(&self) -> Result<Vec<ReadModel>>;
}
