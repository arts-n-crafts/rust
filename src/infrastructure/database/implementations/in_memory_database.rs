use std::collections::HashMap;
use std::sync::Arc;
use serde::Serialize;
use tokio::sync::Mutex;

pub struct InMemoryDatabase<T: Serialize + Send + Sync + Clone> {
    pub data: Arc<Mutex<HashMap<String, Vec<T>>>>,
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
mod database {
    use super::*;
    use rstest::rstest;

    #[derive(Serialize, Clone)]
    struct User {
        name: String,
    }

    #[rstest]
    #[tokio::test]
    async fn should_store_data() {
        let db = InMemoryDatabase::new();
        let user = User { name: "John".to_string() };
        db.store("user", user.clone()).await;
        let data = db.data.lock().await;
        
        assert_eq!(data.get("user").unwrap().len(), 1);
        assert_eq!(data.get("user").unwrap()[0].name, "John");
    }

    #[rstest]
    #[tokio::test]
    async fn should_retrieve_the_data() {
        let db = InMemoryDatabase::new();
        let user = User {
            name: "John".to_string(),
        };
        db.store("user", user).await;
        let data = db.query("user").await;

        assert_eq!(data.len(), 1);
        assert_eq!(data[0].name, "John");
    }
}
