use crate::core::base_payload::BasePayload;
use crate::domain::domain_event::DomainEvent;
use crate::infrastructure::event_bus::event_producer::{EventProducer, EventProducerError};
use async_trait::async_trait;

#[derive(Clone)]
pub struct InMemoryEventProducer;

#[async_trait]
impl<TEventPayload> EventProducer<TEventPayload> for InMemoryEventProducer
where
    TEventPayload: BasePayload + 'static,
{
    async fn publish(
        &self,
        stream: String,
        event: DomainEvent<TEventPayload>,
    ) -> Result<(), EventProducerError> {
        Ok(())
        todo!("Implement once EventConsumer is designed")
    }
}

#[cfg(test)]
mod in_memory_event_bus_tests {
    use super::*;
    use rstest::{fixture, rstest};
    use serde::{Deserialize, Serialize};
    use strum_macros::AsRefStr;
    use uuid::Uuid;

    #[derive(Clone, Serialize, Deserialize, Debug, PartialEq, AsRefStr)]
    pub enum UserEventPayload {
        UserCreated { id: String, name: String },
        UserLiked,
    }

    #[fixture]
    fn user_liked_event() -> DomainEvent<UserEventPayload> {
        let aggregate_id = Uuid::now_v7();
        let payload = UserEventPayload::UserLiked;
        DomainEvent::create(aggregate_id.to_string(), payload)
    }

    #[rstest]
    #[tokio::test]
    #[should_panic(expected = "Implement once EventConsumer is designed")]
    async fn it_should_implement_publish(user_liked_event: DomainEvent<UserEventPayload>) {
        let producer = InMemoryEventProducer::new();
        let result = producer
            .publish("users".to_string(), user_liked_event)
            .await;
    }
}
