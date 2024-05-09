use crate::{
    cli::bounded_context::domain::enums::component_type::ComponentType,
    core::domain::models::value_object::ValueObject,
};

use crate::cli::bounded_context::domain::value_objects::component_name::ComponentName;

#[derive(Clone)]
pub struct LayerComponentValue {
    pub component_type: ComponentType,
    pub component_name: ComponentName,
}

#[derive(Clone)]
pub struct LayerComponent {
    value: LayerComponentValue,
}

impl ValueObject<LayerComponentValue> for LayerComponent {
    fn new(value: LayerComponentValue) -> Self {
        Self { value }
    }

    fn get_value(&self) -> &LayerComponentValue {
        &self.value
    }

    fn is_equal(&self, other: &Self) -> bool {
        self.value.component_name.is_equal(&other.value.component_name) &&
            self.value.component_type.eq(&other.value.component_type)
    }
}
