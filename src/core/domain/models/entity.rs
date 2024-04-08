use chrono::{ DateTime, Local };
use crate::core::domain::models::value_object::ValueObject;

pub trait Entity<Id: ValueObject<String>> {
    fn get_id(&self) -> &String;
    fn get_created_at(&self) -> &DateTime<Local>;
    fn get_updated_at(&self) -> Option<&DateTime<Local>>;
    fn update(&mut self);
}
