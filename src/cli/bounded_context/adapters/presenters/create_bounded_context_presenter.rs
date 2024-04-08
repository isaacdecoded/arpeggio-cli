use async_trait::async_trait;
use anyhow::{ Result, Error };
use crate::core::application::use_case_output_port::UseCaseOutputPort;
use crate::cli::bounded_context::application::commands::create_bounded_context_use_case::CreateBoundedContextResponseModel;

pub struct CreateBoundedContextPresenter;

#[async_trait]
impl UseCaseOutputPort<CreateBoundedContextResponseModel> for CreateBoundedContextPresenter {
    async fn success(&self, response_model: CreateBoundedContextResponseModel) -> Result<()> {
        println!("Bounded Context <{}> created successfully.", response_model.bounded_context_id);
        Ok(())
    }

    async fn failure(&self, error: &Error) -> Result<()> {
        eprintln!("{}", error.to_string());
        Ok(())
    }
}
