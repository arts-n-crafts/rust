use crate::common::user_command::UserCommandPayload;
use crate::common::user_decider::UserDecider;
use crate::common::user_event::UserEventPayload;
use crate::common::user_repository::{RepositoryError, UserRepository};
use arts_n_crafts::core::command::Command;
use arts_n_crafts::core::command_handler::CommandHandler;
use arts_n_crafts::domain::decider::Decider;
use arts_n_crafts::domain::repository::Repository;
use arts_n_crafts::domain::with_identifier::WithIdentifier;
use arts_n_crafts::infrastructure::event_store::{EventStore, EventStoreError};
use async_trait::async_trait;
use thiserror::Error;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum CreateUserCommandHandlerError {
    #[error("execute failed")]
    ExecuteFailed,

    #[error("repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("event store error: {0}")]
    EventStore(#[from] EventStoreError),
}

#[allow(dead_code)]
pub struct CreateUserCommandHandler<TStore>
where
    TStore: EventStore<UserEventPayload> + Send + Sync,
{
    repository: UserRepository<TStore>,
}

impl<TStore> CreateUserCommandHandler<TStore>
where
    TStore: EventStore<UserEventPayload> + Send + Sync,
{
    #[allow(dead_code)]
    pub fn new(repository: UserRepository<TStore>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<TStore> CommandHandler<UserCommandPayload, WithIdentifier, CreateUserCommandHandlerError>
    for CreateUserCommandHandler<TStore>
where
    TStore: EventStore<UserEventPayload> + Send + Sync,
{
    async fn execute(
        &self,
        a_command: Command<UserCommandPayload>,
    ) -> Result<WithIdentifier, CreateUserCommandHandlerError> {
        let current_state = self.repository.load(a_command.aggregate_id.clone()).await?;
        let decision = UserDecider::decide(current_state, a_command.clone());
        self.repository.store(vec![decision]).await?;
        Ok(WithIdentifier {
            id: a_command.aggregate_id,
        })
    }
}
