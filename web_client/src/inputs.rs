use crate::input_base::InputType;
use web_sys::Event;
use yew::html;
use yew::Html;

macro_rules! generic_type {
    ($name:ident<$ty:ty>(|$oninput:ident| $html:expr) |$value:tt| $validate:expr) => {
        pub struct $name;
        impl InputType<$ty> for $name {
            fn input_component<F: Fn(Event) + 'static>(f: F) -> Html {
                let $oninput = move |e| f(Event::from(e));
                $html
            }

            fn validate($value: &$ty) -> Option<&'static str> {
                $validate
            }
        }
    };
}
macro_rules! generic_input_type {
    ($name:ident<$ty:ty>(|$callback:ident| $input_type:expr $(, $( $i_k:ident=$i_v:expr )* )?) |$value:tt| $validate:expr) => {
        generic_type!($name<$ty>(|oninput| html! {
            <input type={$input_type} $( $($i_k={$i_v})* )? $callback={oninput}/>
        }) |$value| $validate);
    };
}
macro_rules! input_type {
    ($name:ident<$ty:ty>($input_type:expr $(, $( $i_k:ident=$i_v:expr )* )?) |$value:tt| $validate:expr) => {
        generic_input_type!($name<$ty>(|oninput| $input_type $(, $($i_k={$i_v})* )?) |$value| $validate);
    };
}

macro_rules! input_change_type {
    ($name:ident<$ty:ty>($input_type:expr $(, $( $i_k:ident=$i_v:expr )* )?) |$value:tt| $validate:expr) => {
        generic_input_type!($name<$ty>(|onchange| $input_type $(, $($i_k={$i_v})* )?) |$value| $validate);
    };
}

input_type!(InputNumber<i32>("number") |_| None);
input_type!(InputNumberPositive<u32>("number", min="0") |_| None);
generic_type!(TextArea<String>(|oninput| html! {
    <textarea oninput={oninput}/>
}) |_| None);
