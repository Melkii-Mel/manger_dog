#![allow(dead_code)]

use crate::log;
use crate::request::Method::{DELETE, GET, PATCH, POST, PUT};
use crate::utils::log;
use gloo_net::http::Response;
use gloo_net::{http, Error};
use once_cell::unsync::OnceCell;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use yew::platform::spawn_local;

#[derive(Deserialize, Default, Debug)]
pub struct RequestConfig {
    pub request_data_message: Option<&'static str>,
    pub json_response_data_message: Option<&'static str>,
    pub body_response_data_message: Option<&'static str>,
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
        }
    }
}

#[allow(dead_code)]
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

macro_rules! generic_method {
    ($name:ident$(($($param:ident:$ty:ty),*$(,)?))? |$url_ident:ident, $value_ident:ident, $callback_ident:ident| $body:block) => {
        pub fn $name<S, F, T>(url: &str, $value_ident: S, $callback_ident: F, $( $($param:$ty,)* )?)
        where
            S: Serialize + 'static,
            F: Fn(T) + 'static,
            T: DeserializeOwned + Debug,
        {
            let $url_ident = url.to_string();
            spawn_local(async move { $body })
        }
    };
}

macro_rules! method {
    ($name:ident($method:expr)) => {
        generic_method!(
            $name | url,
            value,
            callback | { Self::_generic_json_request($method, url, value, callback).await }
        );
    };
}

impl Request {
    pub fn get<F, T>(url: &str, callback: F)
    where
        F: Fn(T) + 'static,
        T: DeserializeOwned + Debug,
    {
        let url = url.to_string();
        spawn_local(async move {
            RequestBuilder::new(&url, GET)
                .finish()
                .await
                .map(async |response| {
                    if let Some(json) = read_json::<T>(response).await {
                        callback(json);
                    }
                });
        })
    }
    pub fn get_body<F>(url: &str, callback: F)
    where
        F: Fn(String) + 'static,
    {
        let url = url.to_string();
        spawn_local(async move {
            RequestBuilder::new(&url, GET)
                .finish()
                .await
                .map(async |response| {
                    if let Some(json) = read_body(response).await {
                        callback(json);
                    }
                });
        })
    }

    method!(post(POST));
    method!(put(PUT));
    method!(patch(PATCH));
    method!(delete(DELETE));

    async fn _generic_json_request<S, F, T>(method: Method, url: String, value: S, callback: F)
    where
        S: Serialize + 'static,
        F: Fn(T) + 'static,
        T: DeserializeOwned + Debug,
    {
        let response = RequestBuilder::new(&url, method)
            .json(value)
            .await;
        if let Some(response) = response {
            if let Some(json) = read_json(response).await {
                callback(json);
            }
        }
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
        let serialized = serde_json::to_string(&json).inspect_err(|e| log!("Failed to set the Json: {e}")).ok()?;
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
