use std::path::Path;
use anyhow::{ Context, Result };
use std::process::Command;
use async_trait::async_trait;
use crate::cli::bounded_context::domain::services::project_service::ProjectService;

pub struct FilesystemProjectService;

impl FilesystemProjectService {
    fn check_directory(&self, directory_path: &str) -> Result<()> {
        let path = Path::new(&directory_path);
        if path.exists() {
            return Err(anyhow::anyhow!("Project <{}> already exists", directory_path));
        }
        Ok(())
    }
}

#[async_trait]
impl ProjectService for FilesystemProjectService {
    async fn create_project(&self, project_name: &str) -> Result<()> {
        self.check_directory(project_name)?;
        Command::new("git")
            .arg("clone")
            // .arg("https://github.com/isaacdecoded/arpeggio-rs")
            .arg("https://github.com/interest-protocol/sui-dca-bot.git")
            .arg(project_name)
            .output()
            .context("Failed to create the project")?;
        Command::new("rm")
            .arg("-rf")
            .arg(format!("{}/.git", project_name))
            .output()
            .context("Failed to create the project")?;
        Ok(())
    }
}
