use crate::form_input::inputs::TextMultiline;
use web_client_proc_macros::forms;
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
use entities::{FinancialGoal, Metadata};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use yew::platform::spawn_local;
use yew::Html;
use yew::{function_component, html, use_state, Callback, Properties};

forms! {
    FinancialGoal {
        currency_id: Selector<Currency>,
        (start_date, end_date): DateSpan,
        target_income: MoneyPositive,
        metadata_id: SubInput<Metadata>,
    }
    Metadata {
        title: Text?,
        description: TextMultiline?,
    }
}

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
