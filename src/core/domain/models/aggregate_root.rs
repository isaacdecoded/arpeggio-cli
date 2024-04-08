use crate::core::domain::models::entity::Entity;
use crate::core::domain::events::domain_event::DomainEvent;
use crate::core::domain::models::value_object::ValueObject;

pub trait AggregateRoot<Id: ValueObject<String>>: Entity<Id> {
    fn add_domain_event(&mut self, domain_event: Box<dyn DomainEvent>);
    fn pull_domain_events(&mut self) -> Vec<Box<dyn DomainEvent>>;
}
