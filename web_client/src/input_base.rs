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

pub trait InputType<T> {
    fn input_component<F: Fn(Event) + 'static>(f: F) -> Html;
    fn validate(value: &T) -> Option<&'static str>;
}

#[derive(Properties, PartialEq)]
pub struct Props<T: PartialEq> {
    pub label: String,
    pub oninput: Callback<T>,
    pub input_requester: Callback<Callback<(), T>>,
    pub error: Option<&'static str>,
}

#[function_component]
pub fn Input<
    T: InputType<TValue>,
    TValue: 'static + Default + FromStr + Display + PartialEq + Clone,
>(
    props: &Props<TValue>,
) -> Html {
    let value_state = use_state(|| TValue::default());
    let error = T::validate(&*value_state).or(props.error);
    let oninput = {
        let value_state = value_state.clone();
        let oninput = props.oninput.clone();
        move |e: Event| {
            let validator = |v: &String| {
                if let Ok(v) = v.parse::<TValue>() {
                    if T::validate(&v).is_none() {
                        return None;
                    }
                }
                Some(format!("{}", &*value_state))
            };
            if let Some(v) = extract_value(e, validator) {
                if let Ok(v) = v.parse::<TValue>() {
                    oninput.emit(v.clone());
                    value_state.set(v)
                }
            }
        }
    };
    let input_component = T::input_component(oninput);
    props.input_requester.emit(Callback::from({
        let value_state = value_state.clone();
        move |_| (*value_state).clone()
    }));
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
