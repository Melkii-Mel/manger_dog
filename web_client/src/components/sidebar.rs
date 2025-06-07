use crate::MouseEvent;
use wasm_bindgen::JsCast;
use web_sys::Element;
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
}
#[function_component]
pub fn Sidebar(props: &Props) -> Html {
    let sidebar_toggle_click = |mouse_event: MouseEvent| {
        let target = mouse_event.target().unwrap();
        let element = target.dyn_ref::<Element>().unwrap().to_owned();
        element
            .closest(".sidebar")
            .unwrap()
            .unwrap()
            .class_list()
            .toggle("collapsed")
            .ok();
    };
    html!(
        <div class="sidebar">
            {props.children.clone()}
            <button class="sidebar-toggle" onclick={sidebar_toggle_click}></button>
        </div>
    )
}
