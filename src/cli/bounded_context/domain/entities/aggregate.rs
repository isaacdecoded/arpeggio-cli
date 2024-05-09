use anyhow::Result;
use crate::{
    cli::bounded_context::domain::{
        enums::layer_name::LayerName,
        value_objects::{ aggregate_layer::AggregateLayer, layer_component::LayerComponent },
    },
    core::domain::models::{
        entity::Entity,
        identity_object::IdentityObject,
        value_object::ValueObject,
    },
};

pub struct Aggregate {
    id: IdentityObject,
    layers: Vec<AggregateLayer>,
}

impl Aggregate {
    pub fn new(id: IdentityObject, layers: Vec<AggregateLayer>) -> Self {
        Self { id, layers }
    }

    pub fn get_layers(&self) -> &Vec<AggregateLayer> {
        &self.layers
    }

    pub fn add_layer_component(
        &mut self,
        layer_name: LayerName,
        component: LayerComponent
    ) -> Result<()> {
        let layer = self.layers
            .iter_mut()
            .find(|layer| layer_name.eq(&layer.get_value().name))
            .unwrap();
        layer.add_component(component)?;
        Ok(())
    }
}

impl Entity<IdentityObject> for Aggregate {
    fn get_id(&self) -> &IdentityObject {
        &self.id
    }
}
