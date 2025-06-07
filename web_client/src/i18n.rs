use i18nrs::yew::I18nProviderConfig;
use std::collections::HashMap;
use yew::Callback;

pub type TranslationsMap = HashMap<&'static str, &'static str>;

pub fn translation_config(translations_string: &str, default_language: &str) -> I18nProviderConfig {
    I18nProviderConfig {
        translations: string_to_map(translations_string),
        default_language: default_language.to_string(),
        onerror: Callback::from(|e| {}),
        ..I18nProviderConfig::default()
    }
}

fn string_to_map(string: &str) -> TranslationsMap {
    let hash_map = serde_json::from_str::<HashMap<String, serde_json::Value>>(string).unwrap();
    hash_map
        .iter()
        .map(|(k, v)| {
            let k: &'static str = Box::leak(k.clone().into_boxed_str());
            let v: &'static str = Box::leak(serde_json::to_string(v).unwrap().into_boxed_str());
            (k, v)
        })
        .collect()
}
