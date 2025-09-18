use arts_and_crafts_rs::core::base_payload::BasePayload;
use arts_and_crafts_rs::domain::domain_event::DomainEvent;
use arts_and_crafts_rs::infrastructure::event_store::stream_key::StreamKey;
use arts_and_crafts_rs::infrastructure::event_store::{EventStore, EventStoreError};
use async_trait::async_trait;
use chrono::Utc;
use dotenvy::dotenv;
use futures::TryStreamExt;
use mongodb::bson::{doc, from_document, to_document, Document};
use mongodb::options::FindOptions;
use mongodb::{options::ClientOptions, Client};
use mongodb::{Collection, Database};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MongoStoredEvent<TEventPayload>
where
    TEventPayload: BasePayload + Send + Sync + 'static,
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

#[allow(dead_code)]
#[derive(Clone)]
pub struct MongodbEventStore {
    collection: Collection<Document>,
}

impl MongodbEventStore {
    #[allow(dead_code)]
    pub async fn new(connection_string: String) -> Self {
        let client_options = ClientOptions::parse(connection_string)
            .await
            .expect("Failed to create ClientOptions.");
        let client = Client::with_options(client_options).expect("Failed to create Client.");
        let db: Database = client.database("test");
        let collection: Collection<Document> = db.collection("event_store");
        MongodbEventStore { collection }
    }

    #[allow(dead_code)]
    pub fn make_connection_string_from_env() -> String {
        dotenv().ok();
        let username = std::env::var("MONGO_INITDB_ROOT_USERNAME")
            .expect("MONGO_INITDB_ROOT_USERNAME not set");
        let password = std::env::var("MONGO_INITDB_ROOT_PASSWORD")
            .expect("MONGO_INITDB_ROOT_PASSWORD not set");
        format!("mongodb://{}:{}@localhost:27017", username, password)
    }
}

#[async_trait]
impl<TEventPayload> EventStore<TEventPayload> for MongodbEventStore
where
    TEventPayload: BasePayload + AsRef<str> + DeserializeOwned + Serialize + Send + Sync + 'static,
{
    async fn append(
        &self,
        stream_key: StreamKey,
        event: DomainEvent<TEventPayload>,
    ) -> Result<(), EventStoreError> {
        let stored_event = MongoStoredEvent::new(stream_key, 1, event);
        let doc = to_document(&stored_event).map_err(|_| EventStoreError::AppendFailed)?;

        self.collection
            .insert_one(doc)
            .await
            .map_err(|_| EventStoreError::AppendFailed)?;

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
            .map_err(|_| EventStoreError::LoadFailed)?;

        let mut events = Vec::new();
        while let Some(doc) = cursor
            .try_next()
            .await
            .map_err(|_| EventStoreError::LoadFailed)?
        {
            let stored_event: MongoStoredEvent<TEventPayload> =
                from_document(doc).map_err(|_| EventStoreError::LoadFailed)?;
            events.push(stored_event.event);
        }

        Ok(events)
    }
}
