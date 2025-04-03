use gloo_net::http::Request;
use gloo_net::Error;
use serde::de::DeserializeOwned;
use wasm_bindgen::JsValue;
use yew::platform::spawn_local;
use yew::prelude::*;

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
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}

fn main() {
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
