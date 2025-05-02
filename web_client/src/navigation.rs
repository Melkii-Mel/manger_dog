use crate::hooks::{use_path_state, use_url_changed_event_listener};
use crate::config::get_config;
use crate::utils::{add_event_listener, dispatch_signal};
use crate::utils::{get_local_storage, get_path};
use std::collections::HashMap;
use wasm_bindgen::JsValue;
use web_sys::{window, MouseEvent};
use yew::{
    function_component, hook, use_context, use_effect, AttrValue, Classes, KeyboardEvent,
    Properties,
};
use yew::{html, Callback, Html};
use yew::{use_state, ContextProvider};

pub type Routes = HashMap<&'static str, Html>;
pub type DefaultRoutes = HashMap<&'static str, &'static str>;

#[macro_export]
macro_rules! routes {
    {$(
        $route_name:expr => $value:tt
    ),*$(,)?} => {
        {
            let mut result = std::collections::hash_map::HashMap::<&str, Html>::new();
            let mut result_default = std::collections::hash_map::HashMap::<&str, &str>::new();
            $(
            let child_result = routes!(@parse_tt $route_name => $value);
            result.extend(child_result.0);
            result_default.extend(child_result.1);
            )*
            (result, result_default)
        }
    };
    {@parse_tt $route_name:expr => [$( $sub_route_name:literal => $value:tt ),*$(,)?]} => {
        {
            let mut result = std::collections::hash_map::HashMap::<&str, Html>::new();
            let mut result_default: Option<std::collections::hash_map::HashMap::<&str, &str>> = None;
            $(
                let child_result = routes!(@parse_tt concat!($route_name, $sub_route_name) => $value);
                if result_default.is_none() {
                    result_default = Some(std::collections::hash_map::HashMap::<&str, &str>::from([($route_name, $sub_route_name)]));
                }
                result_default = result_default.map(|mut map| {
                    map.extend(child_result.1);
                    map
                });
                result.extend(child_result.0);
            )*
            (result, result_default.unwrap_or_default())
        }
    };
    {@parse_tt $route_name:expr => $value:tt} => {
        {
            (std::collections::hash_map::HashMap::<&str, Html>::from([($route_name, yew::html!$value)]), std::collections::hash_map::HashMap::<&str, &str>::new())
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
pub const LOCAL_STORAGE_VALUE_SET: &str = "local_storage_value_set";

pub fn change_url(path: AttrValue) {
    let window = window().expect("failed to get window while trying to change url");
    let new_url = &format!("{}{}", get_config().base_url, path);
    window
        .history()
        .expect("failed to get history while trying to change url")
        .push_state_with_url(&JsValue::NULL, "", Some(new_url))
        .expect("failed to push new url into history while trying to change url");
    dispatch_signal(URL_CHANGED);
}

#[derive(Properties, PartialEq)]
pub struct NavigationItemGroupProps {
    pub url: AttrValue,
    pub children: Html,
    #[prop_or_default]
    pub class: Classes,
}
#[derive(Clone, Debug, PartialEq)]
pub struct NavigationGroupUrl {
    pub url: AttrValue,
}

#[function_component]
pub fn NavigationItemGroup(navigation_item_group_props: &NavigationItemGroupProps) -> Html {
    let group_url = use_combine_with_navigation_group_url(navigation_item_group_props.url.clone());
    let path = use_path_state();
    let ctx = use_state(|| NavigationGroupUrl {
        url: group_url.clone(),
    });
    use_url_changed_event_listener({
        let path = path.clone();
        let group_url = group_url.clone();
        move || {
            let new_path = get_path();
            if path.starts_with(&*group_url)
                && end_url(new_path.clone()).starts_with(&*end_url(group_url.clone()))
            {
                let local_storage_set_result = get_local_storage().set(&group_url, &new_path);
                if let Ok(_) = local_storage_set_result {}
                local_storage_set_result.expect("Failed to set the value in the local storage");
                let signal = &format!("{}{}", LOCAL_STORAGE_VALUE_SET, group_url);
                dispatch_signal(signal);
            }
        }
    });
    if (*path).len() < group_url.len() || !(*path).starts_with(&*group_url) {
        return html! {};
    }
    let mut class = navigation_item_group_props.class.clone();
    class.push("nav-item-group");
    html! {
        <div class={class}>
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
    #[prop_or_default]
    pub class: Classes,
}

#[function_component]
pub fn NavigationItem(
    NavigationItemProps {
        children,
        url,
        class,
    }: &NavigationItemProps,
) -> Html {
    let base_url = use_navigation_group_url();
    let url = combine_urls(base_url.clone(), url.clone());
    let raw_url = url.clone();
    let get_url_with_state = {
        let raw_url = raw_url.clone();
        move || {
            let url = url.clone();
            let default = get_config()
                .default_routes
                .get(&*url)
                .map(|url| combine_urls(raw_url, AttrValue::from(*url)))
                .unwrap_or(url.clone());
            let new_url: AttrValue = get_local_storage()
                .get(&url)
                .map(|url| url.map(|url| url.into()))
                .unwrap_or(Some(default.clone().into()))
                .unwrap_or(default.into());
            new_url
        }
    };
    let url = use_state(get_url_with_state.clone());
    use_effect({
        let raw_url = raw_url.clone();
        let url = url.clone();
        let type_ = AttrValue::from(format!("{}{}", LOCAL_STORAGE_VALUE_SET, &raw_url));
        move || {
            add_event_listener(type_.clone(), move || {
                let updated_url = get_url_with_state();
                url.set(updated_url);
            })
        }
    });
    use_combine_with_navigation_group_url((*url).clone());
    let callback = action_callback(Box::new({
        let url = url.clone();
        move || change_url((*url).clone())
    }));
    let onclick = add_mouse_event_override(callback.clone());
    let onkeydown = add_keydown_event_override(callback);
    let mut class = class.clone();
    class.push("nav-item");
    let url = with_base_url((*url).clone());
    let current_path = use_path_state();
    let selected = format!("{}{}", get_config().base_url, *current_path) == url;
    if selected {
        class.push("selected");
    }
    html! {
        <a class={class} href={url} {onkeydown} {onclick}>{children.clone()}</a>
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
fn use_navigation_group_url() -> AttrValue {
    use_context::<NavigationGroupUrl>()
        .map(|base_url| base_url.url)
        .unwrap_or("".into())
}

#[hook]
fn use_combine_with_navigation_group_url(url: AttrValue) -> AttrValue {
    let base_url = use_navigation_group_url();
    combine_urls(base_url, url)
}

fn end_url(url: AttrValue) -> AttrValue {
    AttrValue::from(format!("{}/", url.trim_end_matches("/")))
}
