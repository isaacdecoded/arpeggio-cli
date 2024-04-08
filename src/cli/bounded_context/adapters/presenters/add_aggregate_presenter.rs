use async_trait::async_trait;
use anyhow::{ Result, Error };
use crate::core::application::use_case_output_port::UseCaseOutputPort;
use crate::cli::bounded_context::application::commands::add_aggregate_use_case::AddAggregateResponseModel;

pub struct AddAggregatePresenter;

#[async_trait]
impl UseCaseOutputPort<AddAggregateResponseModel> for AddAggregatePresenter {
    async fn success(&self, response_model: AddAggregateResponseModel) -> Result<()> {
        println!("Aggregate <{}> added successfully.", response_model.aggregate_name);
        Ok(())
    }

    async fn failure(&self, error: &Error) -> Result<()> {
        eprintln!("{}", error.to_string());
        Ok(())
    }
}
