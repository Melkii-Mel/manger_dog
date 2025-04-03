mod api;
mod api_datatypes;
mod server_setup_helpers;

use crate::api_datatypes::{Creds, User};
use actix_surreal_starter::{
    build_register_config, serve_app, ActixSurrealStarter, LoginData, NamesConfig, RegisterConfig,
    ServerStarter,
};
use actix_web::web::Json;
use actix_web::{get, post, HttpResponse, Responder};
use surrealdb::engine::remote::ws::Client;
use surrealdb::method::Query;

#[get("/hello_world")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[post("/increment")]
async fn increment(data: Json<i32>) -> impl Responder {
    HttpResponse::Ok().json(data.0 + 1)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    ActixSurrealStarter::<Creds>::start(
        NamesConfig::default(),
        build_register_config!(User, "users"
        {
            "email": email,
            "username": username,
            "password": password,
            "registration_date": registration_date,
            "selected_preference": selected_preference,
        }),
        |cfg| {
            cfg.service(serve_app(
                "/a/",
                "../web_client/dist",
                true, /*Change to false on deploy*/
            ))
        },
    )
    .await
}

impl LoginData for User {
    fn get_password_mut(&mut self) -> &mut String {
        &mut self.password
    }

    fn get_password(&self) -> &String {
        &self.password
    }

    fn get_login(&self) -> &String {
        &self.email
    }
}

impl LoginData for Creds {
    fn get_password_mut(&mut self) -> &mut String {
        &mut self.password
    }

    fn get_password(&self) -> &String {
        &self.password
    }

    fn get_login(&self) -> &String {
        &self.email
    }
}
