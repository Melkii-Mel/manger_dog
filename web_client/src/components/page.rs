use crate::config::get_config;
use crate::hooks::use_path_state;
use crate::not_found::not_found;
use yew::{function_component, Html};

#[function_component]
pub fn Page() -> Html {
    let path = use_path_state();
    let string_ref: &str = (*path).as_ref();

    get_config()
        .routes
        .get(string_ref)
        .map(|html| html.clone())
        .unwrap_or(not_found())
}
