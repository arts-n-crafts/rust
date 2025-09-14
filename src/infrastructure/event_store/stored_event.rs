use crate::infrastructure::event_store::stream_key::StreamKey;
use chrono::{Utc};
use serde::{Serialize};
use uuid::{Uuid};

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct StoredEvent<TEvent: Serialize + Send + Sync + Clone> {
    id: Uuid,
    stream_key: StreamKey,
    version: u8,
    event: TEvent,
    timestamp: i64,
}

impl<TEvent: Serialize + Send + Sync + Clone> StoredEvent<TEvent> {
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

#[cfg(test)]
mod stored_event_test {
    use rstest::rstest;
    use crate::domain::domain_event::DomainEvent;
    use super::*;

    #[derive(Serialize, PartialEq, Debug, Clone)]
    struct TestPayload {
        id: Uuid,
    }

    #[rstest]
    fn it_should_create_a_stored_event() {
        let aggregate_id = Uuid::now_v7();
        let stream_key = StreamKey::new("users", aggregate_id);
        let payload = TestPayload {
            id: Uuid::now_v7()
        };
        let event = DomainEvent::create("UserCreated", aggregate_id, payload);
        let stored_event = StoredEvent::new(stream_key.clone(), 1, event.clone());
        assert_eq!(stored_event.stream_key, stream_key);
        assert_eq!(stored_event.version, 1);
        assert_eq!(stored_event.event, event);
    }
}
