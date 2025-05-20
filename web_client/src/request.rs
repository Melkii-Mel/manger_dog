#![allow(dead_code)]

use crate::access_handler::get_access;
use crate::config::get_config;
use crate::log;
use crate::request::Method::{DELETE, GET, PATCH, POST, PUT};
use crate::utils::log;
use actix_surreal_starter_types::ClientResult;
use gloo_net::http::Response;
use gloo_net::{http, Error};
use once_cell::unsync::OnceCell;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::future::Future;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use yew::platform::spawn_local;

#[derive(Deserialize, Default, Debug)]
pub struct RequestConfig {
    pub request_data_message: Option<&'static str>,
    pub json_response_data_message: Option<&'static str>,
    pub body_response_data_message: Option<&'static str>,
    pub client_error_data_message: Option<&'static str>,
}

impl RequestConfig {
    pub fn init(self) {
        REQUEST_CONFIG.with(|cell| cell.set(Rc::new(self)).unwrap())
    }
    pub fn with_default_messages() -> Self {
        Self {
            request_data_message: Some("Sending request with data: "),
            json_response_data_message: Some("Received json response: "),
            body_response_data_message: Some("Received body: "),
            client_error_data_message: Some("Client error: "),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

thread_local! {
    static REQUEST_CONFIG: OnceCell<Rc<RequestConfig>> = OnceCell::new();
}

pub struct Request;
pub struct RRequest;

impl Request {
    pub async fn get<T: DeserializeOwned + Debug>(url: String) -> Option<T> {
        let url = Rc::new(url);
        let requester = {
            let url = url.clone();
            move || RequestBuilder::new(&url, GET).finish()
        };
        Self::_generic_request_with_client_result_response::<_, _, T>(&url, requester).await
    }
    pub async fn get_body(url: &str) -> Option<String> {
        Self::get_access_if_required(url).await;
        Self::get_body_unchecked(url).await
    }
    pub async fn get_body_unchecked(url: &str) -> Option<String> {
        let response = RequestBuilder::new(url, GET).finish().await;
        Some(read_body(response?).await?)
    }
    pub async fn post<T: Serialize + 'static, R: DeserializeOwned + Debug>(
        url: String,
        data: T,
    ) -> Option<R> {
        Self::write(url, data, POST).await
    }
    pub async fn put<T: Serialize + 'static, R: DeserializeOwned + Debug>(
        url: String,
        data: T,
    ) -> Option<R> {
        Self::write(url, data, PUT).await
    }
    pub async fn patch<T: Serialize + 'static, R: DeserializeOwned + Debug>(
        url: String,
        data: T,
    ) -> Option<R> {
        Self::write(url, data, PATCH).await
    }
    pub async fn delete<T: Serialize + 'static, R: DeserializeOwned + Debug>(
        url: String,
        data: T,
    ) -> Option<R> {
        Self::write(url, data, DELETE).await
    }
    async fn write<T: Serialize + 'static, R: DeserializeOwned + Debug>(
        url: String,
        data: T,
        method: Method,
    ) -> Option<R> {
        Self::get_access_if_required(&url).await;
        Self::_generic_json_request(method, url, data).await
    }
    async fn _generic_request_with_client_result_response<F, R, T>(
        url: &str,
        requester: F,
    ) -> Option<T>
    where
        F: Fn() -> R + 'static,
        R: Future<Output = Option<Response>> + Sized,
        T: DeserializeOwned + Debug,
    {
        for _ in 0..2 {
            Self::get_access_if_required(url).await;
            let response = requester().await?;
            let client_result = read_json::<ClientResult<T>>(response).await?;
            match client_result {
                Ok(value) => {
                    return Some(value);
                }
                Err(e) => {
                    if let Some(message) = get_request_config().client_error_data_message {
                        log!("{}{:?}", message, e)
                    }
                }
            }
        }
        None
    }
    async fn _generic_json_request<T, R>(method: Method, url: String, value: T) -> Option<R>
    where
        T: Serialize + 'static,
        R: DeserializeOwned + Debug,
    {
        let value = Rc::new(value);
        let url = Rc::new(url);
        let requester = {
            let url = url.clone();
            move || RequestBuilder::new(&url, method.clone()).json(value.clone())
        };
        Self::_generic_request_with_client_result_response::<_, _, R>(&url, requester).await
    }
    async fn get_access_if_required(url: &str) {
        if get_config()
            .authorized_locations
            .iter()
            .any(|location| url.starts_with(location))
        {
            get_access().await.unwrap();
        }
    }
}

