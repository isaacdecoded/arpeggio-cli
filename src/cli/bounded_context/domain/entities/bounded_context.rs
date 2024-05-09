use anyhow::Result;
use crate::{
    cli::bounded_context::domain::{
        entities::aggregate::Aggregate,
        enums::{ component_type::ComponentType, layer_name::LayerName },
        value_objects::{
            aggregate_layer::AggregateLayer,
            component_name::ComponentName,
            layer_component::{ LayerComponent, LayerComponentValue },
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
        aggregate_id: &IdentityObject,
        layers: &Vec<AggregateLayer>
    ) -> Result<()> {
        self.validate_aggregate_layers(layers)?;
        let aggregate = Aggregate::new(aggregate_id.to_owned(), layers.to_owned());
        self.validate_aggregate_id_duplication(&aggregate)?;
        self.aggregates.push(aggregate);
        Ok(())
    }

    pub fn add_aggregate_component(
        &mut self,
        aggregate_id: &IdentityObject,
        component_type: ComponentType,
        component_name: ComponentName
    ) -> Result<()> {
        let layer_name = self.get_layer_name(&component_type);
        let layer_component: LayerComponent = LayerComponent::new(LayerComponentValue {
            component_type,
            component_name,
        });
        let aggregate = self.aggregates
            .iter_mut()
            .find(|aggregate| { aggregate.get_id().is_equal(aggregate_id) })
            .unwrap();
        aggregate.add_layer_component(layer_name, layer_component)?;
        Ok(())
    }

    fn get_layer_name(&self, component_type: &ComponentType) -> LayerName {
        match component_type {
            ComponentType::Entities => LayerName::Domain,
            ComponentType::Events => LayerName::Domain,
            ComponentType::ValueObjects => LayerName::Domain,
            ComponentType::Repositories => LayerName::Domain,
            ComponentType::Services => LayerName::Domain,
            ComponentType::Commands => LayerName::Application,
            ComponentType::Queries => LayerName::Application,
            ComponentType::Subscribers => LayerName::Application,
            ComponentType::Controllers => LayerName::Adapters,
            ComponentType::Presenters => LayerName::Adapters,
        }
    }

    fn validate_aggregate_id_duplication(&self, new_aggregate: &Aggregate) -> Result<()> {
        if
            self.aggregates
                .iter()
                .any(|aggregate| aggregate.get_id().is_equal(new_aggregate.get_id()))
        {
            return Err(anyhow::anyhow!("Aggregate already exists"));
        }
        Ok(())
    }

    fn validate_aggregate_layers(&self, layers: &[AggregateLayer]) -> Result<()> {
        layers.iter().try_for_each(|layer| {
            let components = &layer.get_value().components;
            match layer.get_value().name {
                LayerName::Domain => {
                    let available_components = [
                        ComponentType::Entities,
                        ComponentType::Events,
                        ComponentType::ValueObjects,
                        ComponentType::Repositories,
                        ComponentType::Services,
                    ];
                    if
                        !components
                            .iter()
                            .all(|component| {
                                available_components.contains(&component.get_value().component_type)
                            })
                    {
                        return Err(anyhow::anyhow!("Invalid components for Domain layer"));
                    }
                }
                LayerName::Application => {
                    let available_components = [
                        ComponentType::Commands,
                        ComponentType::Queries,
                        ComponentType::Subscribers,
                    ];
                    if
                        !components
                            .iter()
                            .all(|component| {
                                available_components.contains(&component.get_value().component_type)
                            })
                    {
                        return Err(anyhow::anyhow!("Invalid components for Application layer"));
                    }
                }
                LayerName::Adapters => {
                    let available_components = [
                        ComponentType::Controllers,
                        ComponentType::Presenters,
                    ];
                    if
                        !components
                            .iter()
                            .all(|component| {
                                available_components.contains(&component.get_value().component_type)
                            })
                    {
                        return Err(anyhow::anyhow!("Invalid components for Adapters layer"));
                    }
                }
                LayerName::Infrastructure => {
                    let available_components = [
                        ComponentType::Repositories,
                        ComponentType::Services,
                    ];
                    if
                        !components
                            .iter()
                            .all(|component| {
                                available_components.contains(&component.get_value().component_type)
                            })
                    {
                        return Err(anyhow::anyhow!("Invalid components for Infrastructure layer"));
                    }
                }
            }
            Ok(())
        })
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
}
