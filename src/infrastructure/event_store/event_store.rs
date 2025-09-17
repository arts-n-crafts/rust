use crate::core::base_payload::BasePayload;
use crate::domain::domain_event::DomainEvent;
use crate::infrastructure::event_store::stream_key::StreamKey;
use std::future::Future;

#[derive(Debug, PartialEq)]
pub enum EventStoreError {
    AppendError,
    LoadError,
}

pub trait EventStore<TEvent>
where
    TEvent: BasePayload,
{
    fn append(
        &self,
        key: StreamKey,
        value: DomainEvent<TEvent>,
    ) -> impl Future<Output = Result<(), EventStoreError>>;

    fn load(
        &self,
        stream_key: StreamKey,
    ) -> impl Future<Output = Result<Vec<DomainEvent<TEvent>>, EventStoreError>>;
}
