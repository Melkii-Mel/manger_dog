use crate::authentication::AuthError::{DbError, NoRefreshToken};
use crate::endpoint_error::{EndpointError, GetStatusCode};
use crate::session::{
    build_session_token_cookies, create_session, delete_session_from_db, delete_tokens,
    refresh_session,
};
use crate::{QueriesConfig, SessionConfig, DB};
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, HttpResponseBuilder};
use bcrypt::{hash, verify, DEFAULT_COST};
use proc_macros::error_type;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;
use surrealdb::engine::remote::ws::Client;
use surrealdb::method::Query;
use surrealdb::opt::IntoQuery;

type AuthResponse = Result<HttpResponseBuilder, EndpointError<AuthError>>;
type AuthResponseWithBody = Result<HttpResponse, EndpointError<AuthError>>;

pub trait LoginData {
    fn get_password_mut(&mut self) -> &mut String;
    fn get_password(&self) -> &String;
    fn get_login(&self) -> &String;
}

#[derive(Deserialize)]
struct IdAndPassword {
    id: String,
    password: String,
}
pub async fn login(
    creds: web::Json<impl LoginData + 'static>,
    queries: Arc<QueriesConfig>,
    session_config: Arc<SessionConfig>,
) -> AuthResponse {
    let creds = creds.into_inner();
    let user: Option<IdAndPassword> = DB
        .query(queries.get_user_id_and_password_by_login)
        .bind(("login", creds.get_login().clone()))
        .await?
        .take(0)?;

    if let Some(id_and_password) = user {
        if validate_password(
            creds.get_password().as_str(),
            id_and_password.password.as_str(),
        ) {
            return Ok(
                respond_with_session_tokens(&queries, id_and_password.id, &session_config).await,
            );
        }
    }
    Err(EndpointError::new(AuthError::InvalidCredentials))
}

pub async fn logout(
    queries_config: Arc<QueriesConfig>,
    http_request: HttpRequest,
    session_config: Arc<SessionConfig>,
) -> AuthResponse {
    delete_session_from_db(
        &queries_config,
        get_access_token(&http_request, &session_config)?,
    )
    .await?;
    Ok(respond_with_tokens_deletion(&session_config).await)
}

/// This function should only be called on valid data
pub async fn register<TCreds, TQuery>(
    queries_config: Arc<QueriesConfig>,
    session_config: Arc<SessionConfig>,
    register_config: Arc<RegisterConfig<TQuery, TCreds>>,
    creds: web::Json<TCreds>,
) -> AuthResponse
where
    TQuery: IntoQuery + Clone + Send + Sync,
    TCreds: LoginData + Send + Sync,
{
    let query = DB.query(register_config.query.clone());
    let mut creds = creds.into_inner();
    let mut raw_password = creds.get_password_mut();
    hash_password(&mut raw_password)
        .map_err(|e| EndpointError::new(AuthError::PasswordHashingError).cause(e.to_string()))?;
    let query_with_bound_data = (register_config.bind_query_data)(query, creds);
    let id = query_with_bound_data
        .await?
        .take::<Option<(String,)>>(0)?
        .unwrap()
        .0;
    Ok(respond_with_session_tokens(&queries_config, id, &session_config).await)
}

pub async fn refresh(
    http_request: HttpRequest,
    session_config: Arc<SessionConfig>,
    queries_config: Arc<QueriesConfig>,
) -> AuthResponse {
    let session_tokens = refresh_session(
        http_request
            .cookie(session_config.refresh_token_cookie_name)
            .ok_or(EndpointError::new(NoRefreshToken))?
            .value()
            .to_string(),
        &queries_config,
        &session_config,
    )
    .await;
    let mut response = HttpResponse::Ok();
    build_session_token_cookies(
        &mut response,
        &session_config,
        session_tokens.access,
        Some(session_tokens.refresh),
    );
    Ok(response)
}

#[derive(Deserialize)]
pub struct Id {
    id: String,
}

