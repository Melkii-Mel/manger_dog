use crate::navigation::URL_CHANGED;
use crate::utils::{add_event_listener, get_path};
use wasm_bindgen::convert::ReturnWasmAbi;
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
    use_effect(move || add_event_listener(AttrValue::from(URL_CHANGED), callback));
}
