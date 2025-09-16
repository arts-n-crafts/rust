#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use arts_and_crafts_rs::domain::domain_event::{DomainEvent, HasEventType};


#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum UserEventPayload {
    UserCreated { id: String, name: String },
    UserLiked,
}

impl HasEventType for UserEventPayload {
    fn event_type(&self) -> &'static str {
        match self {
            UserEventPayload::UserCreated { id: _, name: _ } => "user_created",
            UserEventPayload::UserLiked => "user_liked",
        }
    }
}

pub fn generate_user_created_event() -> DomainEvent<UserEventPayload> {
    let aggregate_id = Uuid::now_v7().to_string();
    let payload = UserEventPayload::UserCreated { id: aggregate_id.clone(), name: "John Doe".to_string() };
    DomainEvent::create(aggregate_id, payload)
}
