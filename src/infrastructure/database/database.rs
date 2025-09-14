use async_trait::async_trait;
use serde::{Serialize};
use crate::infrastructure::database::database_error::DatabaseError;

#[async_trait]
pub trait Database<T>
where
    T: Serialize + Send + Sync + Clone,
{
    async fn store(&self, key: &str, value: T) -> Result<(), DatabaseError>;

    async fn query(&self, table_name: &str) -> Result<Vec<T>, DatabaseError>;
}

#[cfg(test)]
mod database_tests {
    use super::*;
    use rstest::rstest;
    use std::io::{Error as IoError, ErrorKind};
    use serde::Deserialize;

    #[derive(Serialize, Deserialize, Clone)]
    struct User {}

    struct StubDatabase;

    #[async_trait]
    impl Database<User> for StubDatabase {
        async fn store(&self, key: &str, _value: User) -> Result<(), DatabaseError> {
            let err = Box::new(IoError::new(ErrorKind::PermissionDenied, "Cannot store to table"));
            match key {
                "fail" => Err(DatabaseError::DatabaseStoreError(err)),
                _ => Ok(()),
            }
        }

        async fn query(&self, table_name: &str) -> Result<Vec<User>, DatabaseError> {
            let err = Box::new(IoError::new(ErrorKind::BrokenPipe, "Cannot query on table"));
            match table_name {
                "fail" => Err(DatabaseError::DatabaseQueryError(err)),
                _ => Ok(vec![]),
            }
        }
    }

    #[rstest]
    #[tokio::test]
    async fn it_should_succeed_storing_data() {
        let user: User = User {};
        assert!(StubDatabase.store("1", user).await.is_ok());
    }

    #[rstest]
    #[tokio::test]
    async fn it_should_fail_storing_data() {
        let user: User = User {};
        assert!(StubDatabase.store("fail", user).await.is_err());
    }

    #[rstest]
    #[tokio::test]
    async fn it_should_succeed_querying_data() {
        assert!(StubDatabase.query("1").await.is_ok());
    }

    #[rstest]
    #[tokio::test]
    async fn it_should_fail_querying_data() {
        assert!(StubDatabase.query("fail").await.is_err());
    }
}
