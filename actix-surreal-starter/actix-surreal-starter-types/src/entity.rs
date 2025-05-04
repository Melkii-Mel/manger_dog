use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

pub trait Entity<E> {
    fn table_name() -> &'static str;
    fn api_location() -> &'static str;
    fn validate(&self) -> Result<(), E>;
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WithId<T> {
    pub id: RecordId,
    pub data: T,
}

impl<T: DeserializeOwned> WithId<T> {
    pub fn wrap(value: serde_json::Value) -> Option<WithId<T>> {
        match value {
            serde_json::Value::Object(mut map) => {
                let id: RecordId = serde_json::from_value(map.remove("id")?).ok()?;
                let data: T = serde_json::from_value(serde_json::Value::Object(map)).ok()?;
                Some(WithId { id, data })
            }
            _ => None,
        }
    }
}
