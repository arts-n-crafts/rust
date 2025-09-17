#[cfg(test)]
mod common;
use arts_and_crafts_rs::infrastructure::database::database_query::DatabaseQuery;
use arts_and_crafts_rs::infrastructure::database::in_memory_database::{
    DatabaseError, InMemoryDatabase,
};
use async_trait::async_trait;
use common::user::User;
use rstest::{fixture, rstest};

struct GetUsersNamedJohn {
    db: InMemoryDatabase<User>,
}

impl GetUsersNamedJohn {
    pub fn new(db: InMemoryDatabase<User>) -> Self {
        GetUsersNamedJohn { db }
    }
}

#[async_trait]
impl DatabaseQuery<User, DatabaseError> for GetUsersNamedJohn {
    async fn execute(&self) -> Result<Vec<User>, DatabaseError> {
        let users = self.db.query("users").await?;
        Ok(users
            .iter()
            .filter(|user| user.name == "John")
            .cloned()
            .collect())
    }
}

#[fixture]
fn users() -> Vec<User> {
    vec![
        User::new(0.to_string(), "John".to_string()),
        User::new(1.to_string(), "Jane".to_string()),
        User::new(2.to_string(), "Joe".to_string()),
        User::new(3.to_string(), "John".to_string()),
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
    assert_eq!(unwrapped_users[0].id, 0.to_string());
    assert_eq!(unwrapped_users[1].id, 3.to_string());
}

#[rstest]
#[tokio::test]
async fn should_fail_if_the_database_is_offline() {
    let mut db = InMemoryDatabase::new();
    db.go_offline();
    let get_users_named_john = GetUsersNamedJohn::new(db);
    let result = get_users_named_john.execute().await;
    assert!(result.is_err());
    assert_eq!(result.err(), Some(DatabaseError::Unreachable));
}