pub async fn get_user<TUserdata: DeserializeOwned + Serialize>(
    http_request: HttpRequest,
    session_config: Arc<SessionConfig>,
    queries_config: Arc<QueriesConfig>,
) -> AuthResponseWithBody {
    let id: Option<Id> = DB
        .query(queries_config.get_user_id_by_access_token)
        .bind((
            "access_token",
            get_access_token(&http_request, &session_config)?,
        ))
        .await?
        .take(0)?;
    let id = id.ok_or(EndpointError::new(DbError))?.id;
    Ok(HttpResponse::Ok().content_type("application/json").json(
        DB.query(queries_config.get_userdata_by_id)
            .bind(("id", id))
            .await?
            .take::<Option<TUserdata>>(0)?
            .ok_or(EndpointError::new(DbError))?,
    ))
}

//TODO: path prefix is not necessary, should do something about this warning in the macro
#[error_type(BAD_REQUEST)]
pub enum AuthError {
    PasswordHashingError,
    InvalidCredentials,
    #[INTERNAL_SERVER_ERROR]
    DbError,
    KeyExpired,
    NoRefreshToken,
}

pub type BindQueryData<TCreds> = Box<dyn Fn(Query<Client>, TCreds) -> Query<Client> + Send + Sync>;
pub struct RegisterConfig<TQuery, TCreds>
where
    TQuery: IntoQuery + Send + Sync,
    TCreds: Send + Sync,
{
    pub query: TQuery,
    pub bind_query_data: BindQueryData<TCreds>,
}

impl<TCreds> RegisterConfig<String, TCreds>
where
    TCreds: Send + Sync,
{
    pub fn from_names(
        table_name: &str,
        names: Vec<&str>,
        bind_query_data_fn: BindQueryData<TCreds>,
    ) -> Self {
        let mut query = format!("INSERT INTO {} ", table_name);
        query += "{";
        names
            .iter()
            .for_each(|n| query += format!("{0}:${0},", n).as_str());
        query.pop();
        query += "}";
        println!("Register config query built: `{}`", query);
        Self {
            query,
            bind_query_data: bind_query_data_fn,
        }
    }
}

#[macro_export]
macro_rules! build_register_config {
    ($ty:ty, $table_name:literal { $($db_field_name: literal: $struct_field: ident),*$(,)? }) => {
        RegisterConfig::<String, $ty>::from_names($table_name, vec![$($db_field_name,)*], Box::new(|query: Query<Client>, creds:$ty| {
            query$(.bind(($db_field_name, creds.$struct_field)))*
        }))
    };
}

fn hash_password(password: &mut String) -> Result<(), Box<dyn Error>> {
    let result = hash(&password, DEFAULT_COST);
    *password = result?;
    Ok(())
}

fn validate_password(password: &str, hash: &str) -> bool {
    verify(password, hash).unwrap_or(false)
}

async fn respond_with_session_tokens(
    queries_config: &QueriesConfig,
    user_id: String,
    session_config: &SessionConfig,
) -> HttpResponseBuilder {
    let session_tokens = create_session(&queries_config, session_config, user_id).await;

    let mut response = HttpResponse::Ok();
    build_session_token_cookies(
        &mut response,
        &session_config,
        session_tokens.access,
        Some(session_tokens.refresh),
    );
    response
}

async fn respond_with_tokens_deletion(session_config: &SessionConfig) -> HttpResponseBuilder {
    let mut response = HttpResponse::Ok();
    delete_tokens(&mut response, session_config);
    response
}

impl<T: Error> From<T> for EndpointError<AuthError> {
    fn from(value: T) -> Self {
        Self::new(DbError).cause(value.to_string())
    }
}

fn get_access_token(
    http_request: &HttpRequest,
    session_config: &SessionConfig,
) -> Result<String, EndpointError<AuthError>> {
    Ok(http_request
        .cookie(session_config.access_token_cookie_name)
        .ok_or(EndpointError::new(AuthError::KeyExpired))?
        .value()
        .to_string())
}
