use crate::core::domain::models::value_object::ValueObject;

use crate::cli::bounded_context::domain::value_objects::{
    aggregate_name::AggregateName,
    aggregate_layer::AggregateLayer,
};

pub struct AggregateValue {
    pub name: AggregateName,
    pub layers: Vec<AggregateLayer>,
}

pub struct Aggregate {
    value: AggregateValue,
}

impl ValueObject<AggregateValue> for Aggregate {
    fn new(value: AggregateValue) -> Self {
        Self { value }
    }

    fn get_value(&self) -> &AggregateValue {
        &self.value
    }

    fn is_equal(&self, other: &Self) -> bool {
        let layers_are_equal = self.value.layers
            .iter()
            .all(|layer| other.value.layers.iter().any(|other_layer| layer.is_equal(other_layer)));
        self.value.name.is_equal(&other.value.name) && layers_are_equal
    }
}
