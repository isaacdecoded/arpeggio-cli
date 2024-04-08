use anyhow::Result;
use std::fs;
use async_trait::async_trait;
use crate::cli::bounded_context::domain::repositories::find_bounded_contexts_repository::FindBoundedContextsRepository;

pub struct FilesystemFindBoundedContextsRepository;

impl FilesystemFindBoundedContextsRepository {
    const SOURCE_DIR: &'static str = "./src";
}

#[async_trait]
impl FindBoundedContextsRepository<String> for FilesystemFindBoundedContextsRepository {
    async fn list_bounded_contexts(&self) -> Result<Vec<String>> {
        let bounded_context_directories = fs
            ::read_dir(Self::SOURCE_DIR)?
            .filter_map(Result::ok)
            .filter(|e|
                e
                    .file_type()
                    .map(|t| t.is_dir())
                    .unwrap_or(false)
            )
            .filter_map(|e| e.file_name().into_string().ok())
            .filter(|name| name != "core")
            .collect::<Vec<_>>();

        Ok(bounded_context_directories)
    }
}
