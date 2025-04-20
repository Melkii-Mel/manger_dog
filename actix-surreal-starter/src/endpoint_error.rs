use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

pub trait GetStatusCode {
    fn status_code(&self) -> StatusCode;
}

#[derive(Debug, Serialize)]
pub struct EndpointError<T>
where
    T: Debug + Serialize + GetStatusCode,
{
    pub cause: Option<String>,
    pub message: Option<String>,
    pub error_type: T,
}

impl<T: Debug + Serialize + GetStatusCode> Display for EndpointError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Error: {:?}", self.error_type)
    }
}

impl<T: Debug + Serialize + GetStatusCode> ResponseError for EndpointError<T> {
    fn status_code(&self) -> StatusCode {
        self.error_type.status_code()
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .json(self)
    }
}

impl<T: Debug + Serialize + GetStatusCode> EndpointError<T> {
    pub fn new(error_type: T) -> Self {
        Self {
            cause: None,
            message: None,
            error_type,
        }
    }
    builder!(
        cause: impl Into<String> => cause.into(),
        message: impl Into<String> => message.into(),
    );
}
