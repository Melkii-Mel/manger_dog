#![allow(unused_imports)]

mod configuration;
mod session;
#[macro_use]
mod macros;
mod authentication;
mod helper_implementations;
mod server_address;
mod server_starter;
pub mod crud_ops;
pub mod api;
pub mod query_builder;
pub mod static_files;

pub use crate::authentication::{LoginData, RegisterConfig, UserId};
pub use actix_surreal_starter_types::*;
pub use configuration::*;
pub use proc_macros::error_type;
pub use server_starter::*;
