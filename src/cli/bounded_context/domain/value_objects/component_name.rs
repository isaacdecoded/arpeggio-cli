use crate::core::domain::models::value_object::ValueObject;

#[derive(Clone)]
pub struct ComponentName {
    value: String,
}

impl ValueObject<String> for ComponentName {
    fn new(value: String) -> Self {
        Self { value }
    }

    fn get_value(&self) -> &String {
        &self.value
    }

    fn is_equal(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
