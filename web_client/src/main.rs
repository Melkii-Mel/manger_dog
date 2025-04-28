mod access_handler;
mod bindings;
mod config;
mod navbar;
mod navigation;
mod not_found;
mod page_content;
mod refresh_request;
mod request;

use crate::access_handler::get_access;
use crate::config::{set_config, Config};
use crate::navigation::navigation_item;
use crate::page_content::RenderPage;
use crate::request::{request, RequestConfig};
use web_sys::js_sys::Math::random;
use yew::platform::spawn_local;
use yew::prelude::*;

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
                counter.set(res);
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
        routes: routes!(
            "/aaaa" => {
                <h1>{"Let us scream togetha. AAAAAAAAAAAAAAAAAAAAAA"}</h1>
            }
        ),
    };
    RequestConfig::init(RequestConfig::with_default_messages());
    set_config(config);
    spawn_local(async {
        get_access().await.unwrap();
    });
    yew::Renderer::<App>::new().render();
}
