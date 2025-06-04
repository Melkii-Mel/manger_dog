use crate::request::Request;
use actix_surreal_starter_types::global_entities_storage::GlobalStorage;
use actix_surreal_starter_types::{Entity, RecordId};
use actix_surreal_starter_types::WithId;
use serde::de::DeserializeOwned;
use std::any::TypeId;
use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt::Debug;
use std::rc::Rc;

thread_local! {
    static INITIALIZED: Rc<RefCell<HashSet<TypeId>>> = Rc::new(RefCell::new(HashSet::new()));
}

pub async fn get_all<T: Entity + DeserializeOwned + Debug + 'static>() -> Vec<WithId<Rc<T>>> {
    init_if_required::<T>().await;
    storage().get_all::<T>()
}

pub async fn get<T: Entity + DeserializeOwned + Debug + 'static>(id: &RecordId) -> Option<Rc<T>> {
    init_if_required::<T>().await;
    storage().get(&id)
}

pub async fn set<T: Entity + DeserializeOwned + Debug + 'static>(record: WithId<T>) -> Option<Rc<T>> {
    init_if_required::<T>().await;
    storage().set(record)
}

pub async fn delete<T: Entity + DeserializeOwned + Debug + 'static>(id: &RecordId) -> Option<Rc<T>> {
    init_if_required::<T>().await;
    storage().delete(id)
}

fn storage() -> GlobalStorage {
    actix_surreal_starter_types::global_entities_storage::get()
}

fn get_rc() -> Rc<RefCell<HashSet<TypeId>>> {
    INITIALIZED.with(|i| i.clone())
}

async fn init_if_required<T: DeserializeOwned + Debug + 'static + Entity>() {
    let rc = get_rc();
    let ty = TypeId::of::<T>();
    if !rc.borrow().contains(&ty) {
        let entities = Request::get::<Vec<WithId<T>>>(format!("{}/all", T::api_location()))
            .await
            .unwrap();
        entities.into_iter().for_each(|e| {
            storage().set(e);
        });
        rc.borrow_mut().insert(ty);
    }
}