use crate::domain::domain_event::{DomainEvent, EventPayload};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub trait Decider<TState, TCommand, TEventPayload>
where
    TState: Serialize + DeserializeOwned + Send + Sync + Clone,
    TCommand: Serialize + DeserializeOwned + Send + Sync + Clone,
    TEventPayload: EventPayload,
{
    fn initial_state() -> TState;
    fn evolve(current_state: TState, event: DomainEvent<TEventPayload>) -> TState;
    fn decide(current_state: TState, command: TCommand) -> Vec<TEventPayload>;
}

#[cfg(test)]
mod decider_tests {
    use super::*;
    use rstest::rstest;
    use uuid::Uuid;

    #[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
    pub struct User {
        pub id: String,
        pub name: String,
        pub likes: u8,
    }

    impl User {
        pub fn new(id: String, name: String) -> Self {
            Self { id, name, likes: 0 }
        }
    }

    #[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
    pub enum UserCommand {
        CreateUser { id: String, name: String },
        LikeUser,
    }

    #[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
    pub enum UserEventPayload {
        UserCreated { id: String, name: String },
        UserLiked,
    }

    struct UserDecider;

    impl Decider<User, UserCommand, UserEventPayload> for UserDecider {
        fn initial_state() -> User {
            User::new(String::from(""), String::from(""))
        }
        fn evolve(mut current_state: User, event: DomainEvent<UserEventPayload>) -> User {
            match event.payload {
                UserEventPayload::UserCreated { id, name } => {
                    current_state.id = id;
                    current_state.name = name;
                    current_state
                }
                UserEventPayload::UserLiked => {
                    current_state.likes += 1;
                    current_state
                }
                _ => {
                    current_state
                }
            }
        }
        fn decide(_current_state: User, _command: UserCommand) -> Vec<UserEventPayload> {
            todo!()
        }
    }

    #[rstest]
    fn it_should_return_the_initial_state() {
        let state = UserDecider::initial_state();
        assert_eq!(state.id, String::from(""));
        assert_eq!(state.name, String::from(""));
        assert_eq!(state.likes, 0);
    }

    #[rstest]
    fn it_should_evolve_to_the_current_created_state() {
        let aggregate_id = Uuid::now_v7();
        let past_events = vec![DomainEvent::create(
            "user_created",
            aggregate_id,
            UserEventPayload::UserCreated {
                id: aggregate_id.to_string(),
                name: "John Doe".to_string(),
            },
        )];
        let state = past_events
            .into_iter()
            .fold(UserDecider::initial_state(), |state, event| {
                UserDecider::evolve(state, event)
            });
        assert_eq!(state.id, aggregate_id.to_string());
        assert_eq!(state.name, String::from("John Doe"));
        assert_eq!(state.likes, 0);
    }

    #[rstest]
    fn it_should_evolve_to_the_current_state_with_likes() {
        let aggregate_id = Uuid::now_v7();
        let mut past_events = vec![DomainEvent::create(
            "user_created",
            aggregate_id,
            UserEventPayload::UserCreated {
                id: aggregate_id.to_string(),
                name: "John Doe".to_string(),
            },
        )];
        past_events.extend(
            (0..10).map(|_| {
                DomainEvent::create("user_liked", aggregate_id, UserEventPayload::UserLiked)
            }),
        );
        let state = past_events
            .into_iter()
            .fold(UserDecider::initial_state(), |state, event| {
                UserDecider::evolve(state, event)
            });
        assert_eq!(state.id, aggregate_id.to_string());
        assert_eq!(state.name, String::from("John Doe"));
        assert_eq!(state.likes, 10);
    }
}
