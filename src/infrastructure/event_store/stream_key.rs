use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize,  Debug, Clone, PartialEq, Eq, Hash)]
pub struct StreamKey(String);

impl StreamKey {
    pub fn new(stream_name: &str, aggregate_id: Uuid) -> Self {
        let key = format!("{}#{}", stream_name.to_lowercase(), aggregate_id);
        Self(key)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod stream_key_test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn it_should_return_a_stream_key(){
        let stream_name = "users";
        let aggregate_id = Uuid::now_v7();
        let stream_key = StreamKey::new(stream_name, aggregate_id);
        assert_eq!(stream_key.as_str(), format!("users#{}", aggregate_id));
    }
}
