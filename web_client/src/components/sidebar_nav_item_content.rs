use crate::hooks::use_file::use_file;
use yew::function_component;
use yew::{html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub src: &'static str,
    pub children: Html,
}

#[function_component]
pub fn SidebarNavItemContent(props: &Props) -> Html {
    let svg = use_file(props.src);
    html! {
        <div class="nav-item-content btn">
            {Html::from_html_unchecked((*svg).clone())}
            {props.children.clone()}
        </div>
    }
}
