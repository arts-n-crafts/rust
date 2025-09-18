pub mod in_memory_event_store;
pub mod stored_event;
pub mod stream_key;
use crate::core::base_payload::BasePayload;
use crate::domain::domain_event::DomainEvent;
use crate::infrastructure::event_store::stream_key::StreamKey;
use async_trait::async_trait;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum EventStoreError {
    #[error("loading failed")]
    LoadFailed,
    #[error("appending failed")]
    AppendFailed,
}

#[async_trait]
pub trait EventStore<TEvent>: Send + Sync + Clone + 'static
where
    TEvent: BasePayload + Send + Sync,
{
    async fn append(
        &self,
        key: StreamKey,
        value: DomainEvent<TEvent>,
    ) -> Result<(), EventStoreError>;

    async fn load(
        &self,
        stream_key: StreamKey,
    ) -> Result<Vec<DomainEvent<TEvent>>, EventStoreError>;
}
