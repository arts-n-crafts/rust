use crate::core::base_payload::BasePayload;
use crate::domain::domain_event::DomainEvent;
use crate::infrastructure::event_store::stream_key::StreamKey;
use chrono::Utc;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct StoredEvent<TEventPayload>
where
    TEventPayload: BasePayload,
{
    pub id: Uuid,
    stream_key: StreamKey,
    version: u8,
    pub event: DomainEvent<TEventPayload>,
    timestamp: i64,
}

impl<TEventPayload> StoredEvent<TEventPayload>
where
    TEventPayload: BasePayload,
{
    pub fn new(stream_key: StreamKey, version: u8, event: DomainEvent<TEventPayload>) -> Self {
        StoredEvent {
            id: Uuid::now_v7(),
            stream_key,
            version,
            event,
            timestamp: Utc::now().timestamp_millis(),
        }
    }
}

#[cfg(test)]
mod stored_event_test {
    use super::*;
    use rstest::rstest;
    use serde::Deserialize;
    use strum_macros::AsRefStr;

    #[derive(Clone, Serialize, Deserialize, Debug, PartialEq, AsRefStr)]
    pub enum UserEventPayload {
        UserCreated { id: String, name: String },
        UserLiked,
    }

    #[rstest]
    fn it_should_create_a_stored_event() {
        let aggregate_id = Uuid::now_v7();
        let stream_key = StreamKey::new("users", aggregate_id.to_string());
        let payload = UserEventPayload::UserLiked;
        let event = DomainEvent::create(aggregate_id.to_string(), payload);
        let stored_event = StoredEvent::new(stream_key.clone(), 1, event.clone());
        assert_eq!(stored_event.stream_key, stream_key);
        assert_eq!(stored_event.version, 1);
        assert_eq!(stored_event.event, event);
    }
}
