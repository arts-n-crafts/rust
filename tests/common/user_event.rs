#![allow(dead_code)]

use arts_and_crafts_rs::domain::domain_event::DomainEvent;
use serde::{Deserialize, Serialize};
use strum_macros::AsRefStr;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, AsRefStr)]
pub enum UserEventPayload {
    UserCreated { name: String },
    UserLiked,
}

pub fn generate_user_created_event() -> DomainEvent<UserEventPayload> {
    let aggregate_id = Uuid::now_v7().to_string();
    let payload = UserEventPayload::UserCreated {
        name: "John Doe".to_string(),
    };
    DomainEvent::create(aggregate_id, payload)
}
