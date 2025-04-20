use crate::authentication::{LoginData, RegisterConfig};
pub use crate::configuration::*;
use crate::static_files::{StaticFilesSetupError, StaticFilesSetupHandler};

use crate::authentication::{get_userdata, login, logout, refresh, register};
use crate::server_address::get_server_address;
use crate::session::cleanup_expired_sessions;
use actix_web::web::{Json, ServiceConfig};
use actix_web::{web, App, HttpRequest, HttpServer};
use colored::Colorize;
use log::error;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::io;
use std::io::ErrorKind;
use std::sync::{Arc, LazyLock};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::opt::IntoQuery;
use surrealdb::Surreal;
use thiserror::Error;

pub static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

//TODO: middleware for checking access token validity and expiration
//TODO: governor

pub trait ServerStarter<TCreds> {
    #[allow(async_fn_in_trait)]
    async fn start<TAppConfig, TRegisterQuery, TRegisterData, TRegisterDataError>(
        names_config: NamesConfig,
        register_config: RegisterConfig<TRegisterQuery, TRegisterData, TRegisterDataError>,
        app_config: TAppConfig,
    ) -> io::Result<()>
    where
        TAppConfig: Fn(&mut ServiceConfig) -> &mut ServiceConfig + Clone + Send + 'static + Sync,
        TCreds: DeserializeOwned + 'static + LoginData + Send + Sync,
        TRegisterQuery: IntoQuery + Clone + Send + Sync + 'static,
        TRegisterData: Serialize + DeserializeOwned + Send + Sync + LoginData + 'static,
        TRegisterDataError: 'static + Serialize,
    {
        let NamesConfig {
            env_names_config,
            db_access_config,
            session_config,
            env_files_config,
        } = names_config;
        env_files_config.0.iter().for_each(|filename| {
            dotenv::from_filename(filename)
                .inspect_err(|e| {
                    println!(
                        "An error occurred while importing environment file \"{}\": {:?}",
                        filename, e
                    );
                })
                .ok();
        });
        let env_values = EnvValues::new(&env_names_config);
        let queries_config = QueriesConfig::get_formatted(&db_access_config);
        to_arc!(
            session_config,
            env_values,
            queries_config,
            register_config,
            env_names_config
        );
        let address = get_server_address::<TAppConfig>(&env_values)
            .map_err(|e| io::Error::new(ErrorKind::Other, format!("{0}", e)))?;
        println!("{}", "Connecting to the database...".blue());
        db_connect(
            map_var_err!(env_values.db_address, &env_names_config.db_address)?,
            map_var_err!(env_values.db_username, &env_names_config.db_username)?,
            map_var_err!(env_values.db_password, &env_names_config.db_password)?,
            map_var_err!(env_values.db_namespace, &env_names_config.db_namespace)?,
            map_var_err!(env_values.db_name, &env_names_config.db_name)?,
        )
        .await?;
        println!("{}", "Database connection established.".green());
        tokio::spawn(cleanup_expired_sessions(queries_config.clone()));

        std::panic::set_hook(Box::new(|panic_info| {
            println!("{}", format!("Panic occurred: {:?}", panic_info).red());
        }));
        let static_files_setup_handler: Arc<dyn Fn(&mut ServiceConfig) + Send + Sync> =
            match StaticFilesSetupHandler::new(&env_values, &env_names_config) {
                Ok(handler) => {
                    let handler = Arc::new(handler);
                    handler.output_errors();
                    Arc::new(move |cfg| handler.clone().config(cfg))
                }
                Err(e) => {
                    match e {
                        StaticFilesSetupError::EnvNotSet(_, _) => {
                            println!("{}", e.to_string().blue())
                        }
                        _ => println!("{}", e.to_string().yellow()),
                    }
                    Arc::new(|_| {})
                }
            };

        let server = HttpServer::new(move || {
            let queries_config = queries_config.clone();
            let session_config = session_config.clone();
            let register_config = register_config.clone();
            App::new()
                .route(
                    "/login",
                    web::post().to(
                        enclose!((queries_config, session_config) move |creds: Json<TCreds>| {
                        login(
                            creds,
                            queries_config.clone(),
                            session_config.clone(),
                        )
                    }),
                    ),
                )
                .route(
                    "/register",
                    web::post().to(
                        enclose!((queries_config, session_config, register_config) move |creds: Json<TRegisterData>| {
                        register(
                            queries_config.clone(),
                            session_config.clone(),
                            register_config.clone(),
                            creds,
                        )
                    }),
                    ),
                )
                .route(
                    "/logout",
                    web::post().to(
                        enclose!((queries_config, session_config) move |http_request: HttpRequest| {
                        logout(queries_config.clone(), http_request, session_config.clone())
                    }),
                    ),
                )
                .route(
                    "/refresh",
                    web::post().to(
                        enclose!((queries_config, session_config) move |http_request: HttpRequest| {
                    refresh(http_request, session_config.clone(), queries_config.clone())
                    }),
                    ),
                )
                .route(
                    "/me",
                    web::get().to(enclose!((queries_config, session_config) move |http_request: HttpRequest| {
                    get_userdata::<TRegisterData>(http_request, session_config.clone(), queries_config.clone())
                })),
                )
                .configure(|cfg| { app_config(cfg); })
                .configure(enclose!((static_files_setup_handler) move |cfg| {
                    static_files_setup_handler(cfg);
                }))
        })
            .bind(&address)?
            .run();
        println!(
            "{}",
            format!(
                "Server initialized and bound to address {}:{}.",
                address.0, address.1
            )
            .green()
        );
        server.await
    }
}

