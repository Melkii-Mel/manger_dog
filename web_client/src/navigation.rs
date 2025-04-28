use crate::base_hooks::use_path_state;
use crate::config::get_config;
use crate::utils::get_path;
use std::collections::HashMap;
use wasm_bindgen::JsValue;
use web_sys::{window, CustomEvent, MouseEvent};
use yew::{function_component, hook, use_context, AttrValue, KeyboardEvent, Properties};
use yew::{html, Callback, Html};
use yew::{use_state, ContextProvider};

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

pub fn change_url(path: AttrValue) {
    let window = window().expect("failed to get window while trying to change url");
    window
        .history()
        .expect("failed to get history while trying to change url")
        .push_state_with_url(
            &JsValue::NULL,
            "",
            Some(&format!("{}{}", get_config().base_url, path)),
        )
        .expect("failed to push new url into history while trying to change url");
    let event = CustomEvent::new(URL_CHANGED).expect("failed to create URL_CHANGED event");
    window
        .dispatch_event(&event)
        .expect("cannot dispatch URL_CHANGED event");
}

#[derive(Properties, PartialEq)]
pub struct NavigationItemGroupProps {
    pub url: AttrValue,
    pub children: Html,
}
#[derive(Clone, Debug, PartialEq)]
pub struct NavigationGroupUrl {
    pub url: AttrValue,
}

#[function_component]
pub fn NavigationItemGroup(navigation_item_group_props: &NavigationItemGroupProps) -> Html {
    let url = use_navigation_group_url(navigation_item_group_props.url.clone());
    use_path_state();
    let path = get_path();
    let ctx = { use_state(|| NavigationGroupUrl { url: url.clone() }) };
    if path.len() < url.len() || path[..url.len()] != url {
        return html! {};
    }
    html! {
        <div class="nav_item_group">
            <ContextProvider<NavigationGroupUrl> context={(*ctx).clone()}>
            {navigation_item_group_props.children.clone()}
            </ContextProvider<NavigationGroupUrl>>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct NavigationItemProps {
    pub url: AttrValue,
    pub children: Html,
}

#[function_component]
pub fn NavigationItem(NavigationItemProps { children, url }: &NavigationItemProps) -> Html {
    let url = use_navigation_group_url(url.clone());
    let callback = action_callback(Box::new({
        let url = url.clone();
        move || change_url(url.clone())
    }));
    let onclick = add_mouse_event_override(callback.clone());
    let onkeydown = add_keydown_event_override(callback);
    html! {
        <a href={{with_base_url(url)}} {onkeydown} {onclick}>{children.clone()}</a>
    }
}

fn with_base_url(url: AttrValue) -> AttrValue {
    combine_urls(get_config().base_url.into(), url)
}

fn combine_urls(url_a: AttrValue, url_b: AttrValue) -> AttrValue {
    format!(
        "{}/{}",
        url_a.trim_end_matches("/"),
        url_b.trim_start_matches("/")
    )
    .into()
}

#[hook]
fn use_navigation_group_url(url: AttrValue) -> AttrValue {
    let base_url = use_context::<NavigationGroupUrl>()
        .map(|base_url| base_url.url)
        .unwrap_or("".into());
    combine_urls(base_url, url)
}
