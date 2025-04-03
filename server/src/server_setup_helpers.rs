use std::env;
use std::net::ToSocketAddrs;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerAddressError {
    #[error("SERVER_ADDRESS is set but has an invalid format and can't be converted to socked address: {0}"
    )]
    InvalidAddressFormat(String),

    #[error("No valid address found in SERVER_ADDRESS: {0}")]
    NoValidAddress(String),

    #[error("Neither PORT nor SERVER_ADDRESS are provided.")]
    MissingEnvironmentVariables,

    #[error("PORT can't be parsed into u16.")]
    InvalidPort,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    ServerAddress(#[from] ServerAddressError),
    #[error("Key {0} is not defined in the environment")]
    EnvironmentVariableUndefined(&'static str),
}

pub fn get_server_address<T>(env: Result<String, T>) -> Result<(String, u16), ServerAddressError> {
    match env {
        Ok(addr) => {
            let addr = addr
                .to_socket_addrs()
                .map_err(|_| ServerAddressError::InvalidAddressFormat(addr.clone()))?
                .next()
                .ok_or_else(|| ServerAddressError::NoValidAddress(addr))?;

            Ok((addr.ip().to_string(), addr.port()))
        }
        Err(..) => Ok((
            "0.0.0.0".to_string(),
            env::var("PORT")
                .map_err(|_| ServerAddressError::MissingEnvironmentVariables)?
                .parse()
                .map_err(|_| ServerAddressError::InvalidPort)?,
        )),
    }
}
