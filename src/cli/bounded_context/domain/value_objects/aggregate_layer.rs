use anyhow::Result;
use crate::core::domain::models::value_object::ValueObject;
use crate::cli::bounded_context::domain::enums::layer_name::LayerName;

use crate::cli::bounded_context::domain::value_objects::layer_component::LayerComponent;

#[derive(Clone)]
pub struct AggregateLayerValue {
    pub name: LayerName,
    pub components: Vec<LayerComponent>,
}

#[derive(Clone)]
pub struct AggregateLayer {
    value: AggregateLayerValue,
}

impl AggregateLayer {
    pub fn add_component(&mut self, component: LayerComponent) -> Result<()> {
        if self.value.components.iter().any(|c| c.is_equal(&component)) {
            return Err(anyhow::anyhow!("Component already exists"));
        }
        self.value.components.push(component);
        Ok(())
    }
}

impl ValueObject<AggregateLayerValue> for AggregateLayer {
    fn new(value: AggregateLayerValue) -> Self {
        Self { value }
    }

    fn get_value(&self) -> &AggregateLayerValue {
        &self.value
    }

    fn is_equal(&self, other: &Self) -> bool {
        self.value.name == other.value.name &&
            self.value.components
                .iter()
                .all(|c| other.value.components.iter().any(|oc| oc.is_equal(c)))
    }
}
