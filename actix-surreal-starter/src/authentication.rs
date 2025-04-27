use crate::session::{
    build_session_token_cookies, create_session, delete_session_from_db, delete_tokens,
    refresh_session,
};
use crate::{NamesConfig, QueriesConfig, SessionConfig, DB};
use actix_surreal_types::{ClientError, Error, ServerError};
use actix_web::cookie::Cookie;
use actix_web::http::StatusCode;
use actix_web::{web, FromRequest, HttpRequest, HttpResponse, ResponseError};
use bcrypt::{hash, verify, DEFAULT_COST};
use proc_macros::error_type;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use surrealdb::engine::remote::ws::Client;
use surrealdb::method::Query;
use surrealdb::opt::IntoQuery;
use surrealdb::RecordId;

pub trait LoginData {
    fn get_password_mut(&mut self) -> &mut String;
    fn get_password(&self) -> &String;
    fn get_login(&self) -> &String;
}

#[derive(Deserialize)]
struct IdAndPassword {
    id: RecordId,
    password: String,
}
pub async fn login(
    creds: web::Json<impl LoginData>,
    queries: Arc<QueriesConfig>,
    session_config: Arc<SessionConfig>,
) -> actix_surreal_types::ResponseResult {
    let creds = creds.into_inner();
    let user: Option<IdAndPassword> = get_id_and_password(&queries, &creds).await?;
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
    Err(ClientError::InvalidCredentials.into())
}

async fn get_id_and_password(
    queries_config: &QueriesConfig,
    creds: &impl LoginData,
) -> Result<Option<IdAndPassword>, surrealdb::Error> {
    let mut response = DB
        .query(queries_config.get_user_id_and_password_by_login)
        .bind(("login", creds.get_login().clone()))
        .await?;
    Ok(response.take(0)?)
}

pub async fn logout(
    queries_config: Arc<QueriesConfig>,
    http_request: HttpRequest,
    session_config: Arc<SessionConfig>,
) -> actix_surreal_types::ResponseResult {
    match get_access_token(&http_request, &session_config) {
        Ok(access_token) => {
            delete_session_from_db(&queries_config, access_token).await?;
            Ok(respond_with_tokens_deletion(&session_config).await)
        }
        Err(e) => Err(e.into()),
    }
}

/// This function should only be called on valid data
pub async fn register<TUserdata, TQuery, TUserdataError>(
    queries_config: Arc<QueriesConfig>,
    session_config: Arc<SessionConfig>,
    register_config: Arc<RegisterConfig<TQuery, TUserdata, TUserdataError>>,
    creds: web::Json<TUserdata>,
) -> actix_surreal_types::ResponseResult
where
    TQuery: IntoQuery + Clone + Send + Sync,
    TUserdata: LoginData + Send + Sync,
    TUserdataError: Serialize,
{
    let mut creds = creds.into_inner();
    let validation_result = (register_config.validate)(&creds);
    if let Err(_) = validation_result {
        return Ok(HttpResponse::Ok().json(validation_result));
    }
    if get_id_and_password(&queries_config, &creds)
        .await?
        .is_some()
    {
        return Err(ClientError::EmailTaken.into());
    }
    let mut raw_password = creds.get_password_mut();
    hash_password(&mut raw_password).map_err(|e| ServerError::PasswordHashing(e.to_string()))?;
    let query = DB.query(register_config.query.clone());
    let query_with_bound_data = (register_config.bind_query_data)(query, creds);
    let id = query_with_bound_data
        .await?
        .take::<Option<RecordId>>("id")?
        .unwrap();
    Ok(respond_with_session_tokens(&queries_config, id, &session_config).await)
}

