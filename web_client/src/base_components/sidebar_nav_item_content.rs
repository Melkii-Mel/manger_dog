use crate::request::Request;
use yew::function_component;
use yew::{html, use_state, AttrValue, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub src: &'static str,
    pub children: Html,
}

#[function_component]
pub fn SidebarNavItemContent(props: &Props) -> Html {
    let svg = use_state(|| AttrValue::from(""));
    if *svg == "" {
        Request::get_body(props.src, {
            let svg = svg.clone();
            move |result: String| {
                svg.set(AttrValue::from(result))
            }
        });
    }
    html! {
        <div class="nav-item-content btn">
            {Html::from_html_unchecked((*svg).clone())}
            {props.children.clone()}
        </div>
    }
}
