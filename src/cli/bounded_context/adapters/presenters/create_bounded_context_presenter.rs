use std::error::Error;
use async_trait::async_trait;
use crate::core::application::use_case_output_port::UseCaseOutputPort;
use crate::cli::bounded_context::application::commands::create_bounded_context_use_case::CreateBoundedContextResponseModel;

pub struct CreateBoundedContextPresenter;

#[async_trait]
impl UseCaseOutputPort<CreateBoundedContextResponseModel> for CreateBoundedContextPresenter {
    async fn success(&self, response_model: CreateBoundedContextResponseModel) {
        println!("Bounded Context <{}> created successfully.", response_model.bounded_context_id)
    }

    async fn failure(&self, error: Box<dyn Error + Send>) {
        eprintln!("{}", error)
    }
}
