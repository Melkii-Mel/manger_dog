use crate::request::Request;
use yew::{hook, use_state, AttrValue, UseStateHandle};

#[hook]
pub fn use_file(src: &str) -> UseStateHandle<AttrValue> {
    let content = use_state(|| AttrValue::from(""));
    if *content == "" {
        Request::get_body(src, {
            let svg = content.clone();
            move |result: String| svg.set(AttrValue::from(result))
        });
    }
    content
}
