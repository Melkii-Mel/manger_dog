use crate::bindings::DomInteractionError::FailedToSetHref;
use thiserror::Error;
use wasm_bindgen::JsValue;
use web_sys::window;

#[derive(Debug, Error)]
pub enum DomInteractionError {
    #[error("Window not found in the document")]
    NoWindow,
    #[error("Can't set href: {0:?}")]
    FailedToSetHref(JsValue),
}

pub fn set_location_href(new_url: &str) -> Result<(), DomInteractionError> {
    let win = window().ok_or(DomInteractionError::NoWindow)?;
    win.location()
        .set_href(new_url)
        .map_err(|e| FailedToSetHref(e))
}
