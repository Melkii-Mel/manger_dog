use crate::crud_api::entity::Id;
use crate::crud_api::entity::Dto;
use crate::crud_api::error::CrudError;
use crate::DB;
use actix_surreal_starter_types::{RecordId, RecordOf};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum MtmChange<TSelf: JunctionBetween<TOther>, TOther> {
    Bind(RecordOf<TSelf>, TSelf::Context),
    Remove(RecordId),
}

#[derive(Deserialize, Serialize, Debug)]
pub enum MtmChangeResult<T> {
    Created(T, RecordId),
    Bound(RecordId),
    Unbound(RecordId),
}

impl<TSelf: Dto, TOther: Dto> MtmChange<TSelf, TOther>
where
    TSelf: JunctionBetween<TOther>,
    TOther: Dto,
{
    async fn apply(
        changes: &Vec<Self>,
        user_id: &RecordId,
        self_id: &RecordId,
    ) -> Result<Vec<MtmChangeResult<TSelf::Created>>, CrudError>
    where
        TSelf: JunctionBetween<TOther>,
    {
        let mut result = Vec::with_capacity(changes.len());
        for change in changes {
            result.push(match change {
                MtmChange::Bind(record, context) => match record {
                    RecordOf::Record(record) => {
                        let created = record.apply(user_id).await?;
                        let id = <TSelf as JunctionBetween<TOther>>::Junction::insert(
                            user_id,
                            self_id,
                            created.get_id(),
                            context,
                        )
                        .await?;
                        MtmChangeResult::Created(created, id)
                    }
                    RecordOf::Id(id) => MtmChangeResult::Bound(
                        <TSelf as JunctionBetween<TOther>>::Junction::insert(
                            user_id, self_id, id, context,
                        )
                        .await?,
                    ),
                },
                MtmChange::Remove(other_id) => {
                    let removed_id = <TSelf as JunctionBetween<TOther>>::Junction::remove(
                        user_id, self_id, other_id,
                    )
                    .await?;
                    MtmChangeResult::Unbound(removed_id)
                }
            });
        }
        Ok(result)
    }
}

pub trait Junction<TContext> {
    async fn insert(
        user_id: &RecordId,
        id_a: &RecordId,
        id_b: &RecordId,
        context: &TContext,
    ) -> Result<RecordId, CrudError>;
    async fn remove(
        user_id: &RecordId,
        id_a: &RecordId,
        id_b: &RecordId,
    ) -> Result<RecordId, CrudError>;
    async fn _remove(
        query: &'static str,
        user_id: &RecordId,
        id_a: &RecordId,
        id_b: &RecordId,
    ) -> Result<RecordId, CrudError> {
        Ok(DB
            .query(query)
            .bind(("user_id", serde_json::to_value(user_id)?))
            .bind(("id_a", serde_json::to_value(id_a)?))
            .bind(("id_b", serde_json::to_value(id_b)?))
            .await?
            .take::<Option<RecordId>>(0)?
            .ok_or(CrudError::MissingId)?)
    }
}

pub trait JunctionBetween<T2> {
    type Context;
    type Junction: Junction<Self::Context>;
}

