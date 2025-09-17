use serde::Serialize;

pub trait BasePayload: Serialize + Send + Sync + Clone {}
impl<T> BasePayload for T where T: Serialize + Send + Sync + Clone {}
