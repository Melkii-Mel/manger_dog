use wasm_bindgen::JsValue;
use web_sys::window;
use crate::config::get_config;

pub fn get_path() -> String {
    window()
        .expect("failed to get window while trying to get path")
        .location()
        .pathname()
        .expect("failed to get location pathname while trying to get path")
        [get_config().base_url.len()..]
        .to_string()
}

pub fn log(str: &str) {
    web_sys::console::log_1(&JsValue::from_str(str));
}