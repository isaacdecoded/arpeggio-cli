use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait ProjectService: Send + Sync {
    async fn create_project(&self, project_name: &str) -> Result<()>;
}
