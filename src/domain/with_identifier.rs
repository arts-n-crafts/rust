use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct WithIdentifier {
    pub id: String,
}

#[cfg(test)]
mod with_identifier_tests {
    use rstest::rstest;
    use uuid::Uuid;
    use super::*;

    #[rstest]
    fn it_should_contain_an_id() {
        let id = Uuid::now_v7().to_string();
        let obj = WithIdentifier { id: id.clone() };
        assert_eq!(id, obj.id);
    }
}
