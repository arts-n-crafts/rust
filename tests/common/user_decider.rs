use crate::common::user::User;
use crate::common::user_command::UserCommandPayload;
use crate::common::user_event::UserEventPayload;
use arts_n_crafts::core::command::Command;
use arts_n_crafts::domain::decider::Decider;
use arts_n_crafts::domain::domain_event::DomainEvent;

#[allow(dead_code)]
pub struct UserDecider;

impl Decider<User, UserCommandPayload, UserEventPayload> for UserDecider {
    fn initial_state() -> User {
        User::new(String::from(""), String::from(""))
    }
    fn evolve(mut current_state: User, event: DomainEvent<UserEventPayload>) -> User {
        match event.payload {
            UserEventPayload::UserCreated { name } => {
                current_state.name = name;
                current_state
            }
            UserEventPayload::UserLiked => {
                current_state.likes += 1;
                current_state
            }
        }
    }
    fn decide(
        _current_state: User,
        command: Command<UserCommandPayload>,
    ) -> DomainEvent<UserEventPayload> {
        match command.payload {
            UserCommandPayload::CreateUser { name } => DomainEvent::create(
                command.aggregate_id.clone(),
                UserEventPayload::UserCreated { name },
            ),
            UserCommandPayload::LikeUser => {
                DomainEvent::create(command.aggregate_id.clone(), UserEventPayload::UserLiked)
            }
        }
    }
}
