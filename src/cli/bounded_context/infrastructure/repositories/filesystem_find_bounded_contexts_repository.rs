use anyhow::Result;
use std::fs;
use async_trait::async_trait;
use crate::cli::bounded_context::{
    domain::repositories::find_bounded_contexts_repository::{
        FindBoundedContextsRepository,
        FindBoundedContextsRepositoryError,
    },
    application::queries::find_bounded_contexts_use_case::BoundedContextReadModel,
};

pub struct FilesystemFindBoundedContextsRepository;

impl FilesystemFindBoundedContextsRepository {
    const SOURCE_DIR: &'static str = "./src";
}

#[async_trait]
impl FindBoundedContextsRepository<BoundedContextReadModel>
for FilesystemFindBoundedContextsRepository {
    async fn list_bounded_contexts(
        &self
    ) -> Result<Vec<BoundedContextReadModel>, FindBoundedContextsRepositoryError> {
        let mut bounded_contexts: Vec<BoundedContextReadModel> = vec![];
        let bounded_context_directories = fs
            ::read_dir(Self::SOURCE_DIR)
            .map_err(|e| FindBoundedContextsRepositoryError::ListError(e.to_string()))?
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

        bounded_context_directories.iter().for_each(|name| {
            let aggregate_directories = fs
                ::read_dir(format!("{}/{}", Self::SOURCE_DIR, name))
                .map_err(|e| FindBoundedContextsRepositoryError::ListError(e.to_string()))
                .unwrap()
                .filter_map(Result::ok)
                .filter(|e|
                    e
                        .file_type()
                        .map(|t| t.is_dir())
                        .unwrap_or(false)
                )
                .filter_map(|e| e.file_name().into_string().ok())
                .collect::<Vec<_>>();
            bounded_contexts.push(BoundedContextReadModel {
                name: name.to_string(),
                aggregates: aggregate_directories,
            });
        });

        Ok(bounded_contexts)
    }
}
