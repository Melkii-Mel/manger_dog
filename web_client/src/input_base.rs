use std::fmt::Display;
use std::str::FromStr;
use wasm_bindgen::JsCast;
use yew::html;
use yew::use_state;
use yew::Event;
use yew::Html;
use yew::{function_component, Callback, Properties};

// TODO: Consider changing validation semantics. Using Options is a little confusing.
// TODO: Consider allowing invalid inputs including invalid format to not restrict user input and instead provide error messages.
// TODO: Add localization instead of printing error codes

pub trait InputType {
    type T: 'static + Default + FromStr + Display + PartialEq + Clone;
    fn input_component<F: Fn(Event) + 'static>(f: F) -> Html;
    fn validate(value: &Self::T) -> Option<&'static str>;
}

pub trait TypeConversion<TIn, TOut>: InputType<T = TIn> {
    fn convert(input: TIn) -> TOut;
}

impl<A: InputType<T = T>, T> TypeConversion<T, T> for A {
    fn convert(input: T) -> T {
        input
    }
}

#[derive(Properties, PartialEq)]
pub struct Props<T: PartialEq> {
    pub label: String,
    pub oninput: Callback<T>,
    #[prop_or_default]
    pub input_requester: Option<Callback<Callback<(), T>>>,
    pub error: Option<&'static str>,
}

#[function_component]
pub fn Input<T: InputType>(props: &Props<T::T>) -> Html {
    let value_state = use_state(|| T::T::default());
    let error = T::validate(&*value_state).or(props.error);
    let oninput = {
        let value_state = value_state.clone();
        let oninput = props.oninput.clone();
        move |e: Event| {
            let validator = |v: &String| {
                if let Ok(v) = v.parse::<T::T>() {
                    if T::validate(&v).is_none() {
                        return None;
                    }
                }
                Some(format!("{}", &*value_state))
            };
            if let Some(v) = extract_value(e, validator) {
                if let Ok(v) = v.parse::<T::T>() {
                    oninput.emit(v.clone());
                    value_state.set(v)
                }
            }
        }
    };
    let input_component = T::input_component(oninput);
    if let Some(requester) = &props.input_requester {
        requester.emit(Callback::from({
            let value_state = value_state.clone();
            move |_| (*value_state).clone()
        }))
    };
    html! {
        <div class="form-input-field">
            <label class="form-input-label" data-erroneous={error.is_some().to_string()}>
                <span class="form-input-title">{&props.label}</span>
                {input_component}
                <script src="/static/js/input_field_visuals.js"/>
            </label>
            <span class="form-input-error">{error}</span>
        </div>
    }
}

fn extract_value<F: FnOnce(&String) -> Option<String>>(
    _event: Event,
    _validator: F,
) -> Option<String> {
    macro_rules! try_cast_and_validate {
        ($typ:ty) => {
            if let Ok(elem) = _event
                .target()
                .expect("event doesn't have a target when calling extract_value")
                .dyn_into::<$typ>()
            {
                return {
                    let value = elem.value();
                    if let Some(v) = _validator(&value) {
                        elem.set_value(&v);
                        None
                    } else {
                        Some(value)
                    }
                };
            }
        };
    }

    use web_sys::*;

    try_cast_and_validate!(HtmlInputElement);
    try_cast_and_validate!(HtmlSelectElement);
    try_cast_and_validate!(HtmlTextAreaElement);

    panic!("unexpected input type. Only <input>, <select> and <textarea> are supported");
}
