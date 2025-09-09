use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct InMemoryDatabase<T: Serialize + Send + Sync + Clone> {
    data: Arc<Mutex<HashMap<String, Vec<T>>>>,
}

impl<T: Serialize + Send + Sync + Clone> InMemoryDatabase<T> {
    pub fn new() -> Self {
        InMemoryDatabase {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn store(&self, key: &str, value: T) {
        let mut data = self.data.lock().await;
        data.entry(key.to_string())
            .or_insert_with(Vec::new)
            .push(value);
    }

    async fn query(&self, table_name: &str) -> Vec<T> {
        let data = self.data.lock().await;
        data.get(table_name).map(|v| v.to_vec()).unwrap_or_default()
    }
}

#[cfg(test)]
mod in_memory_database {
    use super::*;
    use rstest::rstest;

    #[derive(Serialize, Clone)]
    struct User {
        name: String,
    }
    const TABLE_NAME: &str = "users";

    #[rstest]
    #[tokio::test]
    async fn should_store_and_retrieve_the_data() {
        let db = InMemoryDatabase::new();
        let user = User {
            name: "Elon Musk".to_string(),
        };
        db.store(TABLE_NAME, user.clone()).await;
        let data = db.query(TABLE_NAME).await;

        assert_eq!(data.len(), 1);
        assert_eq!(data[0].name, user.name);
    }

    #[rstest]
    #[tokio::test]
    async fn should_store_and_retrieve_multiple_data() {
        let db = InMemoryDatabase::new();
        db.store(
            TABLE_NAME,
            User {
                name: "Elon Musk".to_string(),
            },
        ).await;
        db.store(
            TABLE_NAME,
            User {
                name: "Donald Trump".to_string(),
            },
        ).await;
        db.store(
            TABLE_NAME,
            User {
                name: "Bill Gates".to_string(),
            },
        ).await;
        db.store(
            TABLE_NAME,
            User {
                name: "Barack Obama".to_string(),
            },
        ).await;
        let data = db.query(TABLE_NAME).await;

        assert_eq!(data.len(), 4);
    }
}