impl RRequest {
    pub async fn get<T: DeserializeOwned + Debug, F: FnOnce(T) + 'static>(
        url: String,
        callback: F,
    ) {
        spawn_local(async {
            if let Some(value) = Request::get::<T>(url).await {
                callback(value)
            }
        })
    }
    pub fn get_body<F: FnOnce(String) + 'static>(url: &str, callback: F) {
        let url = url.to_string();
        spawn_local(async move {
            if let Some(body) = Request::get_body(&url).await {
                callback(body)
            }
        })
    }
    pub fn post<T: Serialize + 'static, R: DeserializeOwned + Debug, F: FnOnce(R) + 'static>(
        url: String,
        data: T,
        callback: F,
    ) {
        spawn_local(async move {
            if let Some(client_result) = Request::post(url, data).await {
                callback(client_result)
            }
        });
    }
    pub fn put<T: Serialize + 'static, R: DeserializeOwned + Debug, F: FnOnce(R) + 'static>(
        url: String,
        data: T,
        callback: F,
    ) {
        spawn_local(async move {
            if let Some(client_result) = Request::put(url, data).await {
                callback(client_result)
            }
        });
    }
    pub fn patch<T: Serialize + 'static, R: DeserializeOwned + Debug, F: FnOnce(R) + 'static>(
        url: String,
        data: T,
        callback: F,
    ) {
        spawn_local(async move {
            if let Some(client_result) = Request::patch(url, data).await {
                callback(client_result)
            }
        });
    }
    pub fn delete<T: Serialize + 'static, R: DeserializeOwned + Debug, F: FnOnce(R) + 'static>(
        url: String,
        data: T,
        callback: F,
    ) {
        spawn_local(async move {
            if let Some(client_result) = Request::delete(url, data).await {
                callback(client_result)
            }
        });
    }
}

enum Body<B, J> {
    Body(B),
    Json(J),
    None,
}

struct RequestBuilder {
    rb: http::RequestBuilder,
}

impl RequestBuilder {
    fn new(url: &str, method: Method) -> Self {
        Self {
            rb: (match method {
                GET => http::Request::get,
                POST => http::Request::post,
                PUT => http::Request::put,
                PATCH => http::Request::patch,
                DELETE => http::Request::delete,
            })(&url),
        }
    }
    async fn body(self, body: impl Into<JsValue>) -> Option<Response> {
        let js_value = body.into();
        if let Some(message) = get_request_config().request_data_message {
            log!("{message}{:?}", &js_value)
        }
        Some(Self::_result_to_option(
            self.rb
                .body(js_value)
                .inspect_err(|e| log!("Failed to set the body: {e}"))
                .ok()?
                .send()
                .await,
        )?)
    }
    async fn json(self, json: impl Serialize) -> Option<Response> {
        let serialized = serde_json::to_string(&json)
            .inspect_err(|e| log!("Failed to set the Json: {e}"))
            .ok()?;
        if let Some(message) = get_request_config().request_data_message {
            log!("{message}{}", &serialized)
        }
        Some(Self::_result_to_option(
            self.rb
                .header("Content-Type", "application/json")
                .body(serialized)
                .inspect_err(|e| log!("Failed to set the Json: {e}"))
                .ok()?
                .send()
                .await,
        )?)
    }
    async fn finish(self) -> Option<Response> {
        Some(Self::_result_to_option(self.rb.send().await)?)
    }

    fn _result_to_option(rb: Result<Response, Error>) -> Option<Response> {
        rb.inspect_err(|e| log!("Failed to get a response: {e}"))
            .ok()
    }
}

async fn read_json<T: DeserializeOwned + Debug>(response: Response) -> Option<T> {
    let request_config = get_request_config();
    let body = read_body_(response).await?;
    let json = serde_json::from_str::<T>(&body);
    match json {
        Ok(json) => {
            if let Some(message) = request_config.json_response_data_message {
                log(&format!("{message}{json:?}"));
            }
            Some(json)
        }
        Err(e) => {
            log(&format!("Error: failed to parse json: {e}"));
            None
        }
    }
}

async fn read_body_(response: Response) -> Option<String> {
    let body = response.text().await;
    body.inspect_err(|e| {
        log(&format!("Error: failed to read the body: {e}"));
    })
    .ok()
}

async fn read_body(response: Response) -> Option<String> {
    let body = read_body_(response).await?;
    if let Some(message) = get_request_config().body_response_data_message {
        log!("{message}{body}");
    }
    Some(body)
}

fn get_request_config() -> Rc<RequestConfig> {
    REQUEST_CONFIG.with(|config| {
        config
            .get()
            .cloned()
            .unwrap_or_else(|| Rc::new(RequestConfig::default()))
    })
}
