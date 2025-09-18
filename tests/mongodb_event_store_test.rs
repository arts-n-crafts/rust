#[cfg(test)]
use arts_and_crafts_rs::domain::domain_event::DomainEvent;
use arts_and_crafts_rs::infrastructure::event_store::stream_key::StreamKey;
use arts_and_crafts_rs::infrastructure::event_store::EventStore;
use futures::future::join_all;
use rstest::rstest;

mod common;
use common::mongodb_event_store::MongodbEventStore;
use common::user_event::{generate_user_created_event, UserEventPayload};

#[rstest]
#[tokio::test]
#[ignore]
async fn integration_mongodb_should_store_the_event() {
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
async fn integration_mongodb_should_load_the_events_of_the_stream() {
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
