#![allow(dead_code)]
use uuid::Uuid;
use arts_and_crafts_rs::domain::domain_event::DomainEvent;
use crate::common::user::User;

pub fn generate_user_created_event() -> DomainEvent<User> {
    let aggregate_id = Uuid::now_v7().to_string();
    let payload = User::new(0.to_string(), "John Doe".to_string());
    DomainEvent::create("user_created", aggregate_id, payload)
}
