use yew::{html, Html, Properties};
use yew::function_component;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub src: &'static str,
    pub children: Html,
}

#[function_component]
pub fn SidebarNavItemContent(props: &Props) -> Html {
    html!{
        <div class="nav-item-content">
            <img src={props.src}/>
            {props.children.clone()}
        </div>
    }
}