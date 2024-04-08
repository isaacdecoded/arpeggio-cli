use async_trait::async_trait;
use anyhow::{ Result, Error };
use crate::core::application::use_case_output_port::UseCaseOutputPort;
use crate::cli::bounded_context::application::queries::find_bounded_contexts_use_case::FindBoundedContextsResponseModel;

pub struct FindBoundedContextsPresenter {
    bounded_contexts_catcher: Box<dyn Fn(Vec<String>) + Sync + Send>,
}

impl FindBoundedContextsPresenter {
    pub fn new(bounded_contexts_catcher: impl Fn(Vec<String>) + 'static + Send + Sync) -> Self {
        Self {
            bounded_contexts_catcher: Box::new(bounded_contexts_catcher),
        }
    }
}

#[async_trait]
impl UseCaseOutputPort<FindBoundedContextsResponseModel> for FindBoundedContextsPresenter {
    async fn success(&self, response_model: FindBoundedContextsResponseModel) -> Result<()> {
        (self.bounded_contexts_catcher)(response_model.bounded_contexts);
        Ok(())
    }

    async fn failure(&self, error: &Error) -> Result<()> {
        eprintln!("{}", error.to_string());
        Ok(())
    }
}
