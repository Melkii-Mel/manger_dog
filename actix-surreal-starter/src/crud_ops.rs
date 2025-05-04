use crate::query_builder::{BuilderError, QueryBuilder};
use crate::DB;
use actix_web::ResponseError;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::option::Option;
use surrealdb::RecordId;
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
}

impl ResponseError for CrudError {}

pub async fn insert<T>(
    value: T,
    user_id: RecordId,
    query_builder: QueryBuilder,
) -> Result<RecordId, CrudError>
where
    T: Serialize + 'static,
{
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
// TODO: Return WithId
pub async fn select_all<T: DeserializeOwned>(
    user_id: RecordId,
    query_builder: QueryBuilder,
) -> Result<Vec<T>, CrudError> {
    Ok(DB
        .query(query_builder.select_all()?)
        .bind(user_id)
        .await?
        .take::<Vec<T>>(0)?)
}

pub async fn update(
    id: RecordId,
    content_to_update: serde_json::Value,
    user_id: RecordId,
    query_builder: QueryBuilder,
) -> Result<(), CrudError> {
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
