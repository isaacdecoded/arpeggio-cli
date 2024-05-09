use std::error::Error;
use async_trait::async_trait;
use crate::core::application::use_case_output_port::UseCaseOutputPort;
use crate::cli::bounded_context::application::queries::find_bounded_contexts_use_case::{
    FindBoundedContextsResponseModel,
    BoundedContextReadModel,
};

pub struct FindBoundedContextsPresenter {
    bounded_contexts_catcher: Box<dyn Fn(Vec<BoundedContextReadModel>) + Sync + Send>,
}

impl FindBoundedContextsPresenter {
    pub fn new(
        bounded_contexts_catcher: impl Fn(Vec<BoundedContextReadModel>) + 'static + Send + Sync
    ) -> Self {
        Self {
            bounded_contexts_catcher: Box::new(bounded_contexts_catcher),
        }
    }
}

#[async_trait]
impl UseCaseOutputPort<FindBoundedContextsResponseModel> for FindBoundedContextsPresenter {
    async fn success(&self, response_model: FindBoundedContextsResponseModel) {
        (self.bounded_contexts_catcher)(response_model.bounded_contexts)
    }

    async fn failure(&self, error: Box<dyn Error + Send>) {
        eprintln!("Failed to find bounded contexts due to: {}", error)
    }
}
