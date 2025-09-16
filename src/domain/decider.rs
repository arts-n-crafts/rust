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
    fn decide(current_state: TState, command: TCommand) -> DomainEvent<TEventPayload>;
}

#[cfg(test)]
mod decider_tests {
    use super::*;
    use rstest::rstest;
    use uuid::Uuid;
    use crate::domain::domain_event::HasEventType;

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

    impl HasEventType for UserEventPayload {
        fn event_type(&self) -> &'static str {
            match self {
                UserEventPayload::UserCreated { id: _, name: _ } => "user_created",
                UserEventPayload::UserLiked => "user_liked",
            }
        }
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
            }
        }
        fn decide(current_state: User, command: UserCommand) -> DomainEvent<UserEventPayload> {
            match command {
                UserCommand::CreateUser { id, name } => {
                    if current_state != UserDecider::initial_state() {
                        panic!("Expected current state to be initial state.");
                    }
                    DomainEvent::create(id.clone(), UserEventPayload::UserCreated { id, name })
                }
                UserCommand::LikeUser => {
                    if current_state == UserDecider::initial_state() {
                        panic!("Expected current state to be an evolved state, not initial state.");
                    }
                    DomainEvent::create(current_state.id, UserEventPayload::UserLiked)
                },
            }
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
            aggregate_id.to_string(),
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
            aggregate_id.to_string(),
            UserEventPayload::UserCreated {
                id: aggregate_id.to_string(),
                name: "John Doe".to_string(),
            },
        )];
        past_events.extend(
            (0..10).map(|_| {
                DomainEvent::create(aggregate_id.to_string(), UserEventPayload::UserLiked)
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

    #[rstest]
    fn it_should_decide_to_emit_an_user_created_event_based_on_create_user_command() {
        let aggregate_id = Uuid::now_v7();
        let past_events = vec![];
        let current_state = past_events
            .into_iter()
            .fold(UserDecider::initial_state(), |state, event| {
                UserDecider::evolve(state, event)
            });
        let event = UserDecider::decide(
            current_state.clone(),
            UserCommand::CreateUser {
                id: aggregate_id.to_string(),
                name: "John Doe".to_string(),
            },
        );
        let next_state = vec![event.clone()]
            .into_iter()
            .fold(current_state, |state, event| {
                UserDecider::evolve(state, event)
            });
        assert_eq!(event.aggregate_id.to_string(), aggregate_id.to_string());
        assert_eq!(
            event.payload,
            UserEventPayload::UserCreated {
                id: aggregate_id.to_string(),
                name: "John Doe".to_string()
            }
        );
        assert_eq!(next_state.id, aggregate_id.to_string());
        assert_eq!(next_state.name, String::from("John Doe"));
        assert_eq!(next_state.likes, 0);
    }

    #[rstest]
    fn it_should_decide_to_emit_an_user_liked_event_based_on_like_user_command() {
        let aggregate_id = Uuid::now_v7();
        let past_events = vec![
            DomainEvent::create(
                aggregate_id.to_string(),
                UserEventPayload::UserCreated {
                    id: aggregate_id.to_string(),
                    name: "John Doe".to_string(),
                },
            )
        ];
        let current_state = past_events
            .into_iter()
            .fold(UserDecider::initial_state(), |state, event| {
                UserDecider::evolve(state, event)
            });
        let event = UserDecider::decide(
            current_state.clone(),
            UserCommand::LikeUser,
        );
        let next_state = vec![event.clone()]
            .into_iter()
            .fold(current_state, |state, event| {
                UserDecider::evolve(state, event)
            });
        assert_eq!(event.aggregate_id.to_string(), aggregate_id.to_string());
        assert_eq!(event.payload, UserEventPayload::UserLiked);
        assert_eq!(next_state.id, aggregate_id.to_string());
        assert_eq!(next_state.name, String::from("John Doe"));
        assert_eq!(next_state.likes, 1);
    }
}
