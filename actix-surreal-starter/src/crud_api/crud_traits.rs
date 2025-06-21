use actix_surreal_starter_types::WithId;
use crate::crud_api::CrudError;
use serde::de::DeserializeOwned;
use serde::Serialize;
use actix_surreal_starter_types::crud_api::RecordId;

pub trait Crud {
    async fn delete(user_id: &RecordId, record_id: &RecordId) -> Result<(), CrudError>;
}

pub trait CrudSer
where
    Self: Sized + Serialize,
{
    async fn create(&self, user_id: &RecordId) -> Result<RecordId, CrudError>;
    async fn update(&self, user_id: &RecordId, self_id: &RecordId) -> Result<(), CrudError>;
}

pub trait CrudDe
where
    Self: Sized + DeserializeOwned,
{
    async fn get(user_id: &RecordId, id: &RecordId) -> Result<Self, CrudError>;
    async fn get_all(user_id: &RecordId) -> Result<Vec<WithId<Self>>, CrudError>;
    async fn get_all_by_fkey(
        user_id: &RecordId,
        fkey_name: &str,
        fkey_value: &RecordId,
    ) -> Result<Vec<WithId<Self>>, CrudError>;
}

#[allow(dead_code)]
pub trait CrudFull
where
    Self: Crud + CrudSer + CrudDe,
{
}