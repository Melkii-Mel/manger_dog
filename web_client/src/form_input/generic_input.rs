use crate::form_input::input_result::InputResult;
use crate::hooks::use_translation::use_t;
use std::rc::Rc;
use web_sys::FocusEvent;
use yew::html;
use yew::use_state;
use yew::Html;
use yew::{function_component, Callback, Properties};

pub trait InputType {
    type T: 'static + PartialEq;
    fn input_component<F: Fn(InputResult<Self::T>) + 'static>(f: F) -> Html;
    fn validate(value: &Self::T) -> Result<(), Rc<str>>;
}

#[derive(Properties, PartialEq)]
pub struct Props<T: PartialEq> {
    pub label: String,
    pub oninput: Callback<InputResult<T>>,
    #[prop_or(Ok(()))]
    pub error: Result<(), Rc<str>>,
}

impl<T> From<InputResult<T>> for Option<T> {
    fn from(value: InputResult<T>) -> Self {
        match value {
            InputResult::Ok(v) => Some(v),
            _ => None,
        }
    }
}

#[function_component]
pub fn GenericInput<T: InputType>(props: &Props<T::T>) -> Html {
    let t = use_t();
    let error_state = use_state(|| Result::<(), Rc<str>>::Ok(()));
    let focused = use_state(|| false);
    let onfocusin = {
        let focused = focused.clone();
        move |_: FocusEvent| focused.set(true)
    };
    let onfocusout = {
        let focused = focused.clone();
        move |_: FocusEvent| focused.set(false)
    };
    let input_component = T::input_component({
        let error_state = error_state.clone();
        let oninput = props.oninput.clone();
        let props_error = props.error.clone();
        let is_focused = *focused;
        move |v: InputResult<T::T>| {
            match &v {
                InputResult::Ok(v) => {
                    let error = T::validate(v).and(props_error.clone());
                    error_state.set(error);
                }
                InputResult::Err => {
                    panic!("I can't handle this! Don't call this callback with InputResult::Err");
                }
                InputResult::Incomplete => {
                    if is_focused {
                        error_state.set(Ok(()));
                    } else {
                        error_state.set(props_error.clone());
                    }
                }
                InputResult::Empty => {
                    error_state.set(props_error.clone());
                }
            };
            oninput.emit(v);
        }
    });
    let error = (&*error_state).clone();
    html! {
        <div class="form-input-field">
            <label onfocusin={onfocusin} onfocusout={onfocusout} class="form-input-label" data-erroneous={error.is_ok().to_string()}>
                <span class="form-input-title">{t(&props.label)}</span>
                {input_component}
                <script src="/static/js/input_field_visuals.js"/>
            </label>
            <span class="form-input-error">{error.err().map(|e| t(&e))}</span>
        </div>
    }
}
