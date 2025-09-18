use serde::{Deserialize, Serialize};
use strum_macros::AsRefStr;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, AsRefStr)]
pub enum UserCommandPayload {
    CreateUser { name: String },
    LikeUser,
}
