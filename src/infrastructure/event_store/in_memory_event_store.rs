use crate::infrastructure::event_store::event_store::{EventStore, EventStoreError};
use crate::infrastructure::event_store::stored_event::StoredEvent;
use crate::infrastructure::event_store::stream_key::StreamKey;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct InMemoryEventStore<TEvent: Serialize + Send + Sync + Clone> {
    data: Arc<Mutex<HashMap<StreamKey, Vec<StoredEvent<TEvent>>>>>,
    is_offline: bool,
}

impl<TEvent: Serialize + Send + Sync + Clone> InMemoryEventStore<TEvent> {
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

impl<TEvent: Serialize + Send + Sync + Clone> EventStore<TEvent> for InMemoryEventStore<TEvent> {
    async fn append(&self, key: StreamKey, value: TEvent) -> Result<(), EventStoreError> {
        if self.is_offline {
            return Err(EventStoreError::AppendError);
        }
        let stored_event = StoredEvent::new(key.clone(), 1, value);
        let mut data = self.data.lock().await;
        data.entry(key).or_insert_with(Vec::new).push(stored_event);
        Ok(())
    }

    async fn load(
        &self,
        stream_key: StreamKey,
    ) -> Result<Vec<StoredEvent<TEvent>>, EventStoreError> {
        if self.is_offline {
            return Err(EventStoreError::LoadError);
        }

        let data = self.data.lock().await;
        let result = data.get(&stream_key).map(|v| v.clone()).unwrap_or_default();
        Ok(result)
    }
}

#[cfg(test)]
mod in_memory_event_store_tests {
    use super::*;
    use crate::domain::domain_event::DomainEvent;
    use rstest::{fixture, rstest};
    use uuid::Uuid;

    #[derive(Serialize, PartialEq, Debug, Clone)]
    struct TestPayload {
        id: Uuid,
    }

    #[fixture]
    fn user_created_event() -> DomainEvent<'static, TestPayload> {
        let aggregate_id = Uuid::now_v7();
        let payload = TestPayload { id: Uuid::now_v7() };
        DomainEvent::create("UserCreated", aggregate_id, payload)
    }

    #[rstest]
    #[tokio::test]
    async fn should_store_the_data(user_created_event: DomainEvent<'static, TestPayload>) {
        let event_store = InMemoryEventStore::new();
        let stream_key = StreamKey::new("users", user_created_event.aggregate_id);
        let result = event_store.append(stream_key, user_created_event).await;
        assert!(result.is_ok());
    }

    #[rstest]
    #[tokio::test]
    async fn should_fail_storing_if_event_store_is_offline(
        user_created_event: DomainEvent<'static, TestPayload>,
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
    async fn should_query_the_data(user_created_event: DomainEvent<'static, TestPayload>) {
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
    async fn should_fail_querying_if_database_is_offline() {
        let mut event_store =
            InMemoryEventStore::<StoredEvent<DomainEvent<'static, TestPayload>>>::new();
        event_store.go_offline();
        let stream_key = StreamKey::new("users", Uuid::now_v7());
        let result = event_store.load(stream_key).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), EventStoreError::LoadError);
    }
}
