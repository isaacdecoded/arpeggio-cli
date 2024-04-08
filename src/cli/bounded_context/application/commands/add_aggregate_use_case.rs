use anyhow::Result;
use async_trait::async_trait;
use crate::{
    cli::bounded_context::domain::{
        enums::{
            aggregate_layer_component::AggregateLayerComponent,
            aggregate_layer_name::AggregateLayerName,
        },
        repositories::bounded_context_repository::BoundedContextRepository,
        value_objects::{
            aggregate_layer::{ AggregateLayer, AggregateLayerValue },
            aggregate_name::AggregateName,
        },
    },
    core::{
        application::{
            use_case_input_port::UseCaseInputPort,
            use_case_output_port::UseCaseOutputPort,
        },
        domain::models::{ identity_object::IdentityObject, value_object::ValueObject },
    },
};

pub struct AddAggregateLayerRequestModel {
    pub layer_name: AggregateLayerName,
    pub components: Vec<AggregateLayerComponent>,
}

pub struct AddAggregateRequestModel {
    pub bounded_context_name: String,
    pub aggregate_name: String,
    pub aggregate_layers: Option<Vec<AddAggregateLayerRequestModel>>,
}

pub struct AddAggregateResponseModel {
    pub aggregate_name: String,
}

pub struct AddAggregateUseCase<'a> {
    repository: &'a dyn BoundedContextRepository,
    output_port: &'a dyn UseCaseOutputPort<AddAggregateResponseModel>,
}

impl<'a> AddAggregateUseCase<'a> {
    pub fn new(
        repository: &'a dyn BoundedContextRepository,
        output_port: &'a dyn UseCaseOutputPort<AddAggregateResponseModel>
    ) -> Self {
        Self {
            repository,
            output_port,
        }
    }

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
}

#[async_trait]
impl<'a> UseCaseInputPort<AddAggregateRequestModel> for AddAggregateUseCase<'a> {
    async fn interact(&self, request_model: AddAggregateRequestModel) -> Result<()> {
        let result = self.repository.read_bounded_context(
            &IdentityObject::new(request_model.bounded_context_name)
        ).await?;
        match result {
            Some(mut bounded_context) => {
                let name = AggregateName::new(request_model.aggregate_name);
                let layers = request_model.aggregate_layers.map_or_else(
                    || self.get_aggregate_layers(),
                    |layers| self.prepare_aggregate_layers(layers)
                );
                bounded_context.add_aggregate(&name, &layers)?;
                if let Err(error) = self.repository.write_bounded_context(&bounded_context).await {
                    self.output_port.failure(&error).await?;
                    return Err(error);
                }
                self.output_port.success(AddAggregateResponseModel {
                    aggregate_name: name.get_value().to_string(),
                }).await?;
            }
            None => {
                self.output_port.failure(&anyhow::anyhow!("Bounded context not found")).await?;
            }
        }
        Ok(())
    }
}
