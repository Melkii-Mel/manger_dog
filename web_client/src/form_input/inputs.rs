use crate::form_input::form::HtmlInputs;
use crate::form_input::generic_input::InputType;
use crate::form_input::input_result::InputResult;
use crate::global_storage;
use crate::hooks::use_translation::use_t;
use crate::parsing::{parse_int, parse_money};
use actix_surreal_starter_types::RecordOf;
use actix_surreal_starter_types::{Entity, RecordId};
use chrono::{DateTime, Timelike, Utc};
use serde::de::DeserializeOwned;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;
use web_sys::{Event, HtmlSelectElement, InputEvent};
use yew::platform::spawn_local;
use yew::{function_component, html, use_state, AttrValue, Callback, Properties};
use yew::{Html, TargetCast};

fn get_value_from_event(_event: impl Into<Event>) -> (String, Event) {
    let _event = _event.into();
    macro_rules! try_cast {
        ($typ:ty) => {
            if let Ok(elem) = _event
                .target()
                .expect("event doesn't have a target when calling extract_value")
                .dyn_into::<$typ>()
            {
                return (elem.value(), _event);
            }
        };
    }

    use wasm_bindgen::JsCast;
    use web_sys::*;

    try_cast!(HtmlInputElement);
    try_cast!(HtmlSelectElement);
    try_cast!(HtmlTextAreaElement);

    panic!("unexpected input type. Only <input>, <select> and <textarea> are supported");
}

fn set_value_from_event(_event: impl Into<Event>, _value: &str) {
    let _event = _event.into();
    macro_rules! try_cast {
        ($typ:ty) => {
            if let Ok(elem) = _event
                .target()
                .expect("event doesn't have a target when calling extract_value")
                .dyn_into::<$typ>()
            {
                elem.set_value(_value);
                return;
            }
        };
    }

    use wasm_bindgen::JsCast;
    use web_sys::*;

    try_cast!(HtmlInputElement);
    try_cast!(HtmlSelectElement);
    try_cast!(HtmlTextAreaElement);

    panic!("unexpected input type. Only <input>, <select> and <textarea> are supported");
}

macro_rules! generic_type {
    ($name:ident<$ty:ty>{ |$oninput:ident| $html:expr, |$string:ident| $conversion_logic:expr, |$to_validate:tt| $validation_logic:expr $(,)?}) => {
        pub struct $name;
        impl InputType for $name {
            type T = $ty;

            fn input_component<F: Fn(InputResult<Self::T>) + 'static>(f: F) -> Html {
                let state = RefCell::new(None::<String>);
                let $oninput = move |e| {
                    let ($string, e) = get_value_from_event(e);
                    let parsed = {
                        let $string = $string.clone();
                        $conversion_logic
                    };
                    if let InputResult::Err = &parsed {
                        if let Some(state) = &*state.borrow() {
                            set_value_from_event(e, &state);
                        }
                    } else {
                        let mut state = state.borrow_mut();
                        *state = Some($string);
                        drop(state);
                        f(parsed);
                    }
                };
                $html
            }

            fn validate($to_validate: &Self::T) -> Result<(), ::std::rc::Rc<str>> {
                $validation_logic
            }
        }
    };
}

generic_type!(Number<i64> {
    |oninput| html! { <input type="number" oninput={oninput}/> },
    |string| parse_int(&string).into(),
    |_| Ok(()),
});
generic_type!(NumberPositive<u64> {
    |oninput| html! { <input type="number" min="0" oninput={oninput}/> },
    |string| parse_int(&string)
        .map_or_else(
            |err| Err(err).into(),
            |ok| {
                u64::try_from(ok).map_or_else(
                    |_| InputResult::Err,
                    |ok| InputResult::Ok(ok)
                )
            },
        )
        .into(),
    |_| Ok(()),
});
generic_type!(Money<i64> {
    |oninput| html! { <input type="number" oninput={oninput}/> },
    |string| parse_money(&string).into(),
    |_| Ok(()),
});
generic_type!(MoneyPositive<u64> {
    |oninput| html! { <input type="number" oninput={oninput}/> },
    |string| parse_money(&string)
        .map_or_else(
            |err| Err(err).into(),
            |ok| {
                u64::try_from(ok).map_or_else(
                    |_| InputResult::Err,
                    |ok| InputResult::Ok(ok)
                )
            },
        )
        .into(),
    |_| Ok(()),
});
generic_type!(Text<String> {
    |oninput| html! { <input oninput={oninput}/> },
    |string| InputResult::Ok(string),
    |_| Ok(())
});
generic_type!(TextMultiline<String> {
    |oninput| html! { <textarea oninput={oninput}/> },
    |string| InputResult::Ok(string),
    |_| Ok(()),
});

pub trait OptionEnum {
    fn display(&self) -> AttrValue;
}

#[derive(Properties, PartialEq)]
struct TranslationProps {
    children: AttrValue,
}

#[function_component]
fn T(translation_props: &TranslationProps) -> Html {
    let t = use_t();
    html!(<>{t(&translation_props.children)}</>)
}

