use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

pub fn error_enum(attr: TokenStream) -> TokenStream {
    let input = parse_macro_input!(attr as DeriveInput);
    let crate_path = input.attrs.iter().find_map(|a| {
        if a.path.is_ident("trait_crate") {
            a.parse_meta().ok().and_then(|meta| {
                if let syn::Meta::NameValue(value) = meta {
                    if let syn::Lit::Str(str) = value.lit {
                        return Some(str.value());
                    }
                }
                None
            })
        } else {
            None
        }
    }).unwrap_or("::actix_surreal_starter_types".to_string());
    let crate_path: syn::Path = syn::parse_str(&crate_path).expect("Invalid trait crate path");
    let enum_ident = input.ident;
    let variants = match input.data {
        Data::Enum(data) => data.variants,
        _ => panic!("Only works with enums"),
    };
    let arms = variants.iter().map(|variant| {
        let arm_ident = &variant.ident;
        let arm_name = arm_ident.to_string();
        match &variant.fields {
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                quote! {
                    #enum_ident::#arm_ident(v) => format!("{}.{}", #arm_name, v.as_dot_path()),
                }
            }
            Fields::Unit => {
                quote! {
                    #enum_ident::#arm_ident => #arm_name.to_string(),
                }
            }
            _ => {
                panic!("only unnamed field or unit arms are supported");
            }
        }
    });

    let expanded = quote! {
        impl #crate_path::ErrorEnum for #enum_ident {
            fn as_dot_path(&self) -> String {
                match self {
                    #(#arms)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
