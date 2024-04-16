use anyhow::Result;
use crate::{
    cli::bounded_context::domain::{
        enums::{
            aggregate_layer_component::AggregateLayerComponent,
            aggregate_layer_name::AggregateLayerName,
        },
        value_objects::{
            aggregate::{ Aggregate, AggregateValue },
            aggregate_layer::AggregateLayer,
            aggregate_name::AggregateName,
        },
    },
    core::domain::{
        events::domain_event::DomainEvent,
        models::{
            aggregate_root::AggregateRoot,
            entity::Entity,
            identity_object::IdentityObject,
            value_object::ValueObject,
        },
    },
};

pub struct BoundedContext {
    pub id: IdentityObject,
    pub aggregates: Vec<Aggregate>,
}

impl BoundedContext {
    pub fn new(id: IdentityObject, aggregates: Vec<Aggregate>) -> Self {
        Self { id, aggregates }
    }

    pub fn add_aggregate(
        &mut self,
        name: &AggregateName,
        layers: &Vec<AggregateLayer>
    ) -> Result<()> {
        self.validate_aggregate_layers(layers)?;
        let aggregate = Aggregate::new(AggregateValue {
            name: name.to_owned(),
            layers: layers.to_owned(),
        });
        self.validate_aggregate_id_duplication(&aggregate)?;
        self.aggregates.push(aggregate);
        Ok(())
    }

    fn validate_aggregate_id_duplication(&self, new_aggregate: &Aggregate) -> Result<()> {
        if self.aggregates.iter().any(|aggregate| aggregate.is_equal(new_aggregate)) {
            return Err(anyhow::anyhow!("Aggregate already exists"));
        }
        Ok(())
    }

    fn validate_aggregate_layers(&self, layers: &[AggregateLayer]) -> Result<()> {
        layers.iter().for_each(|layer| {
            let components = &layer.get_value().components;
            match layer.get_value().name {
                AggregateLayerName::Domain => {
                    let available_components = [
                        AggregateLayerComponent::Entities,
                        AggregateLayerComponent::Events,
                        AggregateLayerComponent::ValueObjects,
                        AggregateLayerComponent::Repositories,
                        AggregateLayerComponent::Services,
                    ];
                    if
                        !components
                            .iter()
                            .all(|component| { available_components.contains(component) })
                    {
                        panic!("Invalid components for Domain layer");
                    }
                }
                AggregateLayerName::Application => {
                    let available_components = [
                        AggregateLayerComponent::Commands,
                        AggregateLayerComponent::Queries,
                        AggregateLayerComponent::Subscribers,
                    ];
                    if
                        !components
                            .iter()
                            .all(|component| { available_components.contains(component) })
                    {
                        panic!("Invalid components for Application layer");
                    }
                }
                AggregateLayerName::Adapters => {
                    let available_components = [
                        AggregateLayerComponent::Controllers,
                        AggregateLayerComponent::Presenters,
                    ];
                    if
                        !components
                            .iter()
                            .all(|component| { available_components.contains(component) })
                    {
                        panic!("Invalid components for Adapters layer");
                    }
                }
                AggregateLayerName::Infrastructure => {
                    let available_components = [
                        AggregateLayerComponent::Repositories,
                        AggregateLayerComponent::Services,
                    ];
                    if
                        !components
                            .iter()
                            .all(|component| { available_components.contains(component) })
                    {
                        panic!("Invalid components for Infrastructure layer");
                    }
                }
            }
        });
        Ok(())
    }
}

impl AggregateRoot<IdentityObject> for BoundedContext {
    fn add_domain_event(&mut self, _domain_event: Box<dyn DomainEvent>) {
        todo!()
    }

    fn pull_domain_events(&mut self) -> Vec<Box<dyn DomainEvent>> {
        todo!()
    }
}

impl Entity<IdentityObject> for BoundedContext {
    fn get_id(&self) -> &IdentityObject {
        &self.id
    }

    fn update(&mut self) {
        todo!()
    }
}
