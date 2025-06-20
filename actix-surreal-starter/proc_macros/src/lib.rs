use proc_macro::TokenStream;

mod api_entities;
mod error_enum;
mod error_type;
mod utils;

#[proc_macro_attribute]
pub fn error_type(attr: TokenStream, item: TokenStream) -> TokenStream {
    error_type::error_type(attr, item)
}

#[proc_macro_derive(ErrorEnum, attributes(trait_crate))]
pub fn error_enum(attr: TokenStream) -> TokenStream {
    error_enum::error_enum(attr)
}

#[proc_macro]
pub fn api_entities(ts: TokenStream) -> TokenStream {
    api_entities::api_entities(ts)
}
