use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use crate::{Error, ServerError};

impl From<surrealdb::Error> for Error {
    fn from(value: surrealdb::Error) -> Self {
        Self::Server(ServerError::Db(value.to_string()))
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::Client(_) => StatusCode::OK,
            Error::Server(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn error_response(&self) -> HttpResponse<BoxBody> {
        match self {
            Error::Client(e) => HttpResponse::Ok().json(Err::<(), _>(e)),
            Error::Server(e) => HttpResponse::InternalServerError().json(e),
        }
    }
}

pub type ResponseResult = Result<HttpResponse, Error>;