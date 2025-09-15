use crate::infrastructure::event_store::event_store::{EventStore, EventStoreError};
use crate::infrastructure::event_store::stream_key::StreamKey;
use chrono::Utc;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct StoredEvent<TEvent>
where
    TEvent: Serialize + Send + Sync + Clone,
{
    pub id: Uuid,
    stream_key: StreamKey,
    version: u8,
    pub event: TEvent,
    timestamp: i64,
}

impl<TEvent: Serialize + DeserializeOwned + Send + Sync + Clone> StoredEvent<TEvent> {
    pub fn new(stream_key: StreamKey, version: u8, event: TEvent) -> Self {
        StoredEvent {
            id: Uuid::now_v7(),
            stream_key,
            version,
            event,
            timestamp: Utc::now().timestamp_millis(),
        }
    }
}
pub struct InMemoryEventStore<TEvent>
where
    TEvent: Serialize + Send + Sync + Clone,
{
    data: Arc<Mutex<HashMap<StreamKey, Vec<StoredEvent<TEvent>>>>>,
    is_offline: bool,
}

impl<TEvent> InMemoryEventStore<TEvent>
where
    TEvent: Serialize + Send + Sync + Clone,
{
    pub fn new() -> Self {
        InMemoryEventStore {
            is_offline: false,
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn go_offline(&mut self) {
        self.is_offline = true
    }
}

impl<TEvent> EventStore<TEvent> for InMemoryEventStore<TEvent>
where
    TEvent: Serialize + DeserializeOwned + Send + Sync + Clone + 'static,
{
    async fn append(&self, stream_key: StreamKey, event: TEvent) -> Result<(), EventStoreError> {
        if self.is_offline {
            return Err(EventStoreError::AppendError);
        }
        let stored_event = StoredEvent::new(stream_key.clone(), 1, event);
        let mut data = self.data.lock().await;
        data.entry(stream_key)
            .or_insert_with(Vec::new)
            .push(stored_event);
        Ok(())
    }

    async fn load(&self, stream_key: StreamKey) -> Result<Vec<TEvent>, EventStoreError> {
        if self.is_offline {
            return Err(EventStoreError::LoadError);
        }

        let data = self.data.lock().await;
        let result = data.get(&stream_key).map(|v| v.clone()).unwrap_or_default();
        Ok(result
            .into_iter()
            .map(|stored_event| stored_event.event)
            .collect())
    }
}

#[cfg(test)]
mod stored_event_test {
    use super::*;
    use crate::domain::domain_event::DomainEvent;
    use rstest::rstest;

    #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
    struct TestPayload {
        id: Uuid,
    }

    #[rstest]
    fn it_should_create_a_stored_event() {
        let aggregate_id = Uuid::now_v7();
        let stream_key = StreamKey::new("users", aggregate_id);
        let payload = TestPayload { id: Uuid::now_v7() };
        let event = DomainEvent::create("UserCreated", aggregate_id, payload);
        let stored_event = StoredEvent::new(stream_key.clone(), 1, event.clone());
        assert_eq!(stored_event.stream_key, stream_key);
        assert_eq!(stored_event.version, 1);
        assert_eq!(stored_event.event, event);
    }
}

#[cfg(test)]
mod in_memory_event_store_tests {
    use futures::future::join_all;
    use super::*;
    use crate::domain::domain_event::DomainEvent;
    use rstest::{fixture, rstest};
    use serde::Deserialize;
    use uuid::Uuid;

    #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
    struct TestPayload {
        id: Uuid,
    }

    #[fixture]
    fn user_created_event() -> DomainEvent<TestPayload> {
        let aggregate_id = Uuid::now_v7();
        let payload = TestPayload { id: Uuid::now_v7() };
        DomainEvent::create("UserCreated", aggregate_id, payload)
    }

    #[rstest]
    #[tokio::test]
    async fn should_store_the_data(user_created_event: DomainEvent<TestPayload>) {
        let event_store = InMemoryEventStore::new();
        let stream_key = StreamKey::new("users", user_created_event.aggregate_id);
        let result = event_store.append(stream_key, user_created_event).await;
        assert!(result.is_ok());
    }

    #[rstest]
    #[tokio::test]
    async fn should_fail_storing_if_event_store_is_offline(
        user_created_event: DomainEvent<TestPayload>,
    ) {
        let mut event_store = InMemoryEventStore::new();
        event_store.go_offline();
        let stream_key = StreamKey::new("users", user_created_event.aggregate_id);
        let result = event_store.append(stream_key, user_created_event).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), EventStoreError::AppendError);
    }

    #[rstest]
    #[tokio::test]
    async fn should_query_the_data(user_created_event: DomainEvent<TestPayload>) {
        let event_store = InMemoryEventStore::new();
        let stream_key = StreamKey::new("users", user_created_event.aggregate_id);
        event_store
            .append(stream_key.clone(), user_created_event)
            .await
            .expect("unexpected store failed");

        let result = event_store.load(stream_key).await;
        assert!(result.is_ok());
    }

    #[rstest]
    #[tokio::test]
    async fn should_query_all_the_data_in_the_stream(user_created_event: DomainEvent<TestPayload>) {
        let event_store = InMemoryEventStore::new();
        let stream_key = StreamKey::new("users", user_created_event.aggregate_id);
        let iterations = 1_000;
        join_all(
            (0..iterations)
                .map(|_| event_store.append(stream_key.clone(), user_created_event.clone()))
                .collect::<Vec<_>>()
        ).await;
        let result = event_store.load(stream_key).await;
        assert!(result.is_ok());
        let events = result.expect("Failed to load events");
        assert_eq!(events.len(), iterations);
    }

    #[rstest]
    #[tokio::test]
    async fn should_fail_querying_if_database_is_offline() {
        let mut event_store = InMemoryEventStore::<StoredEvent<DomainEvent<TestPayload>>>::new();
        event_store.go_offline();
        let stream_key = StreamKey::new("users", Uuid::now_v7());
        let result = event_store.load(stream_key).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), EventStoreError::LoadError);
    }
}
