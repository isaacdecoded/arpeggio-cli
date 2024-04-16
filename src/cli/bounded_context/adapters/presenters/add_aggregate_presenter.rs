use std::error::Error;
use async_trait::async_trait;
use crate::core::application::use_case_output_port::UseCaseOutputPort;
use crate::cli::bounded_context::application::commands::add_aggregate_use_case::AddAggregateResponseModel;

pub struct AddAggregatePresenter;

#[async_trait]
impl UseCaseOutputPort<AddAggregateResponseModel> for AddAggregatePresenter {
    async fn success(&self, response_model: AddAggregateResponseModel) {
        println!("Aggregate <{}> added successfully.", response_model.aggregate_name)
    }

    async fn failure(&self, error: Box<dyn Error + Send>) {
        eprintln!("{}", error)
    }
}
