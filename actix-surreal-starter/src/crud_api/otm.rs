use serde::{Deserialize, Serialize};
use actix_surreal_starter_types::RecordId;
use crate::crud_api::crud_traits::Crud;
use crate::crud_api::error::CrudError;

#[derive(Debug, Deserialize, Serialize)]
pub enum OtmChange<TOther> {
    Bind(TOther),
    Remove(RecordId),
}

pub trait Otmi<TSelf> {
    async fn insert(&self, fkey: &RecordId) -> Result<RecordId, CrudError>;
}

impl<T> OtmChange<T> {
    pub async fn apply<TOther: Otmi<T> + Crud>(
        changes: &Vec<OtmChange<TOther>>,
        user_id: &RecordId,
        self_id: &RecordId,
    ) -> Result<Vec<RecordId>, CrudError> {
        let mut result = Vec::new();
        for change in changes {
            match change {
                OtmChange::Bind(record) => {
                    result.push(<TOther as Otmi<T>>::insert(record, self_id).await?);
                }
                OtmChange::Remove(id) => {
                    TOther::delete(id, user_id).await?;
                }
            }
        }
        Ok(result)
    }
}