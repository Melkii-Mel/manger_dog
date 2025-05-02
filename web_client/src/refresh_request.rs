use crate::bindings::{set_location_href, DomInteractionError};
use actix_surreal_types::ClientUnitResult;
use futures::future::LocalBoxFuture;
use futures::future::Shared;
use futures::FutureExt;
use gloo_net::http::Request;
use std::cell::{OnceCell, RefCell};
use std::rc::Rc;
use derive_more::Display;
use gloo_net::Error;
use thiserror::Error;
use web_sys::{RequestCredentials};

#[derive(Debug, Error, Clone, Display)]
pub enum RefreshError {
    GlooNet(String),
    
}

impl From<gloo_net::Error> for RefreshError {
    fn from(value: Error) -> Self {
        Self::GlooNet(value.to_string())
    }
}

thread_local! {
    pub static TOKEN_REFRESHER: OnceCell<Rc<RefCell<Option<Shared<LocalBoxFuture<'static, Result<(), RefreshError>>>>>>> =
    {
        let once_cell = OnceCell::new();
        once_cell.set(Rc::new(RefCell::new(None))).ok();
        once_cell
    };
}

pub fn refresh_or_relocate() -> Shared<LocalBoxFuture<'static, Result<(), RefreshError>>> {
    TOKEN_REFRESHER.with(|cell| {
        let mut future = cell.get().unwrap().borrow_mut();
        match future.as_mut() {
            None => {
                let task = async move {
                    Request::post("/refresh")
                        .credentials(RequestCredentials::Include)
                        .send()
                        .await?
                        .json::<ClientUnitResult>()
                        .await?
                        .inspect_err(|_| to_auth().unwrap() /* Hack: unwrap */)
                        .ok();
                    Ok(())
                }
                .boxed_local()
                .shared();
                *future = Some(task.clone());
                task
            }
            Some(future) => future.clone(),
        }
    })
}

pub fn to_auth() -> Result<(), DomInteractionError> {
    // TODO: Do something about the language. Perhaps should save preferred language in the settings and then load it or something
    set_location_href("/auth") // HACK: Hardcoded value
}