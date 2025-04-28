use crate::navigation::URL_CHANGED;
use crate::utils::get_path;
use wasm_bindgen::closure::WasmClosureFnOnce;
use web_sys::window;
use yew::{hook, use_effect, use_state, UseStateHandle};

#[hook]
pub fn use_path_state() -> UseStateHandle<String> {
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
    path
}
