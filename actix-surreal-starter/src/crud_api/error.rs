use thiserror::Error;
use actix_surreal_starter_types::crud_api::RecordId;
use crate::query_builder::BuilderError;

#[derive(Debug, Error)]
pub enum CrudError {
    #[error("DB query failed: {0}")]
    DbError(#[from] surrealdb::Error),
    #[error("Insert did not return an ID")]
    MissingId,
    #[error("Select did not find a record with the following id: {0}")]
    MissingRecord(RecordId),
    #[error("Failed to perform serialization/deserialization: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("Internal error: cannot build query. Must be constructed in deeper water: {0}")]
    QueryConstructionError(#[from] BuilderError),
}