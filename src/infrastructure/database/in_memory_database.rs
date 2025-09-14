use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::infrastructure::database::database::{Database};
use crate::infrastructure::database::database_error::DatabaseError;

pub struct InMemoryDatabase<T: Serialize + Send + Sync + Clone> {
    data: Arc<Mutex<HashMap<String, Vec<T>>>>,
    is_offline: bool,
}

impl<T: Serialize + Send + Sync + Clone> InMemoryDatabase<T> {
    pub fn new() -> Self {
        InMemoryDatabase {
            data: Arc::new(Mutex::new(HashMap::new())),
            is_offline: false,
        }
    }

    pub fn go_offline(&mut self) {
        self.is_offline = true
    }
}

impl<T: Serialize + Send + Sync + Clone> Database<T> for InMemoryDatabase<T> {
    async fn store(&self, key: &str, value: T) -> Result<(), DatabaseError> {
        if self.is_offline {
            return Err(DatabaseError::DatabaseStoreError(
                Box::from("Database unreachable")
            ));
        }
        let mut data = self.data.lock().await;
        data.entry(key.to_owned())
            .or_insert_with(Vec::new)
            .push(value.to_owned());

        Ok(())
    }

    async fn query(&self, table_name: &str) -> Result<Vec<T>, DatabaseError> {
        if self.is_offline {
            return Err(DatabaseError::DatabaseStoreError(
                Box::from("Database unreachable")
            ));
        }

        let data = self.data.lock().await;
        let result = data
            .get(table_name)
            .map(|v| v.clone())
            .unwrap_or_default();
        Ok(result)
    }
}

#[cfg(test)]
mod in_memory_database {
    use super::*;
    use rstest::{fixture, rstest};
    const TABLE_NAME: &str = "users";
    #[derive(Serialize, Clone)]
    struct User {
        name: String,
    }
    #[fixture]
    fn user() -> Vec<User> {
        vec![
            User {
                name: "John".to_string(),
            }
        ]
    }

    #[rstest]
    #[tokio::test]
    async fn should_store_the_data(user: Vec<User>) {
        let db = InMemoryDatabase::new();
        let result = db.store(TABLE_NAME, user[0].to_owned()).await;
        assert!(result.is_ok());
    }

    #[rstest]
    #[tokio::test]
    async fn should_fail_storing_if_database_is_offline(user: Vec<User>) {
        let mut db = InMemoryDatabase::new();
        db.go_offline();
        let result = db.store(TABLE_NAME, user[0].to_owned()).await;
        assert!(result.is_err());
    }

    #[rstest]
    #[tokio::test]
    async fn should_query_the_data(user: Vec<User>) {
        let db = InMemoryDatabase::new();
        db.store(TABLE_NAME, user[0].to_owned()).await.expect("unexpected store failed");
        let result = db.query(TABLE_NAME).await;
        assert!(result.is_ok());
    }

    #[rstest]
    #[tokio::test]
    async fn should_fail_querying_if_database_is_offline() {
        let mut db = InMemoryDatabase::<User>::new();
        db.go_offline();
        let result = db.query(TABLE_NAME).await;
        assert!(result.is_err());
    }
}
