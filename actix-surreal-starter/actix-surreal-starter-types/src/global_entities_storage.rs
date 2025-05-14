use crate::WithId;
use std::any::Any;
use std::any::TypeId;
use std::cell::{LazyCell, OnceCell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Mutex;
use std::sync::{Arc, LazyLock, OnceLock};
use surrealdb::RecordId;

#[cfg(feature = "wasm")]
thread_local!(
    static GLOBAL_STORAGE_OWNER: GlobalStorageOwner = GlobalStorageOwner::new();
);

#[cfg(feature = "wasm")]
pub fn get() -> GlobalStorage {
    let mut storage = None::<GlobalStorage>;
    GLOBAL_STORAGE_OWNER.with(|owner| storage = Some(owner.inner.clone()));
    storage.unwrap()
}
#[cfg(not(feature = "wasm"))]
static GLOBAL_STORAGE_OWNER: GlobalStorageOwner = GlobalStorageOwner::new();

#[cfg(not(feature = "wasm"))]
pub fn get() -> GlobalStorage {
    GLOBAL_STORAGE_OWNER.inner.clone()
}

#[cfg(feature = "wasm")]
type StorageMap = HashMap<TypeId, HashMap<RecordId, Rc<dyn Any>>>;

#[cfg(not(feature = "wasm"))]
type StorageMap = HashMap<TypeId, HashMap<RecordId, Arc<dyn Any + Send + Sync>>>;

#[derive(Debug)]
pub struct GlobalStorageOwner {
    #[cfg(feature = "wasm")]
    inner: LazyCell<GlobalStorage>,
    #[cfg(not(feature = "wasm"))]
    inner: LazyLock<GlobalStorage>,
}

#[derive(Debug, Clone)]
pub struct GlobalStorage {
    #[cfg(feature = "wasm")]
    inner: Rc<RefCell<StorageMap>>,
    #[cfg(not(feature = "wasm"))]
    inner: Arc<Mutex<StorageMap>>,
}

impl GlobalStorageOwner {
    pub const fn new() -> Self {
        Self {
            inner: {
                #[cfg(feature = "wasm")]
                {
                    LazyCell::new(|| GlobalStorage {
                        inner: Rc::new(RefCell::new(HashMap::new())),
                    })
                }
                #[cfg(not(feature = "wasm"))]
                {
                    LazyLock::new(|| GlobalStorage {
                        inner: Arc::new(Mutex::new(HashMap::new())),
                    })
                }
            },
        }
    }
}

impl GlobalStorage {
    #[cfg(feature = "wasm")]
    pub fn set<T: 'static>(&self, entry: WithId<T>) -> Option<Rc<T>> {
        self.inner
            .borrow_mut()
            .entry(TypeId::of::<T>())
            .or_insert(HashMap::new())
            .insert(entry.id, Rc::new(entry.data))
            .map(|arc| {
                arc.downcast::<T>()
                    .expect("Failed to downcast a type upon insertion")
            })
    }

    #[cfg(not(feature = "wasm"))]
    pub fn set<T: Send + Sync + 'static>(&self, entry: WithId<T>) -> Option<Arc<T>> {
        self.inner
            .lock()
            .unwrap()
            .entry(TypeId::of::<T>())
            .or_insert(HashMap::new())
            .insert(entry.id, Arc::new(entry.data))
            .map(|arc| {
                arc.downcast::<T>()
                    .expect("Failed to downcast a type upon insertion")
            })
    }

    #[cfg(feature = "wasm")]
    pub fn get<T: 'static>(&self, id: &RecordId) -> Option<Rc<T>> {
        Some(
            self.inner
                .borrow_mut()
                .get(&TypeId::of::<T>())?
                .get(id)?
                .clone()
                .downcast::<T>()
                .expect("Failed to downcast a type upon retrieval"),
        )
    }

    #[cfg(not(feature = "wasm"))]
    pub fn get<T: Send + Sync + 'static>(&self, id: &RecordId) -> Option<Arc<T>> {
        Some(
            self.inner
                .lock()
                .unwrap()
                .get(&TypeId::of::<T>())?
                .get(id)?
                .clone()
                .downcast::<T>()
                .expect("Failed to downcast a type upon retrieval"),
        )
    }

    #[cfg(feature = "wasm")]
    pub fn delete<T: 'static>(&self, id: &RecordId) -> Option<Rc<T>> {
        Some(
            self.inner
                .borrow_mut()
                .get_mut(&TypeId::of::<T>())?
                .remove(id)?
                .downcast::<T>()
                .expect("Failed to downcast a type upon deletion"),
        )
    }

    #[cfg(not(feature = "wasm"))]
    pub fn delete<T: Send + Sync + 'static>(&self, id: &RecordId) -> Option<Arc<T>> {
        Some(
            self.inner
                .lock()
                .unwrap()
                .get_mut(&TypeId::of::<T>())?
                .remove(id)?
                .downcast::<T>()
                .expect("Failed to downcast a type upon deletion"),
        )
    }
}
