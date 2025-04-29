use crate::bindings::DomInteractionError;
use crate::refresh_request::{refresh_or_relocate, to_auth, RefreshError};
use std::collections::HashSet;
use thiserror::Error;
use web_sys::window;

// HACK: Hardcoded cookie names
pub async fn get_access() -> Result<(), AccessError> {
    let cookies = get_cookies()?;
    let keys = get_cookie_keys(cookies.as_str());
    if has_cookie(&keys, "access_token_dummy") {
        return Ok(());
    }
    if has_cookie(&keys, "refresh_token_dummy") {
        refresh_or_relocate().await?;
        return Ok(());
    }
    to_auth()?;
    Err(AccessError::NoAccess)
}

#[derive(Debug, Error)]
pub enum AccessError {
    #[error("cookies not found: {0}")]
    NoCookies(#[from] CookiesError),
    #[error("{0}")]
    Refresh(#[from] RefreshError),
    #[error("no tokens - no access. get out.")]
    NoAccess,
    #[error("External operation error: {0}")]
    ExternalOperation(#[from] DomInteractionError),
}

fn get_cookies() -> Result<String, CookiesError> {
    Ok(window()
        .ok_or(CookiesError::NoWindow)?
        .get("cookies")
        .ok_or(CookiesError::WindowHasNoCookies)?
        .as_string()
        .ok_or(CookiesError::CookiesNotString)?)
}

#[derive(Debug, Error)]
pub enum CookiesError {
    #[error("window not found")]
    NoWindow,
    #[error("window found but it has no `cookies` key")]
    WindowHasNoCookies,
    #[error("`cookies` key inside `window` is not a string")]
    CookiesNotString,
}

fn get_cookie_keys(cookie_str: &str) -> HashSet<String> {
    cookie_str
        .split(';')
        .filter_map(|cookie| {
            let key = cookie.split('=').next()?.trim();
            Some(key.to_string())
        })
        .collect()
}
fn has_cookie(cookie_keys: &HashSet<String>, name: &str) -> bool {
    cookie_keys.contains(name)
}
