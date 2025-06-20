use crate::api_entities::syntax_model::Entities;
use proc_macro::TokenStream;
use syn::parse_macro_input;

mod codegen;
mod parsing;
mod syntax_model;

pub fn api_entities(ts: TokenStream) -> TokenStream {
    codegen::gen(parse_macro_input!(ts as Entities)).unwrap_or_else(|e| e.to_compile_error().into())
}
