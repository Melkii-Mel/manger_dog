use gloo_net::http::Request;
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
    pub response_data_message: Option<&'static str>,
}

impl RequestConfig {
    pub fn init(self) {
        REQUEST_CONFIG.with(|cell| cell.set(Rc::new(self)).unwrap())
    }
    pub fn with_default_messages() -> Self {
        Self {
            request_data_message: Some("Sending request with data: "),
            response_data_message: Some("Received response with data: "),
        }
    }
}

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

pub fn request<T, F>(method: Method, url: &str, value: impl Serialize, callback: F)
where
    F: Fn(T) + 'static,
    T: DeserializeOwned + Debug,
{
    let request_config = REQUEST_CONFIG.with(|config| {
        config
            .get()
            .cloned()
            .unwrap_or_else(|| Rc::new(RequestConfig::default()))
    });
    let url = url.to_string();
    let value = serde_json::to_string(&value).unwrap();
    if let Some(request_data_message) = &request_config.request_data_message {
        web_sys::console::log_1(&JsValue::from_str(&format!(
            "{}{}",
            request_data_message, &value
        )));
    }
    spawn_local(async move {
        match {
            match method {
                Method::GET => Request::get,
                Method::POST => Request::post,
                Method::PUT => Request::put,
                Method::PATCH => Request::patch,
                Method::DELETE => Request::delete,
            }
        }(&url)
        .header("Content-Type", "application/json")
        .body(value)
        .unwrap()
        .send()
        .await
        {
            Ok(res) => {
                let result = res.json::<T>().await;
                if let Some(response_data_message) = &request_config.response_data_message {
                    match result {
                        Ok(result) => {
                            log(&format!("{response_data_message}{result:?}"));
                            callback(result)
                        }
                        Err(e) => {
                            log(&format!("Error: failed to parse the response: {e}"));
                            let text = res.text().await;
                            log(&text
                                .unwrap_or_else(|e| format!("Error: failed to read the body: {e}")))
                        }
                    }
                }
            }
            Err(e) => {
                log(&format!("Response error: {e}"));
            }
        }
    });
}

fn log(str: &str) {
    web_sys::console::log_1(&JsValue::from_str(str));
}
