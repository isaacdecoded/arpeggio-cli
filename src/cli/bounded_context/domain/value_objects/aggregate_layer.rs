use crate::core::domain::models::value_object::ValueObject;
use crate::cli::bounded_context::domain::enums::{
    aggregate_layer_name::AggregateLayerName,
    aggregate_layer_component::AggregateLayerComponent,
};

#[derive(Clone)]
pub struct AggregateLayerValue {
    pub name: AggregateLayerName,
    pub components: Vec<AggregateLayerComponent>,
}

#[derive(Clone)]
pub struct AggregateLayer {
    value: AggregateLayerValue,
}

impl ValueObject<AggregateLayerValue> for AggregateLayer {
    fn new(value: AggregateLayerValue) -> Self {
        Self { value }
    }

    fn get_value(&self) -> &AggregateLayerValue {
        &self.value
    }

    fn is_equal(&self, other: &Self) -> bool {
        self.value.name == other.value.name && self.value.components == other.value.components
    }
}
