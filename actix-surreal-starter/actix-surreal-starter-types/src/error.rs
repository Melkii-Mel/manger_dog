use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use serde::de::DeserializeOwned;

#[derive(Debug, Serialize, Deserialize)]
pub enum Error {
    Client(ClientError),
    Server(ServerError),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerError {
    Db(String),
    PasswordHashing(String),
}
#[derive(Debug, Serialize, Deserialize)]
pub enum ClientError {
    NoAccessToken,
    NoRefreshToken,
    InvalidCredentials,
    EmailTaken,
    InvalidAccessToken,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::Client(client_error) => write!(f, "Client Error: {:?}", client_error),
            Error::Server(server_error) => write!(f, "Server Error: {:?}", server_error),
        }
    }
}

impl From<ServerError> for Error {
    fn from(value: ServerError) -> Self {
        Error::Server(value)
    }
}
impl From<ClientError> for Error {
    fn from(value: ClientError) -> Self {
        Error::Client(value)
    }
}
#[allow(type_alias_bounds)]
pub type ClientResult<T: Serialize + DeserializeOwned + Debug> = Result<T, ClientError>;
pub type ClientUnitResult = ClientResult<()>;
