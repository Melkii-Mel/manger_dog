use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use crate::query_builder::QueryBuilder;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WithId<T> {
    pub id: String,
    pub inner: T,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Id(pub String);

impl<T: DeserializeOwned> WithId<T> {
    pub fn wrap(value: serde_json::Value) -> Option<WithId<T>> {
        let mut obj = value.as_object()?.clone();

        let id = obj.remove("id")?.as_str()?.to_string();
        let inner = serde_json::from_value(serde_json::Value::Object(obj)).ok()?;

        Some(WithId { id, inner })
    }
}
