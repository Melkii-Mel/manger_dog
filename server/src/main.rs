mod api_datatypes;

use crate::api_datatypes::{configure_endpoints, Creds, Register, RegisterError};
use actix_surreal_starter::{
    build_register_config, ActixSurrealStarter, DbAccessConfig, LoginData, NamesConfig,
    RegisterConfig, ServerStarter, Users,
};
use actix_web::web::Json;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use surrealdb::engine::remote::ws::Client;
use surrealdb::method::Query;

#[get("/hello_world")]
async fn hello_world() -> impl Responder {
    "Hello, World!"
}

#[post("/increment")]
async fn increment(data: Json<i32>) -> impl Responder {
    HttpResponse::Ok().json(data.0 + 1)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    ActixSurrealStarter::<Creds>::start(
        NamesConfig {
            db_access_config: DbAccessConfig {
                users: Users {
                    login: "email",
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        },
        build_register_config!("users",
        |creds:Register|RegisterError| {
            query_config: {
                "email": creds.email,
                "username": creds.username,
                "password": creds.password,
                "registration_date": surrealdb::Datetime::from(chrono::Utc::now()),
                "selected_preference": None::<String>,
            }
            validator: creds.validate()
        }),
        |cfg| {
            cfg.service(hello_world).service(increment).route(
                "/another_hello_world/",
                web::get().to(|http_request: HttpRequest| async move {
                    format!(
                        "Hello enclosed world, you sent me a request to the following path {:?}",
                        http_request.path()
                    )
                }),
            );
            configure_endpoints(cfg);
            cfg
        },
    )
    .await
}

impl LoginData for Register {
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

// TODO: Come up with the solution for validating unsigned integers for sign and integers for size
