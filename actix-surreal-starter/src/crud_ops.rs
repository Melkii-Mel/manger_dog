use crate::query_builder::{BuilderError, QueryBuilder};
use crate::WithId;
use crate::DB;
use actix_surreal_starter_types::RecordId;
use actix_web::ResponseError;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::format;
use std::option::Option;
use surrealdb::opt::IntoQuery;
use thiserror::Error;
// OPTIMIZE: Could benefit from using pre-built queries (built during initialization of the server) for tables to not have to format the query each time at runtime.
// OPTIMIZE: Should consider reducing String clowning where possible.

#[derive(Debug, Error)]
pub enum CrudError {
    #[error("DB query failed: {0}")]
    DbError(#[from] surrealdb::Error),
    #[error("DB query returned an error: {0}")]
    DbResultError(String),
    #[error("Insert did not return an ID")]
    MissingId,
    #[error("Select did not find a record with the following id: {0}")]
    MissingRecord(RecordId),
    #[error("Internal error: cannot build query. Must be constructed in deeper water: {0}")]
    QueryConstructionError(#[from] BuilderError),
    #[error("Failed to perform serialization/deserialization: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("Tried to insert a user_id key, but the entity had the wrong format: {0}.\n Entity must serialize into Map. In most cases it just means that the entity must be a struct with named fields.")]
    RecordFormatError(serde_json::Value),
}

impl ResponseError for CrudError {}

pub async fn insert<T>(
    value: &T,
    user_id: RecordId,
    query_builder: QueryBuilder,
    insert_user_id: bool,
) -> Result<RecordId, CrudError>
where
    T: Serialize,
{
    let mut value = serde_json::to_value(value)?;
    if insert_user_id {
        if let serde_json::Value::Object(ref mut map) = value {
            map.insert(
                "user_id".to_string(),
                serde_json::to_value(user_id.clone())?,
            );
        } else {
            return Err(CrudError::RecordFormatError(value));
        }
    }
    let id = DB
        .query(query_builder.insert()?)
        .bind(("value", value))
        .bind(("user_id", user_id))
        .await?
        .take::<Option<RecordId>>(0)?
        .ok_or(CrudError::MissingId)?;
    Ok(id)
}

pub async fn select<T: DeserializeOwned>(
    id: RecordId,
    user_id: RecordId,
    query_builder: QueryBuilder,
) -> Result<T, CrudError> {
    Ok(DB
        .query(query_builder.select()?)
        .bind(("id", id.clone()))
        .bind(("user_id", user_id))
        .await?
        .take::<Option<T>>(0)?
        .ok_or(CrudError::MissingRecord(id.clone()))?)
}

pub async fn select_unchecked_by_id<T: DeserializeOwned>(
    id: RecordId,
) -> Result<T, CrudError> {
    Ok(DB
        .query("SELECT * FROM $id")
        .bind(("id", id.clone()))
        .await?
        .take::<Option<T>>(0)?
        .ok_or(CrudError::MissingRecord(id.clone()))?)
}

pub async fn select_all<T: DeserializeOwned>(
    user_id: RecordId,
    query_builder: QueryBuilder,
) -> Result<Vec<WithId<T>>, CrudError> {
    Ok(DB
        .query(query_builder.select_all()?)
        .bind(user_id)
        .await?
        .take::<Vec<WithId<T>>>(0)?)
}

pub async fn select_all_unchecked<T: DeserializeOwned>(
    query: impl IntoQuery,
) -> Result<Vec<WithId<T>>, CrudError> {
    Ok(DB.query(query).await?.take::<Vec<WithId<T>>>(0)?)
}

pub async fn update(
    id: RecordId,
    content_to_update: &(impl Serialize + 'static),
    user_id: RecordId,
    query_builder: QueryBuilder,
) -> Result<(), CrudError> {
    let mut content_to_update = serde_json::to_value(content_to_update)?;
    DB.query(query_builder.update()?)
        .bind(("user_id", user_id))
        .bind(("id", id))
        .bind(("value", content_to_update))
        .await?;
    Ok(())
}

pub async fn delete(
    id: RecordId,
    user_id: RecordId,
    query_builder: QueryBuilder,
) -> Result<(), CrudError> {
    DB.query(query_builder.delete()?)
        .bind(("user_id", user_id))
        .bind(("id", id))
        .await?;
    Ok(())
}
