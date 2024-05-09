use crate::core::domain::models::value_object::ValueObject;

pub trait Entity<Id: ValueObject<String>> {
    fn get_id(&self) -> &Id;
}
