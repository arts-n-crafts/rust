use crate::domain::domain_event::{DomainEvent, EventPayload};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub trait Decider<TState, TCommand, TEventPayload>
where
    TState: Serialize + DeserializeOwned + Send + Sync + Clone,
    TCommand: Serialize + DeserializeOwned + Send + Sync + Clone,
    TEventPayload: EventPayload,
{
    fn initial_state(id: String) -> TState;
    fn evolve(current_state: TState, event: DomainEvent<TEventPayload>) -> TState;
    fn decide(current_state: TState, command: TCommand) -> Vec<TEventPayload>;
}

#[cfg(test)]
mod decider_tests {
    use rstest::rstest;
    use uuid::Uuid;
    use super::*;

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
        CreateUser { id: u8, name: String },
        LikeUser,
    }

    #[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
    pub enum UserEventPayload {
        UserCreated { id: u8, name: String },
        UserLiked,
    }

    struct UserDecider;
    impl Decider<User, UserCommand, UserEventPayload> for UserDecider {
        fn initial_state(id: String) -> User {
            User::new(id, String::from(""))
        }

        fn evolve(current_state: User, event: DomainEvent<UserEventPayload>) -> User {
            todo!()
        }

        fn decide(current_state: User, command: UserCommand) -> Vec<UserEventPayload> {
            todo!()
        }
    }

    #[rstest]
    fn it_should_return_the_initial_state() {
        let aggregate_id = Uuid::now_v7();
        let state = UserDecider::initial_state(aggregate_id.to_string());
        assert_eq!(state.id, aggregate_id.to_string());
        assert_eq!(state.name, String::from(""));
        assert_eq!(state.likes, 0);
    }
}
