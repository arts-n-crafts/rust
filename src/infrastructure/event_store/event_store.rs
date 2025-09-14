use serde::Serialize;
use crate::infrastructure::event_store::stored_event::StoredEvent;
use crate::infrastructure::event_store::stream_key::StreamKey;

#[derive(Debug, PartialEq)]
pub enum EventStoreError {
    AppendError,
    LoadError,
}

pub trait EventStore<TEvent: Serialize + Send + Sync + Clone> {
    async fn append(&self, key: StreamKey, value: TEvent) -> Result<(), EventStoreError>;

    async fn load(&self, stream_key: StreamKey) -> Result<Vec<StoredEvent<TEvent>>, EventStoreError>;
}
