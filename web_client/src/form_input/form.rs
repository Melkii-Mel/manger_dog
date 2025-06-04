use crate::form_input::generic_input;
use crate::form_input::generic_input::InputType;
use crate::form_input::input_result::InputResult;
use crate::form_input::inputs::DateSpan;
use crate::form_input::inputs::MoneyPositive;
use crate::form_input::inputs::Selector;
use crate::form_input::inputs::{SubInput, Text};
use crate::global_storage;
use crate::request::Request;
use actix_surreal_starter_types::{Entity, ErrorEnum};
use actix_surreal_starter_types::{RecordId, WithId};
use entities::Currency;
use entities::{ApiValidationError, FinancialGoal, Metadata};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use yew::platform::spawn_local;
use yew::Html;
use yew::{function_component, html, use_state, Callback, Properties};

// forms! {
//     FinancialGoal {
//         currency_id: Selector<Currency>,
//         (start_date, end_date): DateSpan,
//         target_income: MoneyPositive,
//         metadata_id: SubInput<Metadata>,
//     }
//     Metadata {
//         title: Text?,
//         description: TextMultiline?,
//     }
// }

#[derive(Properties, PartialEq)]
struct HtmlInputsProps<T: Entity + PartialEq>
where
    <T as Entity>::Error: PartialEq,
{
    on_entity_update: Callback<InputResult<T>>,
    on_errors_update: Callback<Option<T::Error>>,
}
pub trait HtmlInputs: Entity {
    fn html_inputs(
        on_entity_update: Callback<InputResult<Self>>,
        on_errors_update: Callback<Option<Self::Error>>,
    ) -> Html
    where
        Self: Sized;
}

pub enum Mode {
    Insert,
    Update(RecordId),
}

impl HtmlInputs for FinancialGoal {
    fn html_inputs(
        on_entity_update: Callback<InputResult<Self>>,
        on_errors_update: Callback<Option<Self::Error>>,
    ) -> Html
    where
        Self: Sized,
    {
        #[function_component]
        fn Inputs(props: &HtmlInputsProps<FinancialGoal>) -> Html {
            // TODO: Add optional fields support
            let currency_id =
                use_state(|| InputResult::<<Selector<Currency> as InputType>::T>::Empty);
            let __start_date_end_date =
                use_state(|| InputResult::<<DateSpan as InputType>::T>::Empty);
            let target_income = use_state(|| InputResult::<<MoneyPositive as InputType>::T>::Empty);
            let metadata_id =
                use_state(|| InputResult::<<SubInput<Metadata> as InputType>::T>::Empty);
            let entity = (|| {
                InputResult::Ok(FinancialGoal {
                    currency_id: (*currency_id).clone()?,
                    start_date: (*__start_date_end_date).clone()?.0,
                    end_date: (*__start_date_end_date).clone()?.1,
                    target_income: (*target_income).clone()?,
                    metadata_id: (*metadata_id).clone()?,
                })
            })();
            props.on_entity_update.emit(entity.clone());
            let errors = Option::<FinancialGoal>::from(entity)
                .as_ref()
                .map(|e| actix_surreal_starter_types::Entity::validate(e).err())
                .flatten();
            props.on_errors_update.emit(errors.clone());

            macro_rules! error {
                ($($ident:ident),*) => {
                    'block: {
                        if let Some(ref errors) = errors {
                            $(
                                if let Some(first_error) = errors.$ident.get(0) {
                                    break 'block Err(std::rc::Rc::from(first_error.as_dot_path().into_boxed_str()));
                                }
                            )*
                        }
                        Ok(())
                    }
                };
            }

            macro_rules! set {
                ($ident:ident) => {{
                    let $ident = $ident.clone();
                    move |v| {
                        $ident.set(v);
                    }
                }};
            }

            html!(
                <div>
                    <generic_input::GenericInput<Selector<Currency>>
                        label="todo"
                        oninput={set!(currency_id)}
                        error={error!(currency_id)}
                    />
                    <generic_input::GenericInput<DateSpan>
                        label="todo"
                        oninput={set!(__start_date_end_date)}
                        error={error!(start_date, end_date)}
                    />
                    <generic_input::GenericInput<MoneyPositive>
                        label="todo"
                        oninput={set!(target_income)}
                        error={error!(target_income)}
                    />
                    <generic_input::GenericInput<SubInput<Metadata>>
                        label="todo"
                        oninput={set!(metadata_id)}
                        error={error!(metadata_id)}
                    />
                </div>
            )
        }

