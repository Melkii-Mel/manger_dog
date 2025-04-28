use crate::navigation::URL_CHANGED;
use crate::utils::get_path;
use wasm_bindgen::closure::WasmClosureFnOnce;
use wasm_bindgen::convert::ReturnWasmAbi;
use web_sys::window;
use yew::{hook, use_effect, use_state, AttrValue, UseStateHandle};

#[hook]
pub fn use_path_state() -> UseStateHandle<AttrValue> {
    let path = use_state(|| get_path());
    {
        let path = path.clone();
        use_url_changed_event_listener(move || {
            path.set(get_path());
        })
    }
    path
}

#[hook]
pub fn use_url_changed_event_listener<T, R>(callback: T)
where
    T: 'static + FnOnce() -> R,
    R: ReturnWasmAbi + 'static,
{
    use_effect(move || {
        let listener = js_sys::Function::from(callback.into_js_function());
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
