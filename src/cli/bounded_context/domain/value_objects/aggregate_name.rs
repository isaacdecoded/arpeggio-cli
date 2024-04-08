use crate::core::domain::models::value_object::ValueObject;

#[derive(Clone)]
pub struct AggregateName {
    pub value: String,
}

impl AggregateName {
    const MAX_LENGTH: usize = 32;
}

impl ValueObject<String> for AggregateName {
    fn new(value: String) -> Self {
        if value.len() > Self::MAX_LENGTH {
            panic!("The name exceeds the maximum length of {} characters.", Self::MAX_LENGTH);
        }
        Self { value }
    }

    fn get_value(&self) -> &String {
        &self.value
    }

    fn is_equal(&self, other: &Self) -> bool {
        self.value == *other.get_value()
    }
}
