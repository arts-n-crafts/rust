#[cfg(test)]
mod common;

use async_trait::async_trait;
use common::user::User;
use rstest::{fixture, rstest};
use arts_and_crafts_rs::infrastructure::database::database_error::DatabaseError;
use arts_and_crafts_rs::infrastructure::database::in_memory_database::InMemoryDatabase;
use arts_and_crafts_rs::infrastructure::database::database_query::DatabaseQuery;

struct GetUsersNamedJohn {
    db: InMemoryDatabase<User>,
}

impl GetUsersNamedJohn {
    pub fn new(db: InMemoryDatabase<User>) -> Self {
        GetUsersNamedJohn { db }
    }
}

#[async_trait]
impl DatabaseQuery<User> for GetUsersNamedJohn {
    async fn execute(&self) -> Result<Vec<User>, DatabaseError> {
        let users = self.db.query("users").await?;
        Ok(users
            .iter()
            .filter(|user| user.name == "John")
            .cloned()
            .collect()
        )
    }
}

#[fixture]
fn users() -> Vec<User> {
    vec![
        User {
            id: 1,
            name: "John".to_string(),
        },
        User {
            id: 2,
            name: "Jane".to_string(),
        },
        User {
            id: 3,
            name: "Joe".to_string(),
        },
        User {
            id: 4,
            name: "John".to_string(),
        },
    ]
}

#[rstest]
#[tokio::test]
async fn should_retrieve_an_empty_vec_when_there_are_no_users() {
    let db = InMemoryDatabase::new();
    let get_users_named_john = GetUsersNamedJohn::new(db);
    let result = get_users_named_john.execute().await;
    assert!(result.is_ok());
    assert!(result.unwrap().is_empty());
}

#[rstest]
#[tokio::test]
async fn should_retrieve_all_users_named_john(users: Vec<User>) {
    let db = InMemoryDatabase::new();
    futures::future::join_all(
        users
            .iter()
            .map(|user| async { db.store("users", user.to_owned()).await }),
    )
    .await;

    let get_users_named_john = GetUsersNamedJohn::new(db);
    let result = get_users_named_john.execute().await;
    assert!(result.is_ok());
    let unwrapped_users = result.unwrap();
    assert_eq!(unwrapped_users.len(), 2);
    assert_eq!(unwrapped_users[0].id, 1);
    assert_eq!(unwrapped_users[1].id, 4);
}

#[rstest]
#[tokio::test]
async fn should_fail_if_the_database_is_offline() {
    let mut db = InMemoryDatabase::new();
    db.go_offline();
    let get_users_named_john = GetUsersNamedJohn::new(db);
    let result = get_users_named_john.execute().await;
    assert!(result.is_err());
}
