use async_trait::async_trait;
use crate::{
    cli::bounded_context::domain::repositories::find_bounded_contexts_repository::FindBoundedContextsRepository,
    core::application::{
        use_case_input_port::UseCaseInputPort,
        use_case_output_port::UseCaseOutputPort,
    },
};

pub struct FindBoundedContextsRequestModel;

#[derive(Clone)]
pub struct BoundedContextReadModel {
    pub name: String,
    pub aggregates: Vec<String>,
}

pub struct FindBoundedContextsResponseModel {
    pub bounded_contexts: Vec<BoundedContextReadModel>,
}

pub struct FindBoundedContextsUseCase<'a> {
    repository: &'a dyn FindBoundedContextsRepository<BoundedContextReadModel>,
    output_port: &'a dyn UseCaseOutputPort<FindBoundedContextsResponseModel>,
}

impl<'a> FindBoundedContextsUseCase<'a> {
    pub fn new(
        repository: &'a dyn FindBoundedContextsRepository<BoundedContextReadModel>,
        output_port: &'a dyn UseCaseOutputPort<FindBoundedContextsResponseModel>
    ) -> Self {
        Self {
            repository,
            output_port,
        }
    }
}

#[async_trait]
impl<'a> UseCaseInputPort<FindBoundedContextsRequestModel> for FindBoundedContextsUseCase<'a> {
    async fn interact(&self, _request_model: FindBoundedContextsRequestModel) {
        let bounded_contexts = self.repository.list_bounded_contexts().await;
        match bounded_contexts {
            Ok(bounded_contexts) => {
                self.output_port.success(FindBoundedContextsResponseModel {
                    bounded_contexts,
                }).await;
            }
            Err(error) => {
                self.output_port.failure(Box::new(error)).await;
            }
        }
    }
}
