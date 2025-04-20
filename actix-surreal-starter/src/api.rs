use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use surrealdb::RecordId;

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
