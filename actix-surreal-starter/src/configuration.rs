use std::env;
use std::env::VarError;
use once_cell::sync::OnceCell;
use time::Duration;

macro_rules! get_env {
    ($env_name:expr) => {
        env::var($env_name).map(|v| Box::leak(v.into_boxed_str()) as &'static str)
    };
}
macro_rules! env_names {
    ({ $($field_name:ident: $default_value:expr,)*$(,)* }) => {
        names!(EnvNamesConfig { $($field_name: $default_value,)* });
        #[derive(Clone)]
        pub struct EnvValues {
            $(pub $field_name: Result<&'static str, VarError>,)*
        }
        impl EnvValues {
            pub fn new(env_names: &EnvNamesConfig) -> Self {
                Self {
                    $($field_name: get_env!(env_names.$field_name),)*
                }
            }
        }
    };
}

macro_rules! names {
    ($struct_name:ident { $($field_name:ident: $default_value:expr,)*$(,)* }) => {
        #[derive(Clone)]
        pub struct $struct_name {
            $(pub $field_name: &'static str,)*
        }
        impl Default for $struct_name {
            fn default() -> Self {
                Self {
                    $($field_name: $default_value,)*
                }
            }
        }
    };
}

macro_rules! list {
    ($struct_name:ident[$($variant:literal),*$(,)?]) => {
        pub struct $struct_name(pub Vec<&'static str>);
        impl Default for $struct_name {
            fn default() -> Self {
                Self(vec![$($variant,)*])
            }
        }
    }
}

macro_rules! tables {
    ($tables_collection_name:ident { $($table_name_pascal:ident($table_name_snake:ident): $default_table_name_value:expr, { $($field_name:ident: $default_field_name_value:expr,)*$(,)* })* }) => {
        #[derive(Clone)]
        pub struct $tables_collection_name {
            $(pub $table_name_snake: $table_name_pascal,)*
        }
        impl Default for $tables_collection_name {
            fn default() -> Self {
                Self {
                    $($table_name_snake: $table_name_pascal::default(),)*
                }
            }
        }
        $(
        #[derive(Clone)]
        pub struct $table_name_pascal {
            pub table_name: &'static str,
            $(pub $field_name: &'static str,)*
        }
        impl Default for $table_name_pascal {
            fn default() -> Self {
                Self {
                    table_name: $default_table_name_value,
                    $($field_name: $default_field_name_value,)*
                }
            }
        }
        )*
    };
}

macro_rules! queries_config {
    ($struct_name:ident ($param_name:ident: $param_type:ty)
        {
            $($field_name:ident($param_table:ident): $string_to_format:expr =>
            {
                $($param_table_field:ident),*$(,)*
            })*
        }
    ) => {
        names!($struct_name {
            $(
                $field_name: "",
            )*
        });
        impl $struct_name {
            pub fn get_formatted($param_name: $param_type) -> Self {
                Self {
                    $(
                    $field_name: format!(
                        $string_to_format,
                        $($param_name.$param_table.$param_table_field,)*
                    ).leak(),
                    )*
                }
            }
        }
    };
}

env_names!({
    server_address: "SERVER_ADDRESS",
    port: "PORT",
    db_address: "DB_ADDRESS",
    db_username: "DB_USERNAME",
    db_password: "DB_PASSWORD",
    db_namespace: "DB_NAMESPACE",
    db_name: "DB_NAME",
    static_files_serving_config: "STATIC_FILES_SERVING_CONFIG",
});

#[derive(Clone)]
pub struct SessionConfig {
    pub access_token_cookie_name: &'static str,
    pub access_token_dummy_cookie_name: &'static str,
    pub refresh_token_cookie_name: &'static str,
    pub refresh_token_dummy_cookie_name: &'static str,
    pub access_token_expiration: Duration,
    pub refresh_token_expiration: Duration,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            access_token_cookie_name: "access_token",
            access_token_dummy_cookie_name: "access_token_dummy",
            refresh_token_cookie_name: "refresh_token",
            refresh_token_dummy_cookie_name: "refresh_token_dummy",
            access_token_expiration: Duration::minutes(30),
            refresh_token_expiration: Duration::days(30),
        }
    }
}

tables!(DbAccessConfig {
    Users(users): "users", {
        login: "login",
        password: "password",
    }
    Sessions(sessions): "sessions", {
        access_token: "access_token",
        refresh_token: "refresh_token",
        access_expiration: "access_expiration",
        refresh_expiration: "refresh_expiration",
        user_id: "user_id",
    }
});

queries_config!(QueriesConfig (db_access_config: &DbAccessConfig)
{
    get_user_id_and_password_by_login(users): "SELECT id, {} as password FROM {} WHERE {} = $login" => {
        password,
        table_name,
        login,
    }
    create_session(sessions): "CREATE {} SET {} = $access_token, {} = $refresh_token, {} = $access_expiration, {} = $refresh_expiration, {} = $user_id" => {
        table_name,
        access_token,
        refresh_token,
        user_id,
        access_expiration,
        refresh_expiration,
    }
    refresh_session(sessions): "UPDATE {} SET {} = $access_token, {} = $refresh_token, {} = $access_expiration, {} = $refresh_expiration WHERE {} = $user_id" => {
        table_name,
        access_token,
        refresh_token,
        access_expiration,
        refresh_expiration,
        user_id,
    }
    delete_session(sessions): "DELETE {} WHERE {} = $access_token" => {
        table_name,
        access_token,
    }
    get_session_by_access_token(sessions): "SELECT * FROM {} WHERE {} = $access_token" => {
        table_name,
        access_token,
    }
    get_session_by_refresh_token(sessions): "SELECT * FROM {} WHERE {} = $refresh_token" => {
        table_name,
        refresh_token,
    }
    delete_expired_sessions(sessions): "DELETE {} WHERE {} < time::now()" => {
        table_name,
        refresh_expiration,
    }
    get_user_id_by_access_token(sessions): "SELECT {} FROM {} where {} = $access_token" => {
        user_id,
        table_name,
        access_token
    }
    get_userdata_by_id(users): "SELECT * OMIT {} FROM {} WHERE id = $id" => {
        password,
        table_name
    }
});

list!(EnvFilesConfig[".env"]);

#[derive(Default)]
pub struct NamesConfig {
    pub env_names_config: EnvNamesConfig,
    pub db_access_config: DbAccessConfig,
    pub session_config: SessionConfig,
    pub env_files_config: EnvFilesConfig,
}

static NAMES_CONFIG_INSTANCE: OnceCell<NamesConfig> = OnceCell::new();

impl NamesConfig {
    pub fn instance() -> &'static NamesConfig {
        NAMES_CONFIG_INSTANCE.get().expect("Internal error: Config instance is not initialized. Call NamesConfig::initialize(value) at the start of the program")
    }

    pub fn initialize(value: NamesConfig) {
        NAMES_CONFIG_INSTANCE.set(value).map_err(|_| "Config Instance is already initialized.").unwrap()
    }
}

static QUERIES_CONFIG_INSTANCE: OnceCell<QueriesConfig> = OnceCell::new();

impl QueriesConfig {
    pub fn instance() -> &'static QueriesConfig {
        QUERIES_CONFIG_INSTANCE.get().expect("Internal error: Config instance is not initialized. Call NamesConfig::initialize(value) at the start of the program")
    }

    pub fn initialize(value: QueriesConfig) {
        QUERIES_CONFIG_INSTANCE.set(value).map_err(|_| "Config Instance is already initialized.").unwrap()
    }
}