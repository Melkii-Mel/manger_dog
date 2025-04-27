mod access_handler;
mod bindings;
mod navbar;
mod navigation;
mod refresh_request;
mod page_content;
mod config;
mod not_found;

use crate::access_handler::get_access;
use crate::navigation::{navigation_item};
use gloo_net::http::Request;
use gloo_net::Error;
use serde::de::DeserializeOwned;
use wasm_bindgen::JsValue;
use web_sys::js_sys::Math::random;
use yew::platform::spawn_local;
use yew::prelude::*;
use crate::config::{set_config, Config};
use crate::page_content::RenderPage;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let counter = counter.clone();
            request("/increment", *counter, move |res| {
                if let Ok(res) = res {
                    counter.set(res)
                }
            })
        }
    };

    html! {
        <>
        {navigation_item("/".to_string(), None, "NAVIGATER!!!".to_string())}
        {navigation_item("/aaaa".to_string(), None, "AAAAAAAAA!!!".to_string())}
        <h1>{"Hello Worlds"}</h1>
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
            <p>{format!("Random stuff: {}", random())}</p>
        </div>
        <h1>{"This is the page itself::::::::"}</h1>
        <div>
            <RenderPage></RenderPage>
        </div>
        </>
    }
}

fn main() {
    let config = Config {
        base_url: "/app",
        routes: Default::default(),
    };
    set_config(config);
    spawn_local(async {
        get_access().await.unwrap();
    });
    yew::Renderer::<App>::new().render();
}

fn request<T, F>(url: &str, value: impl Into<JsValue> + 'static, callback: F)
where
    F: Fn(Result<T, Error>) + 'static,
    T: DeserializeOwned,
{
    let url = url.to_string();
    let value = value.into();
    web_sys::console::log_1(&JsValue::from_str(
        "Sending request with the following data: ",
    ));
    web_sys::console::log_1(&value);
    spawn_local(async move {
        if let Ok(res) = Request::post(&url)
            .header("Content-Type", "application/json")
            .body(value)
            .unwrap()
            .send()
            .await
            .and_then(|resp| Ok(async move { resp.json::<T>().await }))
        {
            callback(res.await)
        }
    });
}
