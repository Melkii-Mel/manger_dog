use proc_macro::TokenStream;

mod error_type;
mod error_enum;

#[proc_macro_attribute]
pub fn error_type(attr: TokenStream, item: TokenStream) -> TokenStream {
    error_type::error_type(attr, item)
}

#[proc_macro_derive(ErrorEnum)]
pub fn error_enum(attr: TokenStream) -> TokenStream {
    error_enum::error_enum(attr)
}