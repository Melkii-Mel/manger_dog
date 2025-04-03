use crate::helper_implementations::CookieBuilder;
use crate::{QueriesConfig, SessionConfig, DB};
use actix_web::HttpResponseBuilder;
use chrono::{Duration, Utc};
use serde::Serialize;
use std::sync::Arc;
use surrealdb::Response;
use tokio::time;
use uuid::Uuid;

pub struct SessionTokens {
    pub access: TokenData,
    pub refresh: TokenData,
}

impl SessionTokens {
    fn new(session_config: &SessionConfig) -> Self {
        Self {
            access: TokenData {
                token: Uuid::new_v4().to_string(),
                expiration: (Utc::now()
                    + Duration::minutes(session_config.access_token_expiration.whole_minutes()))
                .to_rfc3339(),
            },
            refresh: TokenData {
                expiration: (Utc::now()
                    + Duration::minutes(session_config.refresh_token_expiration.whole_minutes()))
                .to_rfc3339(),
                token: Uuid::new_v4().to_string(),
            },
        }
    }
}

pub struct TokenData {
    pub token: String,
    pub expiration: String,
}

pub async fn create_session<T>(
    queries: &QueriesConfig,
    session_config: &SessionConfig,
    user_id: T,
) -> SessionTokens
where
    T: Serialize + 'static,
{
    let session_tokens = SessionTokens::new(session_config);
    DB.query(queries.create_session)
        .bind(("access_token", session_tokens.access.token.clone()))
        .bind(("refresh_token", session_tokens.refresh.token.clone()))
        .bind(("user_id", user_id))
        .bind((
            "access_expiration",
            session_tokens.access.expiration.clone(),
        ))
        .bind((
            "refresh_expiration",
            session_tokens.refresh.expiration.clone(),
        ))
        .await
        .unwrap();
    session_tokens
}

pub async fn refresh_session<T>(
    refresh_token: T,
    queries: &QueriesConfig,
    session_config: &SessionConfig,
) -> SessionTokens
where
    T: Serialize + 'static,
{
    let session_tokens = SessionTokens::new(session_config);
    DB.query(queries.refresh_session)
        .bind(("access_token", session_tokens.access.token.clone()))
        .bind(("refresh_token", refresh_token))
        .bind((
            "access_expiration",
            session_tokens.access.expiration.clone(),
        ))
        .bind((
            "refresh_expiration",
            session_tokens.refresh.expiration.clone(),
        ))
        .await
        .unwrap();
    session_tokens
}

pub async fn delete_session_from_db<T>(
    queries: &QueriesConfig,
    access_token: T,
) -> surrealdb::Result<Response>
where
    T: Serialize + 'static,
{
    DB.query(queries.delete_session)
        .bind(("access_token", access_token))
        .await
}

pub async fn cleanup_expired_sessions(queries: Arc<QueriesConfig>) -> ! {
    let mut interval = time::interval(time::Duration::from_secs(1800));

    loop {
        interval.tick().await;
        let _ = DB.query(queries.delete_expired_sessions).await;
    }
}

pub fn delete_tokens(response: &mut HttpResponseBuilder, session_config: &SessionConfig) {
    response.delete_cookies(vec![
        session_config.refresh_token_cookie_name,
        session_config.refresh_token_dummy_cookie_name,
        session_config.access_token_cookie_name,
        session_config.access_token_dummy_cookie_name,
    ]);
}

pub fn build_session_token_cookies(
    response: &mut HttpResponseBuilder,
    session_config: &SessionConfig,
    access_token: TokenData,
    refresh_token: Option<TokenData>,
) {
    response
        .build_cookie(
            session_config.access_token_cookie_name,
            access_token.token.as_str(),
            true,
            None,
            session_config.access_token_expiration,
        )
        .build_cookie(
            session_config.access_token_dummy_cookie_name,
            access_token.expiration.as_str(),
            false,
            None,
            session_config.access_token_expiration,
        );

    if let Some(refresh_token) = refresh_token {
        response
            .build_cookie(
                session_config.refresh_token_cookie_name,
                refresh_token.token.as_str(),
                true,
                Some("/refresh"),
                session_config.refresh_token_expiration,
            )
            .build_cookie(
                session_config.refresh_token_dummy_cookie_name,
                refresh_token.expiration.as_str(),
                false,
                Some("/refresh"),
                session_config.refresh_token_expiration,
            );
    }
}
