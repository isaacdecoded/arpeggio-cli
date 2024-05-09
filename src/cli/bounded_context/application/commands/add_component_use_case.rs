use std::error::Error;

use anyhow::Result;
use async_trait::async_trait;
use crate::{
    cli::bounded_context::domain::{
        enums::component_type::ComponentType,
        repositories::bounded_context_repository::BoundedContextRepository,
        value_objects::component_name::ComponentName,
    },
    core::{
        application::{
            use_case_input_port::UseCaseInputPort,
            use_case_output_port::UseCaseOutputPort,
        },
        domain::models::{ identity_object::IdentityObject, value_object::ValueObject },
    },
};

pub struct ComponentRequestModel {
    pub component_type: ComponentType,
    pub component_name: String,
}

pub struct AddComponentRequestModel {
    pub bounded_context_name: String,
    pub aggregate_name: String,
    pub component: ComponentRequestModel,
}

pub struct AddComponentResponseModel {
    pub component_type: ComponentType,
    pub component_name: String,
}

pub struct AddComponentUseCase<'a> {
    repository: &'a dyn BoundedContextRepository,
    output_port: &'a dyn UseCaseOutputPort<AddComponentResponseModel>,
}

impl<'a> AddComponentUseCase<'a> {
    pub fn new(
        repository: &'a dyn BoundedContextRepository,
        output_port: &'a dyn UseCaseOutputPort<AddComponentResponseModel>
    ) -> Self {
        Self {
            repository,
            output_port,
        }
    }

    /*
    fn get_aggregate_layers(&self) -> Vec<AggregateLayer> {
        vec![
            AggregateLayer::new(AggregateLayerValue {
                name: AggregateLayerName::Domain,
                components: vec![
                    AggregateLayerComponent::Entities,
                    AggregateLayerComponent::Events,
                    AggregateLayerComponent::ValueObjects,
                    AggregateLayerComponent::Repositories,
                    AggregateLayerComponent::Services
                ],
            }),
            AggregateLayer::new(AggregateLayerValue {
                name: AggregateLayerName::Application,
                components: vec![
                    AggregateLayerComponent::Commands,
                    AggregateLayerComponent::Queries,
                    AggregateLayerComponent::Subscribers
                ],
            }),
            AggregateLayer::new(AggregateLayerValue {
                name: AggregateLayerName::Adapters,
                components: vec![
                    AggregateLayerComponent::Controllers,
                    AggregateLayerComponent::Presenters
                ],
            }),
            AggregateLayer::new(AggregateLayerValue {
                name: AggregateLayerName::Infrastructure,
                components: vec![
                    AggregateLayerComponent::Repositories,
                    AggregateLayerComponent::Services
                ],
            })
        ]
    }

    fn prepare_aggregate_layers(
        &self,
        layers: Vec<AddAggregateLayerRequestModel>
    ) -> Vec<AggregateLayer> {
        layers
            .iter()
            .map(|layer| {
                AggregateLayer::new(AggregateLayerValue {
                    name: layer.layer_name.clone(),
                    components: layer.components.clone(),
                })
            })
            .collect()
    }
    */

    async fn try_interact(
        &self,
        request_model: AddComponentRequestModel
    ) -> Result<AddComponentResponseModel, Box<dyn Error + Send + Sync>> {
        let result = self.repository.read_bounded_context(
            &IdentityObject::new(request_model.bounded_context_name)
        ).await?;
        match result {
            Some(mut bounded_context) => {
                let aggregate_id = IdentityObject::new(request_model.aggregate_name);
                let component_name = ComponentName::new(
                    request_model.component.component_name.clone()
                );
                bounded_context.add_aggregate_component(
                    &aggregate_id,
                    request_model.component.component_type.clone(),
                    component_name
                )?;

                self.repository.write_bounded_context(&bounded_context).await?;
                Ok(AddComponentResponseModel {
                    component_type: request_model.component.component_type,
                    component_name: request_model.component.component_name,
                })
            }
            None => { Err("Bounded context not found".into()) }
        }
    }
}

#[async_trait]
impl<'a> UseCaseInputPort<AddComponentRequestModel> for AddComponentUseCase<'a> {
    async fn interact(&self, request_model: AddComponentRequestModel) {
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
