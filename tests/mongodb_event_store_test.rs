#[cfg(test)]
use arts_and_crafts_rs::domain::domain_event::DomainEvent;
use arts_and_crafts_rs::infrastructure::event_store::event_store::{EventStore, EventStoreError};
use arts_and_crafts_rs::infrastructure::event_store::stream_key::StreamKey;
use chrono::Utc;
use dotenvy::dotenv;
use futures::future::join_all;
use futures::TryStreamExt;
use mongodb::bson::{doc, from_document, to_document, Document};
use mongodb::options::FindOptions;
use mongodb::{options::ClientOptions, Client};
use mongodb::{Collection, Database};
use rstest::rstest;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use arts_and_crafts_rs::core::base_payload::BasePayload;

mod common;
use common::user_created_event::{generate_user_created_event, UserEventPayload};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MongoStoredEvent<TEventPayload>
where
    TEventPayload: BasePayload,
{
    pub _id: String,
    stream_key: StreamKey,
    version: u8,
    pub event: DomainEvent<TEventPayload>,
    timestamp: i64,
}

impl<TEventPayloadPayload> MongoStoredEvent<TEventPayloadPayload>
where
    TEventPayloadPayload: BasePayload,
{
    pub fn new(
        stream_key: StreamKey,
        version: u8,
        event: DomainEvent<TEventPayloadPayload>,
    ) -> Self {
        MongoStoredEvent {
            _id: Uuid::now_v7().to_string(),
            stream_key,
            version,
            event,
            timestamp: Utc::now().timestamp_millis(),
        }
    }
}

pub struct MongodbEventStore {
    collection: Collection<Document>,
}

impl MongodbEventStore {
    pub async fn new(connection_string: String) -> Self {
        let client_options = ClientOptions::parse(connection_string)
            .await
            .expect("Failed to create ClientOptions.");
        let client = Client::with_options(client_options).expect("Failed to create Client.");
        let db: Database = client.database("test");
        let collection: Collection<Document> = db.collection("event_store");
        MongodbEventStore { collection }
    }

    pub fn make_connection_string_from_env() -> String {
        dotenv().ok();
        let username = std::env::var("MONGO_INITDB_ROOT_USERNAME")
            .expect("MONGO_INITDB_ROOT_USERNAME not set");
        let password = std::env::var("MONGO_INITDB_ROOT_PASSWORD")
            .expect("MONGO_INITDB_ROOT_PASSWORD not set");
        format!("mongodb://{}:{}@localhost:27017", username, password)
    }
}

impl<TEventPayload> EventStore<TEventPayload> for MongodbEventStore
where
    TEventPayload: BasePayload + AsRef<str> + DeserializeOwned + Serialize,
{
    async fn append(
        &self,
        stream_key: StreamKey,
        event: DomainEvent<TEventPayload>,
    ) -> Result<(), EventStoreError> {
        let stored_event = MongoStoredEvent::new(stream_key, 1, event);
        let doc = to_document(&stored_event).map_err(|_| EventStoreError::AppendError)?;

        self.collection
            .insert_one(doc)
            .await
            // .map_err(|e| EventStoreError::AppendError(e.to_string()))?;
            .map_err(|_| EventStoreError::AppendError)?;

        Ok(())
    }

    async fn load(
        &self,
        stream_key: StreamKey,
    ) -> Result<Vec<DomainEvent<TEventPayload>>, EventStoreError> {
        let filter = doc! { "stream_key": &stream_key.as_str() };
        let find_options = FindOptions::builder().sort(doc! { "timestamp": 1 }).build();

        let mut cursor = self
            .collection
            .find(filter)
            .with_options(find_options)
            .await
            .map_err(|_| EventStoreError::LoadError)?;

        let mut events = Vec::new();
        while let Some(doc) = cursor
            .try_next()
            .await
            .map_err(|_| EventStoreError::LoadError)?
        {
            let stored_event: MongoStoredEvent<TEventPayload> =
                from_document(doc).map_err(|_| EventStoreError::LoadError)?;
            events.push(stored_event.event);
        }

        Ok(events)
    }
}

#[rstest]
#[tokio::test]
#[ignore]
async fn mongodb_should_store_the_event() {
    let connection_string = MongodbEventStore::make_connection_string_from_env();
    let event_store = MongodbEventStore::new(connection_string).await;
    let user_created_event = generate_user_created_event();
    let stream_key = StreamKey::new("users", user_created_event.aggregate_id.clone());
    let result = event_store.append(stream_key, user_created_event).await;
    assert!(result.is_ok());
}

#[rstest]
#[tokio::test]
#[ignore]
async fn mongodb_should_load_the_events_of_the_stream() {
    let user_created_event = generate_user_created_event();
    let user_updated_event = DomainEvent::create(
        user_created_event.aggregate_id.clone(),
        user_created_event.payload.clone(),
    );
    let connection_string = MongodbEventStore::make_connection_string_from_env();
    let event_store = MongodbEventStore::new(connection_string).await;
    let stream_key = StreamKey::new("users", user_created_event.aggregate_id.clone());
    event_store
        .append(stream_key.clone(), user_created_event)
        .await
        .expect("Failed to append event.");

    let iterations = 100;
    join_all(
        (0..iterations)
            .map(|_| event_store.append(stream_key.clone(), user_updated_event.clone()))
            .collect::<Vec<_>>(),
    )
    .await;

    let result: Result<Vec<DomainEvent<UserEventPayload>>, _> = event_store.load(stream_key).await;
    assert!(result.is_ok());
    let events = result.expect("Failed to load events");
    assert_eq!(events.len(), iterations + 1);
}
