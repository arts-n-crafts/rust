use chrono::Utc;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

pub trait EventPayload: Serialize + DeserializeOwned + Send + Sync + Clone {}
impl<T> EventPayload for T where T: Serialize + DeserializeOwned + Send + Sync + Clone {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum EventSource {
    Internal,
    External,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(bound(deserialize = "TPayload: DeserializeOwned"))]
pub struct DomainEvent<TPayload>
where
    TPayload: EventPayload,
{
    id: Uuid,
    r#type: String,
    pub aggregate_id: Uuid,
    source: EventSource,
    pub payload: TPayload,
    timestamp: i64,
    metadata: HashMap<String, String>,
}

impl<TPayload> DomainEvent<TPayload>
where
    TPayload: EventPayload,
{
    pub fn create(name: &str, aggregate_id: Uuid, payload: TPayload) -> Self {
        DomainEvent {
            id: Uuid::now_v7(),
            r#type: name.to_string(),
            aggregate_id,
            source: EventSource::Internal,
            payload,
            timestamp: Utc::now().timestamp_millis(),
            metadata: HashMap::new(),
        }
    }

    pub fn add_to_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    pub fn set_correlation_id(&mut self, correlation_id: String) {
        self.metadata
            .insert("correlation_id".to_string(), correlation_id);
    }

    pub fn set_causation_id(&mut self, causation_id: String) {
        self.metadata
            .insert("causation_id".to_string(), causation_id);
    }
}

#[cfg(test)]
mod create_domain_event_tests {
    use super::*;
    use rstest::{fixture, rstest};
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    #[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
    struct User {
        id: u8,
        name: String,
    }

    #[fixture]
    fn fixture() -> (&'static str, Uuid, User) {
        let event_name = "user_created";
        let aggregate_id = Uuid::now_v7();
        let payload: User = User {
            id: 1,
            name: "John".to_string(),
        };
        (event_name, aggregate_id.clone(), payload.clone())
    }

    #[rstest]
    fn it_should_create_a_domain_event(fixture: (&'static str, Uuid, User)) {
        let (event_name, aggregate_id, payload) = fixture;
        let event = DomainEvent::create(event_name, aggregate_id.clone(), payload.clone());
        assert_eq!(event.r#type, event_name);
        assert_eq!(event.aggregate_id, aggregate_id);
        assert_eq!(event.source, EventSource::Internal);
        assert_eq!(event.payload, payload);
        assert_eq!(event.metadata, HashMap::new());
    }

    #[rstest]
    fn it_should_add_metadata_causation_id_and_correlation_id(fixture: (&'static str, Uuid, User)) {
        let (event_name, aggregate_id, payload) = fixture;
        let mut event = DomainEvent::create(event_name, aggregate_id.clone(), payload.clone());
        let causation_id = Uuid::now_v7();
        let correlation_id = Uuid::now_v7();
        event.set_causation_id(causation_id.clone().to_string());
        event.set_correlation_id(correlation_id.clone().to_string());
        event.add_to_metadata("version".to_string(), "1".to_string());

        assert_eq!(
            *event.metadata.get("correlation_id").unwrap(),
            correlation_id.to_string()
        );
        assert_eq!(
            *event.metadata.get("causation_id").unwrap(),
            causation_id.to_string()
        );
        assert_eq!(*event.metadata.get("version").unwrap(), "1".to_string());
    }
}
