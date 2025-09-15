use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct User {
    pub id: u8,
    pub name: String,
    pub likes: u8,
}

impl User {
    pub fn new(id: u8, name: String) -> Self {
        Self { id, name, likes: 0 }
    }
}
