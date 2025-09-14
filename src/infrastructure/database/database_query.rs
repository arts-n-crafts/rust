use crate::infrastructure::database::database_error::DatabaseError;
use serde::Serialize;

pub trait DatabaseQuery<T: Serialize + Send + Sync + Clone> {
    async fn execute(&self) -> Result<Vec<T>, DatabaseError>;
}

#[cfg(test)]
mod database_query {
    use super::*;
    use rstest::{fixture, rstest};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Serialize, Deserialize, Debug)]
    struct User {
        id: u8,
        name: String,
    }

    struct GetUsersNamedJohn {
        users: Vec<User>,
    }
    impl DatabaseQuery<User> for GetUsersNamedJohn {
        async fn execute(&self) -> Result<Vec<User>, DatabaseError> {
            let result = self
                .users
                .iter()
                .filter(|user| user.name == "John")
                .cloned()
                .collect();
            Ok(result)
        }
    }

    #[fixture]
    fn empty_users() -> Vec<User> {
        vec![]
    }
    #[fixture]
    fn with_users() -> Vec<User> {
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
    async fn should_retrieve_an_empty_vec_when_there_are_no_users(empty_users: Vec<User>) {
        let get_users_named_john = GetUsersNamedJohn { users: empty_users };
        let result = get_users_named_john
            .execute()
            .await
            .expect("failed to execute get_users_named_john");
        assert!(result.is_empty());
    }

    #[rstest]
    #[tokio::test]
    async fn should_retrieve_all_users_named_john(with_users: Vec<User>) {
        let get_users_named_john = GetUsersNamedJohn { users: with_users };
        let result = get_users_named_john
            .execute()
            .await
            .expect("failed to execute get_users_named_john");
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].id, 1);
        assert_eq!(result[1].id, 4);
    }
}
