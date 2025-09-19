use crate::common::user::User;
use crate::common::user_decider::UserDecider;
use crate::common::user_event::UserEventPayload;
use arts_n_crafts::domain::decider::Decider;
use arts_n_crafts::domain::domain_event::DomainEvent;
use arts_n_crafts::domain::repository::Repository;
use arts_n_crafts::domain::with_identifier::WithIdentifier;
use arts_n_crafts::infrastructure::event_store::stream_key::StreamKey;
use arts_n_crafts::infrastructure::event_store::{EventStore, EventStoreError};
use async_trait::async_trait;
use thiserror::Error;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("appending failed")]
    AppendFailed,

    #[error("loading failed")]
    LoadFailed,

    #[error("event store error: {0}")]
    EventStore(#[from] EventStoreError),
}

#[allow(dead_code)]
pub struct UserRepository<TStore>
where
    TStore: EventStore<UserEventPayload> + Send + Sync,
{
    event_store: TStore,
    stream_name: &'static str,
}

impl<TStore> UserRepository<TStore>
where
    TStore: EventStore<UserEventPayload> + Send + Sync,
{
    #[allow(dead_code)]
    pub fn new(event_store: TStore) -> Self {
        Self {
            event_store,
            stream_name: "users",
        }
    }
}

#[async_trait]
impl<TStore> Repository<User, UserEventPayload, RepositoryError> for UserRepository<TStore>
where
    TStore: EventStore<UserEventPayload> + Send + Sync,
{
    async fn store(
        &self,
        events: Vec<DomainEvent<UserEventPayload>>,
    ) -> Result<WithIdentifier, RepositoryError> {
        let id = events
            .first()
            .cloned()
            .ok_or(RepositoryError::AppendFailed)?
            .aggregate_id;
        let stream_key = StreamKey::new(self.stream_name, id.clone());
        for event in events {
            self.event_store.append(stream_key.clone(), event).await?;
        }
        Ok(WithIdentifier { id })
    }

    async fn load(&self, aggregate_id: String) -> Result<User, RepositoryError> {
        let stream_key = StreamKey::new(self.stream_name, aggregate_id);
        let past_events = self.event_store.load(stream_key).await?;
        let state = past_events
            .into_iter()
            .fold(UserDecider::initial_state(), |state, event| {
                UserDecider::evolve(state, event)
            });
        Ok(state)
    }
}
