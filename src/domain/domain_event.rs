use crate::core::base_payload::BasePayload;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum EventSource {
    Internal,
    External,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct DomainEvent<TPayload>
where
    TPayload: BasePayload,
{
    pub event_id: String,
    pub event_type: String,
    pub event_source: EventSource,
    pub aggregate_id: String,
    pub payload: TPayload,
    pub timestamp: i64,
    pub metadata: HashMap<String, String>,
}

impl<TPayload> DomainEvent<TPayload>
where
    TPayload: BasePayload + AsRef<str>,
{
    pub fn create(aggregate_id: String, payload: TPayload) -> Self {
        DomainEvent {
            event_id: Uuid::now_v7().to_string(),
            event_source: EventSource::Internal,
            event_type: payload.as_ref().to_string(),
            aggregate_id: aggregate_id.to_string(),
            payload: payload.clone(),
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
    use strum_macros::AsRefStr;
    use uuid::Uuid;

    #[allow(dead_code)]
    #[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
    pub struct User {
        pub id: String,
        pub name: String,
        pub likes: u8,
    }

    #[derive(Clone, Serialize, Deserialize, Debug, PartialEq, AsRefStr)]
    pub enum UserEventPayload {
        UserCreated { id: String, name: String },
        UserLiked,
    }

    #[fixture]
    fn fixture() -> (String, UserEventPayload) {
        let aggregate_id = Uuid::now_v7();
        let payload = UserEventPayload::UserCreated {
            id: 1.to_string(),
            name: "John".to_string(),
        };
        (aggregate_id.to_string(), payload.clone())
    }

    #[rstest]
    fn it_should_create_a_domain_event(fixture: (String, UserEventPayload)) {
        let (aggregate_id, payload) = fixture;
        let event = DomainEvent::create(aggregate_id.to_string(), payload.clone());
        assert_eq!(event.payload.as_ref(), payload.as_ref());
        assert_eq!(event.aggregate_id, aggregate_id.to_string());
        assert_eq!(event.event_source, EventSource::Internal);
        assert_eq!(event.payload, payload);
        assert_eq!(event.metadata, HashMap::new());
    }

    #[rstest]
    fn it_should_add_metadata_causation_id_and_correlation_id(fixture: (String, UserEventPayload)) {
        let (aggregate_id, payload) = fixture;
        let mut event = DomainEvent::create(aggregate_id.to_string(), payload.clone());
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
