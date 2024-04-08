use async_trait::async_trait;
use std::error::Error;
use crate::core::domain::events::domain_event::DomainEvent;
use crate::core::domain::events::domain_event_subscriber::DomainEventSubscriber;

#[async_trait]
pub trait DomainEventBus: Send + Sync {
    async fn publish(&self, domain_events: Vec<Box<dyn DomainEvent>>) -> Result<(), Box<dyn Error>>;
    async fn add_subscribers(
        &mut self,
        subscribers: Vec<Box<dyn DomainEventSubscriber>>
    ) -> Result<(), Box<dyn Error>>;
}