pub async fn refresh(
    http_request: HttpRequest,
    session_config: Arc<SessionConfig>,
    queries_config: Arc<QueriesConfig>,
) -> actix_surreal_types::ResponseResult {
    let refresh_token_cookie = http_request.cookie(session_config.refresh_token_cookie_name);
    match refresh_token_cookie {
        None => Err(ClientError::NoRefreshToken.into()),
        Some(refresh_token_cookie) => {
            let session_tokens = refresh_session(
                refresh_token_cookie.value().to_string(),
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
            Ok(response.json(Ok::<(),()>(())))
        }
    }
}

pub async fn get_userdata<TUserdata: DeserializeOwned + Serialize>(
    http_request: HttpRequest,
    session_config: Arc<SessionConfig>,
    queries_config: Arc<QueriesConfig>,
) -> actix_surreal_types::ResponseResult {
    match get_access_token(&http_request, &session_config) {
        Ok(access_token) => Ok(HttpResponse::Ok().json(
            DB.query(queries_config.get_userdata_by_id)
                .bind(("id", get_user_id(access_token, &queries_config).await?))
                .await?
                .take::<Option<TUserdata>>(0)?
                .ok_or(Error::Server(ServerError::Db(
                    "session found, but associated user not found".to_string(),
                )))?,
        )),
        Err(e) => Err(e.into()),
    }
}

async fn get_user_id(
    access_token: String,
    queries_config: &QueriesConfig,
) -> Result<RecordId, Error> {
    DB.query(queries_config.get_user_id_by_access_token)
        .bind(("access_token", access_token))
        .await?
        .take::<Option<RecordId>>("id")?
        .ok_or(ClientError::InvalidAccessToken.into())
}

pub type BindQueryData<TUserdata> =
    Box<dyn Fn(Query<Client>, TUserdata) -> Query<Client> + Send + Sync>;
pub type Validator<TUserdata, TUserdataError> = fn(&TUserdata) -> Result<(), TUserdataError>;
pub struct RegisterConfig<TQuery, TUserdata, TUserdataError>
where
    TQuery: IntoQuery + Send + Sync,
    TUserdata: Send + Sync,
{
    pub query: TQuery,
    pub bind_query_data: BindQueryData<TUserdata>,
    pub validate: Validator<TUserdata, TUserdataError>,
}

impl<TUserdata, TUserdataError> RegisterConfig<String, TUserdata, TUserdataError>
where
    TUserdata: Send + Sync,
{
    pub fn with_generated_query(
        table_name: &str,
        names: Vec<&str>,
        bind_query_data_fn: BindQueryData<TUserdata>,
        validator: Validator<TUserdata, TUserdataError>,
    ) -> Self {
        let mut query = format!("INSERT INTO {} ", table_name);
        query += "{";
        names
            .iter()
            .for_each(|n| query += format!("{0}:${0},", n).as_str());
        query.pop();
        query += "} RETURN id";
        Self {
            query,
            bind_query_data: bind_query_data_fn,
            validate: validator,
        }
    }
}

#[macro_export]
macro_rules! build_register_config {
    ($table_name:literal, |$ident:ident:$ty:ty|$ty_error:ty| { query_config: { $($db_field_name: literal: $value: expr),*$(,)? } validator: $validator:expr } ) => {
        RegisterConfig::<String, $ty, $ty_error>::with_generated_query($table_name, vec![$($db_field_name,)*], Box::new(|query: Query<Client>, $ident:$ty| {
            query$(.bind(($db_field_name, $value)))*
        }), |$ident| $validator)
    };
}

// TODO: extract cost factor into configuration
fn hash_password(password: &mut String) -> Result<(), Box<dyn std::error::Error>> {
    let result = hash(&password, 8);
    *password = result?;
    Ok(())
}

fn validate_password(password: &str, hash: &str) -> bool {
    verify(password, hash).unwrap_or(false)
}

async fn respond_with_session_tokens(
    queries_config: &QueriesConfig,
    user_id: RecordId,
    session_config: &SessionConfig,
) -> HttpResponse {
    let session_tokens = create_session(&queries_config, session_config, user_id).await;

    let mut response = HttpResponse::Ok();
    build_session_token_cookies(
        &mut response,
        &session_config,
        session_tokens.access,
        Some(session_tokens.refresh),
    );
    response.finish()
}

async fn respond_with_tokens_deletion(session_config: &SessionConfig) -> HttpResponse {
    let mut response = HttpResponse::Ok();
    delete_tokens(&mut response, session_config);
    response.finish()
}

fn get_access_token(
    http_request: &HttpRequest,
    session_config: &SessionConfig,
) -> Result<String, ClientError> {
    http_request
        .cookie(session_config.access_token_cookie_name)
        .map(|c| c.value().to_string())
        .ok_or(ClientError::NoAccessToken)
}

pub struct UserId(pub RecordId);

impl FromRequest for UserId {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(http_request: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let access_token = get_access_token(http_request, &NamesConfig::instance().session_config);
        Box::pin(async move {
            Ok(get_user_id(
                access_token.map_err(|_| Error::Client(ClientError::NoAccessToken))?,
                &QueriesConfig::instance(),
            )
            .await
            .map_err(|e| Error::Server(ServerError::Db(e.to_string())))
            .map(|s| UserId(s))?)
        })
    }
}
