use crate::core::base_payload::BasePayload;
use crate::domain::domain_event::DomainEvent;
use async_trait::async_trait;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum EventProducerError {
    #[error("publish event failed")]
    PublishEventFailed,
}

#[async_trait]
pub trait EventProducer<TEventPayload>: Send + Sync + Clone + 'static
where
    TEventPayload: BasePayload + Send + Sync,
{
    async fn publish(
        &self,
        stream: String,
        event: DomainEvent<TEventPayload>,
    ) -> Result<(), EventProducerError>;
}
