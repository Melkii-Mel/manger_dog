use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemEnum, Meta, Path};

pub fn error_type(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemEnum);
    let enum_name = &input.ident;
    let default_status_code = parse_macro_input!(attr as Path);
    let mut match_arms = Vec::new();

    for variant in &mut input.variants {
        let variant_name = &variant.ident;
        let mut status_code: Option<Path> = None;

        variant.attrs.retain(|attr| {
            if let Ok(Meta::Path(meta_path)) = attr.parse_meta() {
                status_code = Some(meta_path.clone());
            } else {
                panic!("Invalid attribute. Path expected.");
            }
            false
        });
        let status_code = status_code.unwrap_or_else(|| default_status_code.clone());
        match_arms.push(quote! {
            #enum_name::#variant_name => StatusCode::#status_code,
        });
    }

    quote! {
        #[allow(unused_qualifications)]
        #[derive(::std::fmt::Debug, ::serde::Serialize)]
        #input

        impl GetStatusCode for #enum_name {
            fn status_code(&self) -> ::actix_web::http::StatusCode {
                match self {
                    #(#match_arms)*
                }
            }
        }
    }
    .into()
}
