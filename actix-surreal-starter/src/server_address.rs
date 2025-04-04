use crate::EnvValues;
use std::env::VarError;
use std::net::ToSocketAddrs;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerAddressError {
    #[error("server address is set but has an invalid format and can't be converted to socked address: {0}"
    )]
    InvalidAddressFormat(String),

    #[error("No valid address found in server address: {0}")]
    NoValidAddress(String),

    #[error("Neither port nor server address are provided. Internal server address error: {0}. Internal server port error: {1}"
    )]
    MissingEnvironmentVariables(String, String),

    #[error("port can't be parsed into u16.")]
    InvalidPort,
}

pub fn get_server_address<T>(env_values: &EnvValues) -> Result<(String, u16), ServerAddressError> {
    match &env_values.server_address {
        Ok(addr) => {
            let addr = addr
                .to_socket_addrs()
                .map_err(|_| ServerAddressError::InvalidAddressFormat(addr.to_string()))?
                .next()
                .ok_or_else(|| ServerAddressError::NoValidAddress(addr.to_string()))?;

            Ok((addr.ip().to_string(), addr.port()))
        }
        Err(e_address) => {
            let port: &Result<&str, VarError> = &env_values.port;
            let port = port.as_ref().map_err(|e_port| {
                ServerAddressError::MissingEnvironmentVariables(
                    format!("{:?}", e_address),
                    format!("{:?}", e_port),
                )
            })?;
            Ok((
                "0.0.0.0".to_string(),
                port.parse().map_err(|_| ServerAddressError::InvalidPort)?,
            ))
        }
    }
}
