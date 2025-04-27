use crate::config::get_config;
use crate::navigation::URL_CHANGED;
use crate::not_found::not_found;
use wasm_bindgen::closure::WasmClosureFnOnce;
use web_sys::window;
use yew::prelude::*;
use yew::{function_component, use_effect, Html};

#[function_component]
pub fn RenderPage() -> Html {
    let path = use_state(|| get_path());
    {
        let path = path.clone();
        use_effect(move || {
            let listener = js_sys::Function::from(
                (move || {
                    path.set(get_path());
                })
                .into_js_function(),
            );
            window()
                .unwrap()
                .add_event_listener_with_callback(URL_CHANGED, &listener)
                .unwrap();

            || {
                window()
                    .unwrap()
                    .remove_event_listener_with_callback(URL_CHANGED, &listener)
                    .unwrap();
                drop(listener)
            }
        });
    }
    let string_ref: &str = (*path).as_ref();

    get_config()
        .routes
        .get(string_ref)
        .map(|html| html.clone())
        .unwrap_or(not_found())
}

fn get_path() -> String {
    window()
        .expect("failed to get window while trying to get path")
        .location()
        .pathname()
        .expect("failed to get location pathname while trying to get path")
        [get_config().base_url.len()..]
        .to_string()
}