pub struct ActixSurrealStarter<TCreds>(TCreds);

impl<TCreds> ServerStarter<TCreds> for ActixSurrealStarter<TCreds> {}

#[derive(Debug, Error)]
enum DbConnectionError {
    #[error("Missing required database connection information")]
    NoConnectionInfo(),
    #[error("Can't establish database connection. Address {0}. Cause: {1}")]
    NotAvailable(String, surrealdb::Error),
    #[error("Can't access database due to invalid credentials. Username: {0}. Password: {1}. Cause: {2}"
    )]
    BadCredentials(String, String, surrealdb::Error),
    #[error(
        "Can't find the namespace {0} or the database {1} inside the namespace {0}. Cause: {2}"
    )]
    DatabaseNameError(String, String, surrealdb::Error),
}

impl From<DbConnectionError> for io::Error {
    fn from(value: DbConnectionError) -> Self {
        io::Error::new(
            match value {
                DbConnectionError::NoConnectionInfo() => ErrorKind::InvalidInput,
                DbConnectionError::NotAvailable(_, _) => ErrorKind::AddrNotAvailable,
                DbConnectionError::BadCredentials(_, _, _) => ErrorKind::PermissionDenied,
                DbConnectionError::DatabaseNameError(_, _, _) => ErrorKind::InvalidData,
            },
            value.to_string(),
        )
    }
}

async fn db_connect(
    address: &str,
    username: &str,
    password: &str,
    namespace: &str,
    name: &str,
) -> Result<(), DbConnectionError> {
    if address.is_empty() || username.is_empty() || password.is_empty() {
        return Err(DbConnectionError::NoConnectionInfo());
    }

    DB.connect::<Ws>(address)
        .await
        .map_err(|e| DbConnectionError::NotAvailable(address.to_string(), e))?;

    DB.signin(Root { username, password }).await.map_err(|e| {
        DbConnectionError::BadCredentials(username.to_string(), password.to_string(), e)
    })?;

    DB.use_ns(namespace).use_db(name).await.map_err(|e| {
        DbConnectionError::DatabaseNameError(namespace.to_string(), name.to_string(), e)
    })?;

    Ok(())
}
