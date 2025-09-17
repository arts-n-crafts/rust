use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, PartialEq)]
pub enum DatabaseError {
    Unreachable,
}

pub struct InMemoryDatabase<T>
where
    T: Serialize + Send + Sync + Clone,
{
    data: Arc<Mutex<HashMap<String, Vec<T>>>>,
    is_offline: bool,
}

impl<T> InMemoryDatabase<T>
where
    T: Serialize + Send + Sync + Clone,
{
    pub fn new() -> Self {
        InMemoryDatabase {
            data: Arc::new(Mutex::new(HashMap::new())),
            is_offline: false,
        }
    }

    pub fn go_offline(&mut self) {
        self.is_offline = true
    }

    pub async fn store(&self, key: &str, value: T) -> Result<(), DatabaseError> {
        if self.is_offline {
            return Err(DatabaseError::Unreachable);
        }
        let mut data = self.data.lock().await;
        data.entry(key.to_owned())
            .or_insert_with(Vec::new)
            .push(value.to_owned());

        Ok(())
    }

    pub async fn query(&self, table_name: &str) -> Result<Vec<T>, DatabaseError> {
        if self.is_offline {
            return Err(DatabaseError::Unreachable);
        }

        let data = self.data.lock().await;
        let result = data.get(table_name).cloned().unwrap_or_default();
        Ok(result)
    }
}

impl<T> Default for InMemoryDatabase<T>
where
    T: Serialize + Send + Sync + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod in_memory_database_tests {
    use super::*;
    use rstest::{fixture, rstest};
    const TABLE_NAME: &str = "users";

    #[derive(Serialize, Clone, Debug, PartialEq)]
    struct User {
        name: String,
    }

    #[fixture]
    fn user() -> Vec<User> {
        vec![User {
            name: "John".to_string(),
        }]
    }

    #[rstest]
    #[tokio::test]
    async fn should_store_the_data(user: Vec<User>) {
        let db = InMemoryDatabase::default();
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
        assert_eq!(result.err().unwrap(), DatabaseError::Unreachable);
    }

    #[rstest]
    #[tokio::test]
    async fn should_query_the_data(user: Vec<User>) {
        let db = InMemoryDatabase::new();
        db.store(TABLE_NAME, user[0].to_owned())
            .await
            .expect("unexpected store failed");
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
        assert_eq!(result.err().unwrap(), DatabaseError::Unreachable);
    }
}
