use crate::core::base_payload::BasePayload;
use crate::domain::domain_event::DomainEvent;
use crate::infrastructure::event_store::stored_event::StoredEvent;
use crate::infrastructure::event_store::stream_key::StreamKey;
use crate::infrastructure::event_store::{EventStore, EventStoreError};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct InMemoryEventStore<TEventPayload>
where
    TEventPayload: BasePayload,
{
    data: Arc<Mutex<HashMap<StreamKey, Vec<StoredEvent<TEventPayload>>>>>,
    is_offline: bool,
}

impl<TEventPayload> InMemoryEventStore<TEventPayload>
where
    TEventPayload: BasePayload,
{
    fn new() -> Self {
        InMemoryEventStore {
            is_offline: false,
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn go_offline(&mut self) {
        self.is_offline = true
    }
}

impl<TEventPayload> Default for InMemoryEventStore<TEventPayload>
where
    TEventPayload: BasePayload,
{
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl<TEventPayload> EventStore<TEventPayload> for InMemoryEventStore<TEventPayload>
where
    TEventPayload: BasePayload + 'static,
{
    async fn append(
        &self,
        stream_key: StreamKey,
        event: DomainEvent<TEventPayload>,
    ) -> Result<(), EventStoreError> {
        if self.is_offline {
            return Err(EventStoreError::AppendFailed);
        }
        let stored_event = StoredEvent::new(stream_key.clone(), 1, event);
        let mut data = self.data.lock().await;
        data.entry(stream_key)
            .or_insert_with(Vec::new)
            .push(stored_event);
        Ok(())
    }

    async fn load(
        &self,
        stream_key: StreamKey,
    ) -> Result<Vec<DomainEvent<TEventPayload>>, EventStoreError> {
        if self.is_offline {
            return Err(EventStoreError::LoadFailed);
        }

        let data = self.data.lock().await;
        let result = data.get(&stream_key).cloned().unwrap_or_default();
        Ok(result
            .into_iter()
            .map(|stored_event| stored_event.event)
            .collect())
    }
}
#[cfg(test)]
mod in_memory_event_store_tests {
    use super::*;
    use crate::domain::domain_event::DomainEvent;
    use futures::future::join_all;
    use rstest::{fixture, rstest};
    use serde::Deserialize;
    use serde::Serialize;
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
    async fn should_store_the_data(user_liked_event: DomainEvent<UserEventPayload>) {
        let event_store = InMemoryEventStore::default();
        let stream_key = StreamKey::new("users", user_liked_event.aggregate_id.clone());
        let result = event_store.append(stream_key, user_liked_event).await;
        assert!(result.is_ok());
    }

    #[rstest]
    #[tokio::test]
    async fn should_fail_storing_if_event_store_is_offline(
        user_liked_event: DomainEvent<UserEventPayload>,
    ) {
        let mut event_store = InMemoryEventStore::new();
        event_store.go_offline();
        let stream_key = StreamKey::new("users", user_liked_event.aggregate_id.clone());
        let result = event_store.append(stream_key, user_liked_event).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), EventStoreError::AppendFailed);
    }

    #[rstest]
    #[tokio::test]
    async fn should_query_the_data(user_liked_event: DomainEvent<UserEventPayload>) {
        let event_store = InMemoryEventStore::new();
        let stream_key = StreamKey::new("users", user_liked_event.aggregate_id.clone());
        event_store
            .append(stream_key.clone(), user_liked_event)
            .await
            .expect("unexpected store failed");

        let result = event_store.load(stream_key).await;
        assert!(result.is_ok());
    }

    #[rstest]
    #[tokio::test]
    async fn should_query_all_the_data_in_the_stream(
        user_liked_event: DomainEvent<UserEventPayload>,
    ) {
        let event_store = InMemoryEventStore::new();
        let stream_key = StreamKey::new("users", user_liked_event.aggregate_id.clone());
        let iterations = 100;
        join_all(
            (0..iterations)
                .map(|_| event_store.append(stream_key.clone(), user_liked_event.clone()))
                .collect::<Vec<_>>(),
        )
        .await;
        let result = event_store.load(stream_key).await;
        assert!(result.is_ok());
        let events = result.expect("Failed to load events");
        assert_eq!(events.len(), iterations);
    }

    #[rstest]
    #[tokio::test]
    async fn should_fail_querying_if_database_is_offline() {
        let mut event_store: InMemoryEventStore<UserEventPayload> = InMemoryEventStore::new();
        event_store.go_offline();
        let stream_key = StreamKey::new("users", Uuid::now_v7().to_string());
        let result = event_store.load(stream_key).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), EventStoreError::LoadFailed);
    }
}
