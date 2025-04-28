use crate::config::get_config;
use wasm_bindgen::JsValue;
use web_sys::window;
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
