use std::fmt::Debug;
use phf::Map;
use serde::de::DeserializeOwned;
use serde::Serialize;
use actix_surreal_starter_types::{RecordId, TableName};
use crate::crud_api::error::CrudError;
use crate::query_builder::QueryBuilder;

pub trait Entity: TableName {
    fn paths() -> &'static [&'static str];
    fn fkey_path_map() -> &'static Map<&'static str, &'static str>;
    fn query_builder() -> QueryBuilder {
        QueryBuilder {
            table_name: Self::table_name(),
            paths: Self::paths(),
            fkey_path_map: Some(Self::fkey_path_map()),
        }
    }
}

pub trait Dto: TableName {
    type Inner: Debug + DeserializeOwned + Serialize + Entity;
    type Created: Debug + DeserializeOwned + Serialize + Id;
    fn apply(&self, user_id: &RecordId) -> impl std::future::Future<Output = Result<Self::Created, CrudError>>;
}

pub trait ToTemp<'a, TTemp> {
    fn to_temp(&'a self) -> TTemp;
}

pub trait Id {
    fn get_id(&self) -> &RecordId;
}