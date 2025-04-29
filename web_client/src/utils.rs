use crate::config::get_config;
use wasm_bindgen::closure::WasmClosureFnOnce;
use wasm_bindgen::convert::ReturnWasmAbi;
use wasm_bindgen::JsValue;
use web_sys::{window, CustomEvent, Storage};
use yew::AttrValue;

pub fn get_path() -> AttrValue {
    let path = window()
        .expect("failed to get window while trying to get path")
        .location()
        .pathname()
        .expect("failed to get location pathname while trying to get path")
        [get_config().base_url.len()..]
        .to_string();
    AttrValue::from(path)
}

pub fn log(str: &str) {
    web_sys::console::log_1(&JsValue::from_str(str));
}

pub fn get_local_storage() -> Storage {
    window()
        .expect("no global `window` exists")
        .local_storage()
        .expect("should have `localStorage` available")
        .expect("`localStorage` was `None`")
}

pub fn dispatch_signal(type_: &str) {
    let window = window().expect(&format!(
        "failed to get window while trying to dispatch signal: {}",
        type_
    ));
    let event = CustomEvent::new(type_).expect("failed to create URL_CHANGED event");
    window
        .dispatch_event(&event)
        .expect(&format!("cannot dispatch signal: {}", type_));
}

pub fn add_event_listener<T: 'static + FnOnce() -> R, R: ReturnWasmAbi + 'static>(
    type_: AttrValue,
    callback: T,
) -> Box<dyn Fn()> {
    let listener = js_sys::Function::from(callback.into_js_function());
    window()
        .expect(&format!(
            "failed to get window to add listener to the event: {}",
            type_
        ))
        .add_event_listener_with_callback(&*type_, &listener)
        .expect(&format!("failed to add listener to the event: {}", type_));

    Box::new(move || {
        window()
            .expect(&format!(
                "failed to get window to remove listener from the event: {}",
                type_
            ))
            .remove_event_listener_with_callback(&*type_, &listener)
            .expect(&format!(
                "failed to remove listener from the event: {}",
                type_
            ));
    })
}
