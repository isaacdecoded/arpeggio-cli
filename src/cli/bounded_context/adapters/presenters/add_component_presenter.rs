use std::error::Error;
use async_trait::async_trait;
use crate::core::application::use_case_output_port::UseCaseOutputPort;
use crate::cli::bounded_context::{
    domain::enums::component_type::ComponentType,
    application::commands::add_component_use_case::AddComponentResponseModel,
};

pub struct AddComponentPresenter;

impl AddComponentPresenter {
    fn get_component_type(&self, component_type: &ComponentType) -> String {
        (
            match component_type {
                ComponentType::Controllers => "Controller",
                ComponentType::Presenters => "Presenter",
                ComponentType::Commands => "Command",
                ComponentType::Queries => "Query",
                ComponentType::Subscribers => "Subscriber",
                ComponentType::Entities => "Entity",
                ComponentType::Events => "Event",
                ComponentType::Repositories => "Repository",
                ComponentType::Services => "Service",
                ComponentType::ValueObjects => "ValueObject",
            }
        ).to_string()
    }
}

#[async_trait]
impl UseCaseOutputPort<AddComponentResponseModel> for AddComponentPresenter {
    async fn success(&self, response_model: AddComponentResponseModel) {
        println!(
            "{} <{}> added successfully.",
            self.get_component_type(&response_model.component_type),
            response_model.component_name
        )
    }

    async fn failure(&self, error: Box<dyn Error + Send>) {
        eprintln!("Failed to add component due to: {}", error)
    }
}
