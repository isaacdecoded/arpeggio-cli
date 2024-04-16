use anyhow::Result;
use async_trait::async_trait;
// use crate::core::domain::models::identity_object::IdentityObject;

#[async_trait]
pub trait ProjectService: Send + Sync {
    async fn create_project(&self, project_name: &str) -> Result<()>;
    // async fn initialize_bounded_context(&self, bounded_context_id: &IdentityObject) -> Result<()>;
}
