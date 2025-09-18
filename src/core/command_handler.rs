use crate::core::base_payload::BasePayload;
use crate::core::command::Command;
use async_trait::async_trait;
use serde::Serialize;

#[async_trait]
pub trait CommandHandler<TPayload, TResult, TError>
where
    TPayload: BasePayload + AsRef<str>,
    TResult: Serialize + Send + Sync + Clone,
{
    async fn execute(&self, a_command: Command<TPayload>) -> Result<TResult, TError>;
}

#[cfg(test)]
mod command_handler_tests {
    use super::*;
    use crate::domain::with_identifier::WithIdentifier;
    use rstest::rstest;
    use serde::{Deserialize, Serialize};
    use strum_macros::AsRefStr;
    use uuid::Uuid;

    #[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
    struct User {
        pub id: String,
        pub name: String,
        pub likes: u8,
    }

    #[derive(Clone, Serialize, Deserialize, Debug, PartialEq, AsRefStr)]
    enum UserCommandPayload {
        CreateUser { name: String },
        LikeUser,
    }

    struct CreateUserCommandHandler;

    #[async_trait]
    impl CommandHandler<UserCommandPayload, WithIdentifier, ()> for CreateUserCommandHandler {
        async fn execute(
            &self,
            a_command: Command<UserCommandPayload>,
        ) -> Result<WithIdentifier, ()> {
            Ok(WithIdentifier {
                id: a_command.aggregate_id,
            })
        }
    }

    #[rstest]
    #[tokio::test]
    async fn it_should_return_the_id_of_the_created_user() {
        let an_aggregate_id = Uuid::now_v7().to_string();
        let a_payload = UserCommandPayload::CreateUser {
            name: "John Doe".to_string(),
        };
        let a_command = Command::create(an_aggregate_id.clone(), a_payload);
        let a_command_handler = CreateUserCommandHandler {};
        let a_result = a_command_handler
            .execute(a_command)
            .await
            .expect("failed to execute command");
        assert_eq!(a_result.id, an_aggregate_id);
    }
}
