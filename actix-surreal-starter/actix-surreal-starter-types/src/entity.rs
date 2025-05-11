use serde::de::{DeserializeOwned, Error, Visitor};
use serde::ser::SerializeTupleStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use std::fmt::Formatter;
use std::marker::PhantomData;
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
    pub fn wrap(value: Value) -> Option<WithId<T>> {
        match value {
            Value::Object(mut map) => {
                let id: RecordId = serde_json::from_value(map.remove("id")?).ok()?;
                let data: T = serde_json::from_value(Value::Object(map)).ok()?;
                Some(WithId { id, data })
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum RecordOf<T: InsertRecord> {
    Id(RecordId),
    Record(T),
}

impl<T: InsertRecord + Serialize> Serialize for RecordOf<T> {
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

impl<'de, T: InsertRecord + DeserializeOwned> Deserialize<'de> for RecordOf<T> {
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

pub trait InsertRecord {
    fn insert(&self) -> RecordId;
}
