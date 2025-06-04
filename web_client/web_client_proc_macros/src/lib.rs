use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Ident, Token, Type,
};

struct InputField {
    idents: Vec<Ident>,
    ty: Type,
}

struct InputStruct {
    name: Ident,
    fields: Vec<InputField>,
}

struct InputsMacro {
    structs: Vec<InputStruct>,
}

impl Parse for InputsMacro {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut structs = Vec::new();

        while !input.is_empty() {
            let name = input.parse::<Ident>()?;
            let content;
            syn::braced!(content in input);
            let mut fields = Vec::new();

            // Parse fields within braces
            while !content.is_empty() {
                if content.peek(syn::token::Paren) {
                    // Handle tuple field: (ident1, ident2): Type
                    let tuple_content;
                    syn::parenthesized!(tuple_content in content);
                    let mut idents = Vec::new();
                    while !tuple_content.is_empty() {
                        idents.push(tuple_content.parse::<Ident>()?);
                        if tuple_content.peek(Token![,]) {
                            tuple_content.parse::<Token![,]>()?;
                        }
                    }
                    content.parse::<Token![:]>()?;
                    let ty = content.parse::<Type>()?;
                    fields.push(InputField { idents, ty });
                } else {
                    // Handle a single field: ident: Type
                    let ident = content.parse::<Ident>()?;
                    content.parse::<Token![:]>()?;
                    let ty = content.parse::<Type>()?;
                    fields.push(InputField {
                        idents: vec![ident],
                        ty,
                    });
                }
                if content.peek(Token![,]) {
                    content.parse::<Token![,]>()?;
                }
            }
            structs.push(InputStruct { name, fields });

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(InputsMacro { structs })
    }
}

#[proc_macro]
pub fn forms(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as InputsMacro);

    let outputs = input.structs.into_iter().map(|input_struct| {

        let fields = input_struct.fields.into_iter().map(|field| {
            let ty = field.ty;
            if field.idents.len() > 1 {
                let field_name = field.idents.into_iter().reduce(move |ident_a, ident_b| {
                    format_ident!("{}_{}", ident_a, ident_b)
                }).unwrap();
                quote! { #field_name: #ty }
            } else {
                let ident = &field.idents[0];
                quote! { #ident: #ty }
            }
        });
        
        let struct_ident = format_ident!("{}FormEntity", input_struct.name);
        
        quote! {
            pub struct #struct_ident {
                #( #fields ),*
            }
        }
    });

    quote! { #( #outputs )* }.into()
}