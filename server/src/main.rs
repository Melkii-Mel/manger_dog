mod api_datatypes;

use crate::api_datatypes::{configure_endpoints, Creds, User};
use actix_surreal_starter::{
    build_register_config, serve_app, ActixSurrealStarter, LoginData, NamesConfig, RegisterConfig,
    ServerStarter,
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
            cfg.service(hello_world)
                .service(increment)
                .route(
                    "/another_hello_world/",
                    web::get().to(|http_request: HttpRequest| {
                        async move {
                            format!("Hello enclosed world, you sent me a request to the following path {:?}", http_request.path())
                        }
                    }),
                )
                .service(serve_app("/a/", "../web_client/dist"));
            configure_endpoints(cfg);
            cfg
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
