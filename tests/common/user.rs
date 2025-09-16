#![allow(dead_code)]
use serde::{Deserialize, Serialize};

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
