use derive_more::Display;
use crate::global_entities_storage;
use serde::de::{DeserializeOwned, Error, Visitor};
use serde::ser::SerializeTupleStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use std::fmt::Formatter;
use std::marker::PhantomData;

pub trait Entity<E> {
    fn table_name() -> &'static str;
    fn api_location() -> &'static str;
    fn validate(&self) -> Result<(), E>;
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WithId<T> {
    pub id: RecordId,
    #[serde(flatten)]
    pub data: T,
}

#[cfg(not(feature = "wasm"))]
impl<T: Send + Sync + 'static> WithId<T> {
    pub fn register_record(self) {
        global_entities_storage::get().set(self);
    }
}

#[cfg(feature = "wasm")]
impl<T: 'static> WithId<T> {
    pub fn register_record(self) {
        global_entities_storage::get().set(self);
    }
}

#[derive(Debug, Display, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct RecordId(
    #[cfg(feature = "wasm")]
    Value,
    #[cfg(not(feature = "wasm"))]
    surrealdb::RecordId,
);

#[cfg(not(feature = "wasm"))]
impl From<surrealdb::RecordId> for RecordId {
    fn from(id: surrealdb::RecordId) -> Self {
        RecordId(id)
    }
}

#[cfg(not(feature = "wasm"))]
impl From<RecordId> for surrealdb::RecordId {
    fn from(value: RecordId) -> Self {
        value.0
    }
}

#[derive(Debug, Clone)]
pub enum RecordOf<T> {
    Id(RecordId),
    Record(T),
}

impl<T: Serialize> Serialize for RecordOf<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            RecordOf::Id(id) => id.serialize(serializer),
            RecordOf::Record(record) => {
                serializer.serialize_newtype_variant("RecordOf", 0, "Record", record)
            }
        }
    }
}

impl<'de, T: DeserializeOwned> Deserialize<'de> for RecordOf<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut value = Value::deserialize(deserializer)?;
        let mut is_record = false;
        if let Value::Object(map) = &value {
            if let Some(_) = map.get("Record") {
                is_record = true;
            }
        }
        if is_record {
            Ok(RecordOf::Record(
                T::deserialize(value.as_object_mut().unwrap().remove("Record").unwrap())
                    .map_err(Error::custom)?,
            ))
        } else {
            Ok(RecordOf::Id(
                serde_json::from_value(value).map_err(Error::custom)?,
            ))
        }
    }
}
pub trait _ReplaceWithIds {
    fn _replace_with_ids(self, value: &mut Value) -> Result<Self, serde_json::Error>
    where
        Self: Sized;
}
pub trait ReplaceWithIds {
    fn replace_with_ids(self, value: Value) -> Result<RecordId, serde_json::Error>;
}

trait _Blank {}
impl<T> _Blank for T {}
macro_rules! impl_replace_with_ids {
    ($( $trait_bound:ident )|*) => {
        impl<T: _ReplaceWithIds + 'static $( + $trait_bound )*> ReplaceWithIds for T {
            fn replace_with_ids(mut self, mut value: Value) -> Result<RecordId, serde_json::Error> {
                self = self._replace_with_ids(&mut value)?;
                let id: RecordId =
                    serde_json::from_value(value["id"].take()).map_err(serde::de::Error::custom)?;
                WithId {
                    id: id.clone(),
                    data: self,
                }
                .register_record();
                Ok(id)
            }
        }
    };
}

#[cfg(feature = "wasm")]
impl_replace_with_ids!();

#[cfg(not(feature = "wasm"))]
impl_replace_with_ids!(Send | Sync);