        html! {
            <Inputs on_entity_update={on_entity_update} on_errors_update={on_errors_update}/>
        }
    }
}

impl HtmlInputs for Metadata {
    fn html_inputs(
        on_entity_update: Callback<InputResult<Self>>,
        on_errors_update: Callback<Option<Self::Error>>,
    ) -> Html
    where
        Self: Sized,
    {
        #[function_component]
        fn Inputs(props: &HtmlInputsProps<Metadata>) -> Html {
            // TODO: Add optional fields support
            let title = use_state(|| InputResult::<<Text as InputType>::T>::Empty);
            let description = use_state(|| InputResult::<<Text as InputType>::T>::Empty);
            let entity = (|| {
                InputResult::Ok(Metadata {
                    title: (*title).clone().option()?,
                    description: (*description).clone().option()?,
                })
            })();
            props.on_entity_update.emit(entity.clone());
            let errors = entity
                .ok()
                .as_ref()
                .map(|e| actix_surreal_starter_types::Entity::validate(e).err())
                .flatten();
            props.on_errors_update.emit(errors.clone());

            macro_rules! error {
                ($($ident:ident),*) => {
                    'block: {
                        if let Some(ref errors) = errors {
                            $(
                                if let Some(first_error) = errors.$ident.get(0) {
                                    break 'block Err(std::rc::Rc::from(first_error.as_dot_path().into_boxed_str()));
                                }
                            )*
                        }
                        Ok(())
                    }
                };
            }

            macro_rules! set {
                ($ident:ident) => {{
                    let $ident = $ident.clone();
                    move |v| {
                        $ident.set(v);
                    }
                }};
            }

            html!(
                <div>
                    <generic_input::GenericInput<Text>
                        label="todo"
                        oninput={set!(title)}
                        error={error!(title)}
                    />
                    <generic_input::GenericInput<Text>
                        label="todo"
                        oninput={set!(description)}
                        error={error!(description)}
                    />
                </div>
            )
        }

        html! {
            <Inputs on_entity_update={on_entity_update} on_errors_update={on_errors_update}/>
        }
    }
}

#[function_component]
pub fn GenericForm<
    T: Entity + Serialize + DeserializeOwned + Debug + 'static + Clone + HtmlInputs,
>() -> yew::Html
where
    <T as Entity>::Error: DeserializeOwned + Debug,
{
    // TODO: Add mode switching
    let mode = use_state(|| Mode::Insert);
    let entity = use_state(|| InputResult::Empty::<T>);
    let errors = use_state(|| None::<T::Error>);
    let on_submit = {
        let mode = mode.clone();
        let entity = entity.clone();
        move |_: web_sys::MouseEvent| {
            let mode = mode.clone();
            let entity = entity.clone();
            match &*mode {
                Mode::Insert => {
                    if let InputResult::Ok(entity) = &*entity {
                        let entity = entity.clone();
                        spawn_local(async move {
                            let result: Option<Result<RecordId, T::Error>> =
                                Request::post(T::api_location().to_string(), &entity).await;
                            let result = result.unwrap().unwrap();
                            global_storage::set(WithId {
                                id: result,
                                data: entity,
                            })
                            .await;
                        });
                    }
                }
                Mode::Update(id) => {
                    let id = id.clone();
                    if let InputResult::Ok(entity) = &*entity {
                        let record = WithId {
                            id: id.clone(),
                            data: entity.clone(),
                        };
                        spawn_local(async move {
                            let _: Option<()> =
                                Request::put(T::api_location().to_string(), &record).await;
                            global_storage::set(record).await;
                        });
                    }
                }
            }
        }
    };
    let on_entity_update = Callback::from({
        let entity = entity.clone();
        move |e: InputResult<T>| {
            entity.set(e);
        }
    });
    let on_errors_update = Callback::from({
        let errors = errors.clone();
        move |e: Option<T::Error>| {
            errors.set(e);
        }
    });
    html! {
        <form>
            { T::html_inputs(on_entity_update, on_errors_update) }
            <button disabled={errors.is_some()} type="submit" onclick={on_submit}/>
        </form>
    }
}
