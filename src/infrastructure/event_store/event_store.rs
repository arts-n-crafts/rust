use std::future::Future;
use serde::Serialize;
use crate::infrastructure::event_store::stream_key::StreamKey;

#[derive(Debug, PartialEq)]
pub enum EventStoreError {
    AppendError,
    LoadError,
}

pub trait EventStore<TEvent: Serialize + Send + Sync + Clone> {
    fn append(&self, key: StreamKey, value: TEvent) -> impl Future<Output=Result<(), EventStoreError>>;

    fn load(&self, stream_key: StreamKey) -> impl Future<Output=Result<Vec<TEvent>, EventStoreError>>;
}
