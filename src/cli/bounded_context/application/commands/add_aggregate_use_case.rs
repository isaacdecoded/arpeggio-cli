use std::error::Error;

use anyhow::Result;
use async_trait::async_trait;
use crate::{
    cli::bounded_context::domain::{
        enums::layer_name::LayerName,
        repositories::bounded_context_repository::BoundedContextRepository,
        value_objects::{
            aggregate_layer::{ AggregateLayer, AggregateLayerValue },
            layer_component::LayerComponent,
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
    pub layer_name: LayerName,
    pub components: Vec<LayerComponent>,
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
                name: LayerName::Domain,
                components: vec![],
            }),
            AggregateLayer::new(AggregateLayerValue {
                name: LayerName::Application,
                components: vec![],
            }),
            AggregateLayer::new(AggregateLayerValue {
                name: LayerName::Adapters,
                components: vec![],
            }),
            AggregateLayer::new(AggregateLayerValue {
                name: LayerName::Infrastructure,
                components: vec![],
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

    async fn try_interact(
        &self,
        request_model: AddAggregateRequestModel
    ) -> Result<AddAggregateResponseModel, Box<dyn Error + Send + Sync>> {
        let result = self.repository.read_bounded_context(
            &IdentityObject::new(request_model.bounded_context_name)
        ).await?;
        match result {
            Some(mut bounded_context) => {
                let aggregate_id = IdentityObject::new(request_model.aggregate_name);
                let layers = request_model.aggregate_layers.map_or_else(
                    || self.get_aggregate_layers(),
                    |layers| self.prepare_aggregate_layers(layers)
                );
                bounded_context.add_aggregate(&aggregate_id, &layers)?;
                self.repository.write_bounded_context(&bounded_context).await?;
                Ok(AddAggregateResponseModel {
                    aggregate_name: aggregate_id.get_value().to_string(),
                })
            }
            None => { Err("Bounded context not found".into()) }
        }
    }
}

#[async_trait]
impl<'a> UseCaseInputPort<AddAggregateRequestModel> for AddAggregateUseCase<'a> {
    async fn interact(&self, request_model: AddAggregateRequestModel) {
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