// TODO: Into macro
pub struct Selector<T>(PhantomData<T>);

use crate as web_client;
impl<
        TEnum: OptionEnum
            + ::core::cmp::PartialEq
            + 'static
            + ::std::fmt::Debug
            + ::actix_surreal_starter_types::Entity
            + ::serde::de::DeserializeOwned,
    > web_client::form_input::generic_input::InputType for Selector<TEnum>
{
    type T = ::actix_surreal_starter_types::RecordId;

    fn input_component<
        F: Fn(web_client::form_input::input_result::InputResult<Self::T>) + 'static,
    >(
        f: F,
    ) -> Html {
        #[derive(Properties, PartialEq)]
        struct Props {
            f: Callback<
                web_client::form_input::input_result::InputResult<
                    ::actix_surreal_starter_types::RecordId,
                >,
            >,
        }
        #[function_component]
        fn C<
            TEnum: OptionEnum
                + PartialEq
                + 'static
                + std::fmt::Debug
                + actix_surreal_starter_types::Entity
                + DeserializeOwned,
        >(
            props: &Props,
        ) -> Html {
            let f = props.f.clone();
            let oninput = move |e: ::web_sys::Event| {
                f.emit(InputResult::Ok(
                    serde_json::from_str(
                        &e.target_dyn_into::<HtmlSelectElement>().unwrap().value(),
                    )
                    .unwrap(),
                ))
            };
            let items = use_state(|| vec![]);
            spawn_local({
                let items = items.clone();
                async move {
                    items.set(
                        global_storage::get_all::<TEnum>()
                            .await
                            .into_iter()
                            .map(|with_id| {
                                (
                                    AttrValue::from(serde_json::to_string(&with_id.id).unwrap()),
                                    with_id.data.display(),
                                )
                            })
                            .collect(),
                    );
                }
            });
            html! {
                <select onchange={oninput}>
                    { for (*items).iter().map(|i| html!(<option value={&i.0}><T>{&i.1}</T></option>)) }
                </select>
            }
        }
        let f = Callback::from(f);
        html! {
            <C<TEnum> f={f}/>
        }
    }

    fn validate(_: &Self::T) -> Result<(), Rc<str>> {
        Ok(())
    }
}

pub struct DateSpan;
impl InputType for DateSpan {
    type T = (DateTime<Utc>, DateTime<Utc>);

    fn input_component<F: Fn(InputResult<Self::T>) + 'static>(f: F) -> Html {
        fn today() -> DateTime<Utc> {
            Utc::now()
                .with_hour(0)
                .unwrap()
                .with_minute(0)
                .unwrap()
                .with_second(0)
                .unwrap()
        }
        fn from_string(string: &str) -> Option<DateTime<Utc>> {
            chrono::NaiveDate::parse_from_str(string, "%Y-%m-%d")
                .ok()
                .map(|date| DateTime::<Utc>::from_naive_utc_and_offset(date.into(), Utc))
        }
        fn to_string(date_time: DateTime<Utc>) -> AttrValue {
            date_time.date_naive().format("%Y-%m-%d").to_string().into()
        }
        let state = Rc::new(RefCell::new((today(), today())));
        let f = Rc::new(f);

        let oninput = move |input_event: InputEvent, index: usize| {
            if let Some(date) = from_string(&get_value_from_event(input_event).0) {
                let mut borrow = state.borrow_mut();
                match index {
                    1 => borrow.1 = date,
                    _ => borrow.0 = date,
                };
                drop(borrow);
                f(InputResult::Ok(state.borrow().clone()));
            }
        };
        let oninput_start = {
            let oninput = oninput.clone();
            move |e| oninput(e, 0)
        };
        let oninput_end = move |e| oninput(e, 0);
        html! {
            <div>
                <input oninput={move |e| oninput_start(e)} type="date"/>
                <input oninput={move |e| oninput_end(e)} type="date"/>
            </div>
        }
    }

    fn validate(value: &Self::T) -> Result<(), Rc<str>> {
        if value.0 == value.1 {
            return Err("D1EQD2".into());
        }
        if value.0 > value.1 {
            return Err("D1GTD2".into());
        }
        Ok(())
    }
}

pub struct SubInput<T>(PhantomData<T>);
impl<TEntity: PartialEq + Entity + HtmlInputs + 'static> InputType for SubInput<TEntity> {
    type T = RecordOf<TEntity>;

    fn input_component<F: Fn(InputResult<Self::T>) + 'static>(f: F) -> Html {
        let inputs = TEntity::html_inputs(
            Callback::from(move |entity: InputResult<TEntity>| {
                f(entity.map(|entity| RecordOf::Record(entity)))
            }),
            Callback::from(
                |_| {}, /* TODO: Figure out what to do with this error callback */
            ),
        );
        html! {
            <div>
                { inputs }
            </div>
        }
    }

    fn validate(_: &Self::T) -> Result<(), Rc<str>> {
        Ok(())
    }
}
