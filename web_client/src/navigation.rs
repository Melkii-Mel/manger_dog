use crate::config::get_config;
use std::collections::HashMap;
use wasm_bindgen::JsValue;
use web_sys::{window, CustomEvent, MouseEvent};
use yew::{html, Callback, Html};
use yew::KeyboardEvent;

pub type Routes = HashMap<&'static str, Html>;

#[macro_export]
macro_rules! routes {
    {$(
        $route_name:expr => $value:tt
    ),*$(,)?} => {
        {
            let mut result = std::collections::hash_map::HashMap::<&str, Html>::new();
            $( result.extend(routes!(@parse_tt $route_name => $value)); )*
            result
        }
    };
    {@parse_tt $route_name:expr => [$( $sub_route_name:literal => $value:tt ),*$(,)?]} => {
        {
            let mut result = std::collections::hash_map::HashMap::<&str, Html>::new();
            $(
                result.extend(routes!(@parse_tt concat!($route_name, $sub_route_name) => $value));
            )*
            result
        }
    };
    {@parse_tt $route_name:expr => $value:tt} => {
        {
            std::collections::hash_map::HashMap::<&str, Html>::from([($route_name, yew::html!$value)])
        }
    };
}

pub fn action_callback(action: Box<dyn Fn()>) -> Callback<()> {
    Callback::from(move |_: ()| action())
}

pub fn add_mouse_event_override(action: Callback<()>) -> Callback<MouseEvent> {
    Callback::from(move |e: MouseEvent| {
        e.prevent_default();
        action.emit(());
    })
}

pub fn add_keydown_event_override(action: Callback<()>) -> Callback<KeyboardEvent> {
    Callback::from(move |e: KeyboardEvent| {
        if e.key() == "Enter" || e.key() == " " {
            e.prevent_default();
            action.emit(());
        }
    })
}
pub const URL_CHANGED: &str = "url_changed";

pub fn change_url(path: &str) {
    let window = window().expect("failed to get window while trying to change url");
    window
        .history()
        .expect("failed to get history while trying to change url")
        .push_state_with_url(
            &JsValue::NULL,
            "",
            Some(&format!(
                "{}{}",
                get_config().base_url,
                path
            )),
        )
        .expect("failed to push new url into history while trying to change url");
    let event = CustomEvent::new(URL_CHANGED).expect("failed to create URL_CHANGED event");
    window
        .dispatch_event(&event)
        .expect("cannot dispatch URL_CHANGED event");
}

pub fn navigation_item(url: String, class: Option<String>, content: String) -> Html {
    let switch_href = {
        let url = url.clone();
        move || change_url(&url)
    };
    let callback = action_callback(Box::new(switch_href));
    let onclick = add_mouse_event_override(callback.clone());
    let onkeydown = add_keydown_event_override(callback);
    html! {
        <a href={url} {class} {onkeydown} {onclick}>{content}</a>
    }
}
