use thiserror::Error;
use async_trait::async_trait;

#[derive(Error, Debug)]
pub enum FindBoundedContextsRepositoryError {
    #[error("An error occurred while trying to list bounded contexts: {0}")] ListError(String),
}

#[async_trait]
pub trait FindBoundedContextsRepository<ReadModel>: Send + Sync {
    async fn list_bounded_contexts(
        &self
    ) -> Result<Vec<ReadModel>, FindBoundedContextsRepositoryError>;
}
