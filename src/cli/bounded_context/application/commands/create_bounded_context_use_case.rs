use anyhow::Result;
use async_trait::async_trait;
use crate::{
    cli::bounded_context::domain::{
        entities::bounded_context::BoundedContext,
        repositories::bounded_context_repository::BoundedContextRepository,
    },
    core::{
        application::{
            use_case_input_port::UseCaseInputPort,
            use_case_output_port::UseCaseOutputPort,
        },
        domain::models::{
            entity::Entity,
            identity_object::IdentityObject,
            value_object::ValueObject,
        },
    },
};

pub struct CreateBoundedContextRequestModel {
    pub bounded_context_name: String,
}

pub struct CreateBoundedContextResponseModel {
    pub bounded_context_id: String,
}

pub struct CreateBoundedContextUseCase<'a> {
    repository: &'a dyn BoundedContextRepository,
    output_port: &'a dyn UseCaseOutputPort<CreateBoundedContextResponseModel>,
}

impl<'a> CreateBoundedContextUseCase<'a> {
    pub fn new(
        repository: &'a dyn BoundedContextRepository,
        output_port: &'a dyn UseCaseOutputPort<CreateBoundedContextResponseModel>
    ) -> Self {
        Self {
            repository,
            output_port,
        }
    }
}

#[async_trait]
impl<'a> UseCaseInputPort<CreateBoundedContextRequestModel> for CreateBoundedContextUseCase<'a> {
    async fn interact(&self, request_model: CreateBoundedContextRequestModel) -> Result<()> {
        let bounded_context = BoundedContext::new(
            IdentityObject::new(request_model.bounded_context_name),
            vec![]
        );

        if let Err(error) = self.repository.write_bounded_context(&bounded_context).await {
            self.output_port.failure(&error).await?;
            return Err(error);
        }

        self.output_port.success(CreateBoundedContextResponseModel {
            bounded_context_id: bounded_context.get_id().to_string(),
        }).await?;
        Ok(())
    }
}
