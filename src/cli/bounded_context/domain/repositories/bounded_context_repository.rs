use anyhow::Result;
use async_trait::async_trait;
use crate::{
    cli::bounded_context::domain::entities::bounded_context::BoundedContext,
    core::domain::models::identity_object::IdentityObject,
};

#[async_trait]
pub trait BoundedContextRepository: Send + Sync {
    async fn write_bounded_context(&self, bounded_context: &BoundedContext) -> Result<()>;
    async fn read_bounded_context(
        &self,
        bounded_context_id: &IdentityObject
    ) -> Result<Option<BoundedContext>>;
}
