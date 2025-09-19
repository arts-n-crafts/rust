#[cfg(test)]
mod common;
use crate::common::create_user_command_handler::CreateUserCommandHandler;
use crate::common::mongodb_event_store::MongodbEventStore;
use crate::common::user_command::UserCommandPayload;
use crate::common::user_event::UserEventPayload;
use crate::common::user_repository::UserRepository;
use arts_n_crafts::core::command::Command;
use arts_n_crafts::core::command_handler::CommandHandler;
use arts_n_crafts::domain::domain_event::DomainEvent;
use arts_n_crafts::domain::with_identifier::WithIdentifier;
use arts_n_crafts::infrastructure::event_store::EventStore;
use arts_n_crafts::infrastructure::event_store::stream_key::StreamKey;
use rstest::rstest;
use uuid::Uuid;

#[rstest]
#[tokio::test]
#[ignore]
async fn integration_mongodb_should_create_a_new_user() {
    let connection_string = MongodbEventStore::make_connection_string_from_env();
    let event_store = MongodbEventStore::new(connection_string).await;
    let repository = UserRepository::new(event_store.clone());
    let handler = CreateUserCommandHandler::new(repository);

    let aggregate_id = Uuid::now_v7().to_string();
    let payload = UserCommandPayload::CreateUser {
        name: "Ruddy Rut".to_string(),
    };
    let command = Command::create(aggregate_id.clone(), payload.clone());
    let command_result = handler.execute(command).await;
    assert!(command_result.is_ok());
    assert_eq!(
        command_result.unwrap(),
        WithIdentifier {
            id: aggregate_id.clone()
        }
    );

    let stream_key = StreamKey::new("users", aggregate_id.clone());
    let load_result: Result<Vec<DomainEvent<UserEventPayload>>, _> =
        event_store.load(stream_key).await;
    assert!(load_result.is_ok());
    let event = load_result.unwrap();
    let last_event = event.last().unwrap();
    assert_eq!(last_event.clone().aggregate_id, aggregate_id);
    assert_eq!(
        last_event.payload,
        UserEventPayload::UserCreated {
            name: "Ruddy Rut".to_string(),
        }
    );
}
