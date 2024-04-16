use thiserror::Error;
use async_trait::async_trait;
use crate::{
    cli::bounded_context::domain::entities::bounded_context::BoundedContext,
    core::domain::models::identity_object::IdentityObject,
};

#[derive(Error, Debug)]
pub enum BoundedContextRepositoryError {
    #[error("Bounded Context not found: {0}")] NotFound(String),
    #[error("Error writing Bounded Context: {0}")] WriteError(String),
    #[error("Error reading Bounded Context: {0}")] ReadError(String),
}

#[async_trait]
pub trait BoundedContextRepository: Send + Sync {
    async fn write_bounded_context(
        &self,
        bounded_context: &BoundedContext
    ) -> Result<(), BoundedContextRepositoryError>;
    async fn read_bounded_context(
        &self,
        bounded_context_id: &IdentityObject
    ) -> Result<Option<BoundedContext>, BoundedContextRepositoryError>;
}
