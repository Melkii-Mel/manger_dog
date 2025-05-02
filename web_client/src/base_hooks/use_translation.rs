use i18nrs::yew::use_translation;
use yew::hook;

#[hook]
pub fn use_t() -> Box<dyn Fn(&str) -> String> {
    let t = use_translation().0;
    Box::new(move |key: &str| t.t(key))
}
