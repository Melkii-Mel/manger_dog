// use crate::use_t;
// use entities::FinancialGoal;
// use yew::{function_component, html, Html};
//
// #[function_component]
// pub fn FinancialGoals() -> Html {
//     html! {
//         <div>
//             <Header<FinancialGoal>/>
//             <Entities<FinancialGoal>/>
//             <Input<FinancialGoal>/>
//         </div>
//     }
// }
//
// pub trait Title {
//     fn title() -> &'static str;
// }
//
// pub trait EntityView {}
// pub trait EntityForm {
//     fn to_form(&self) -> Html;
//     fn from_form(html: Html) -> Self;
// }
//
// impl EntityView for FinancialGoal {}
// impl EntityForm for FinancialGoal {}
// impl Title for FinancialGoal {
//     fn title() -> &'static str {
//         "financial goal"
//     }
// }
//
// #[function_component]
// pub fn Header<T: Title>() -> Html {
//     let t = use_t();
//     html! {
//         <h1>{t(T::title())}</h1>
//     }
// }
//
// #[function_component]
// pub fn Entities<T: EntityView>() -> Html {
//     html! {
//
//     }
// }
//
// /// inputs! {
// ///     FinancialGoal {
// ///         currency_id: Selector<Currency>,
// ///         (start_date, end_date): DateTimeSpan,
// ///         target_income: MoneyPositive,
// ///         metadata_id: SubInput<Metadata>
// ///     },
// ///     ...
// /// }
// ///
// #[function_component]
// pub fn Input<T: EntityForm>() -> Html {
//     html! {
//
//     }
// }
