use crate::crud_api::entity::Entity;
use actix_surreal_starter_types::WithId;
use crate::crud_api::crud_traits::CrudFull;
use crate::crud_api::Crud;
use crate::crud_api::crud_traits::CrudSer;
use crate::crud_api::crud_traits::CrudDe;
use crate::crud_api::CrudError;
use serde::de::DeserializeOwned;
use serde::Serialize;
use actix_surreal_starter_types::crud_api::RecordId;
use crate::DB;

fn query<'a>(
    query: String,
    user_id: &RecordId,
) -> Result<surrealdb::method::Query<'a, surrealdb::engine::remote::ws::Client>, CrudError> {
    Ok(DB
        .query(query)
        .bind(("user_id", serde_json::to_value(user_id)?)))
}

async fn execute_option<T: DeserializeOwned, E, F: FnOnce() -> E>(
    query: surrealdb::method::Query<'_, surrealdb::engine::remote::ws::Client>,
    error: F,
) -> Result<T, CrudError>
where
    CrudError: From<E>,
{
    Ok(query.await?.take::<Option<T>>(0)?.ok_or_else(error)?)
}

async fn execute_vec<T: DeserializeOwned>(
    query: surrealdb::method::Query<'_, surrealdb::engine::remote::ws::Client>,
) -> Result<Vec<T>, CrudError> {
    Ok(query.await?.take::<Vec<T>>(0)?)
}

async fn execute_none(
    query: surrealdb::method::Query<'_, surrealdb::engine::remote::ws::Client>,
) -> Result<(), CrudError> {
    query.await?;
    Ok(())
}

impl<T: Entity> Crud for T {
    async fn delete(user_id: &RecordId, record_id: &RecordId) -> Result<(), CrudError> {
        execute_none(
            query(T::query_builder().delete()?, user_id)?
                .bind(("id", serde_json::to_value(record_id)?)),
        )
            .await
    }
}

impl<T: Entity + Serialize> CrudSer for T {
    async fn create(&self, user_id: &RecordId) -> Result<RecordId, CrudError> {
        execute_option(
            query(T::query_builder().insert()?, user_id)?
                .bind(("value", serde_json::to_value(self)?)),
            || CrudError::MissingId,
        )
            .await
    }

    async fn update(&self, user_id: &RecordId, id: &RecordId) -> Result<(), CrudError> {
        execute_none(
            query(T::query_builder().update()?, user_id)?
                .bind(("id", serde_json::to_value(id)?))
                .bind(("value", serde_json::to_value(self)?)),
        )
            .await
    }
}

impl<T: Entity + DeserializeOwned> CrudDe for T {
    async fn get(user_id: &RecordId, id: &RecordId) -> Result<Self, CrudError> {
        execute_option(
            query(T::query_builder().select()?, user_id)?.bind(("id", serde_json::to_value(id)?)),
            || CrudError::MissingRecord(id.clone()),
        )
            .await
    }

    async fn get_all(user_id: &RecordId) -> Result<Vec<WithId<Self>>, CrudError> {
        execute_vec(query(T::query_builder().select_all()?, user_id)?).await
    }

    async fn get_all_by_fkey(
        user_id: &RecordId,
        fkey_name: &str,
        fkey_value: &RecordId,
    ) -> Result<Vec<WithId<Self>>, CrudError>
    where
        T: DeserializeOwned,
    {
        execute_vec(
            query(T::query_builder().select_all_by_fkey(fkey_name)?, user_id)?
                .bind(("fkey", serde_json::to_value(fkey_value)?)),
        )
            .await
    }
}

impl<T: Entity + DeserializeOwned + Serialize> CrudFull for T {}