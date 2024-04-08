use async_trait::async_trait;
use std::error::Error;
use crate::core::domain::events::{
    domain_event::DomainEvent,
    domain_event_bus::DomainEventBus,
    domain_event_subscriber::DomainEventSubscriber,
};
use std::collections::HashMap;

#[derive(Default)]
pub struct InMemoryDomainEventBus {
    subscribers: HashMap<String, Vec<Box<dyn DomainEventSubscriber>>>,
}

#[async_trait]
impl DomainEventBus for InMemoryDomainEventBus {
    async fn publish(
        &self,
        domain_events: Vec<Box<dyn DomainEvent>>
    ) -> Result<(), Box<dyn Error>> {
        for domain_event in domain_events.iter() {
            if let Some(subscribers) = self.subscribers.get(&domain_event.get_name()) {
                for subscriber in subscribers.iter() {
                    subscriber.on(domain_event.as_ref()).await?;
                }
            }
        }
        Ok(())
    }

    async fn add_subscribers(
        &mut self,
        subscribers: Vec<Box<dyn DomainEventSubscriber>>
    ) -> Result<(), Box<dyn Error>> {
        for subscriber in subscribers {
            let subscriber_domain_event_name = subscriber.subscribed_to();
            if let Some(subscribers) = self.subscribers.get_mut(&subscriber_domain_event_name) {
                subscribers.push(subscriber);
            } else {
                let subscribers = vec![subscriber];
                self.subscribers.insert(subscriber_domain_event_name, subscribers);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{
        domain::events::{
            domain_event_bus::DomainEventBus,
            domain_event::DomainEvent,
            domain_event_subscriber::DomainEventSubscriber,
        },
        infrastructure::in_memory_domain_event_bus::InMemoryDomainEventBus,
    };
    use chrono::prelude::{ DateTime, Local };
    use async_trait::async_trait;
    use std::any::Any;

    struct TestSubscriber;
    struct TestDomainEvent {
        aggregate_root_id: String,
        occurring_time: DateTime<Local>,
    }

    impl TestSubscriber {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl TestDomainEvent {
        pub fn new() -> Self {
            Self {
                aggregate_root_id: "aggregate_root_id".to_string(),
                occurring_time: Local::now(),
            }
        }
    }

    #[async_trait]
    impl DomainEventSubscriber for TestSubscriber {
        fn subscribed_to(&self) -> String {
            "test".to_string()
        }

        async fn on(
            &self,
            _domain_event: &dyn DomainEvent
        ) -> Result<(), Box<dyn std::error::Error>> {
            Ok(())
        }
    }

    impl DomainEvent for TestDomainEvent {
        fn get_name(&self) -> String {
            "test".to_string()
        }

        fn get_aggregate_root_id(&self) -> &String {
            &self.aggregate_root_id
        }

        fn get_occurring_time(&self) -> &DateTime<Local> {
            &self.occurring_time
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    #[tokio::test]
    async fn should_initialize_valid_instance() {
        let mut in_memory_domain_event_bus = InMemoryDomainEventBus::default();
        let subscriber = TestSubscriber::new();
        let domain_events: Vec<Box<dyn DomainEvent>> = vec![Box::new(TestDomainEvent::new())];
        let add_subscriber_result = in_memory_domain_event_bus.add_subscribers(
            vec![Box::new(subscriber)]
        ).await;
        let publish_result = in_memory_domain_event_bus.publish(domain_events).await;
        assert!(add_subscriber_result.is_ok());
        assert!(publish_result.is_ok());
    }
}
