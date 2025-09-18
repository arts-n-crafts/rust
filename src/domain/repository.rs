use crate::core::base_payload::BasePayload;
use crate::domain::domain_event::DomainEvent;
use crate::domain::with_identifier::WithIdentifier;
use async_trait::async_trait;
use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub enum RepositoryError {
    AppendError,
    LoadError,
}

#[async_trait]
pub trait Repository<TState, TEventPayload>
where
    TState: Debug + PartialEq,
    TEventPayload: BasePayload,
{
    async fn store(
        &self,
        events: Vec<DomainEvent<TEventPayload>>,
    ) -> Result<WithIdentifier, RepositoryError>;

    async fn load(&self, aggregate_id: String) -> Result<TState, RepositoryError>;
}

#[cfg(test)]
pub mod repository_tests {
    use super::*;
    use async_trait::async_trait;
    use rstest::rstest;
    use serde::{Deserialize, Serialize};
    use strum_macros::AsRefStr;
    use uuid::Uuid;

    #[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
    pub struct User {
        pub id: String,
        pub name: String,
        pub likes: u8,
    }

    #[derive(Clone, Serialize, Deserialize, Debug, PartialEq, AsRefStr)]
    pub enum UserEventPayload {
        UserCreated { id: String, name: String },
        UserLiked,
    }

    struct UserRepository;

    #[async_trait]
    impl Repository<User, UserEventPayload> for UserRepository {
        async fn store(
            &self,
            events: Vec<DomainEvent<UserEventPayload>>,
        ) -> Result<WithIdentifier, RepositoryError> {
            Ok(WithIdentifier {
                id: events.first().cloned().unwrap().aggregate_id,
            })
        }

        async fn load(&self, aggregate_id: String) -> Result<User, RepositoryError> {
            Ok(User {
                id: aggregate_id,
                name: "John Doe".to_string(),
                likes: 0,
            })
        }
    }

    #[rstest]
    #[tokio::test]
    async fn it_should_store_and_return_the_id_of_the_user_subject() {
        let repository = UserRepository;
        let aggregate_id = Uuid::now_v7().to_string();
        let event = DomainEvent::create(
            aggregate_id.clone(),
            UserEventPayload::UserCreated {
                id: Uuid::now_v7().to_string(),
                name: "John Doe".to_string(),
            },
        );
        let result = repository.store(vec![event]).await.unwrap();
        assert_eq!(result, WithIdentifier { id: aggregate_id });
    }

    #[rstest]
    #[tokio::test]
    async fn it_should_load_the_user_with_given_id() {
        let repository = UserRepository;
        let aggregate_id = Uuid::now_v7().to_string();
        let result = repository.load(aggregate_id.clone()).await.unwrap();
        assert_eq!(
            result,
            User {
                id: aggregate_id,
                name: "John Doe".to_string(),
                likes: 0,
            }
        );
    }
}
