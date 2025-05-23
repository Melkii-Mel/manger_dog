use proc_macro::TokenStream;

mod error_type;

#[proc_macro_attribute]
pub fn error_type(attr: TokenStream, item: TokenStream) -> TokenStream {
    error_type::error_type(attr, item)
}
