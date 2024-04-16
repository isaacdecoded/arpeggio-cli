use std::error::Error;

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

    async fn try_interact(
        &self,
        request_model: CreateBoundedContextRequestModel
    ) -> Result<CreateBoundedContextResponseModel, Box<dyn Error + Send + Sync>> {
        let bounded_context = BoundedContext::new(
            IdentityObject::new(request_model.bounded_context_name),
            vec![]
        );

        self.repository.write_bounded_context(&bounded_context).await?;
        Ok(CreateBoundedContextResponseModel {
            bounded_context_id: bounded_context.get_id().to_string(),
        })
    }
}

#[async_trait]
impl<'a> UseCaseInputPort<CreateBoundedContextRequestModel> for CreateBoundedContextUseCase<'a> {
    async fn interact(&self, request_model: CreateBoundedContextRequestModel) {
        let result = self.try_interact(request_model).await;
        match result {
            Ok(response_model) => {
                self.output_port.success(response_model).await;
            }
            Err(error) => {
                self.output_port.failure(error).await;
            }
        }
    }
}
