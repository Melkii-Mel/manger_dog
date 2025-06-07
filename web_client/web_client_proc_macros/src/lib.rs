use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Ident, Token, Type,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

macro_rules! check_and_parse {
    ($ps:ident $token:tt) => {
        if $ps.peek(Token![$token]) {
            $ps.parse::<Token![$token]>()?;
            true
        } else {
            false
        }
    };
}

struct InputField {
    idents: Vec<Ident>,
    ty: Type,
    optional: bool,
}

impl InputField {
    fn parse(ps: ParseStream) -> syn::Result<Self> {
        let mut idents = vec![];
        if ps.peek(syn::token::Paren) {
            let tuple_content;
            syn::parenthesized!(tuple_content in ps);
            while !tuple_content.is_empty() {
                idents.push(tuple_content.parse::<Ident>()?);
                check_and_parse!(tuple_content,);
            }
        } else {
            idents.push(ps.parse::<Ident>()?);
        }
        check_and_parse!(ps :);
        let ty = ps.parse::<Type>()?;
        let optional = check_and_parse!(ps?);
        check_and_parse!(ps,);
        Ok(Self {
            idents,
            ty,
            optional,
        })
    }
}

struct InputStruct {
    name: Ident,
    fields: Vec<InputField>,
}

impl InputStruct {
    fn parse(ps: ParseStream) -> syn::Result<Self> {
        let name = ps.parse::<Ident>()?;
        let content;
        syn::braced!(content in ps);
        let mut fields = Vec::new();
        while !content.is_empty() {
            fields.push(InputField::parse(&content)?);
        }
        check_and_parse!(ps,);
        Ok(Self { name, fields })
    }
}

struct InputsMacro {
    structs: Vec<InputStruct>,
}

impl Parse for InputsMacro {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut structs = Vec::new();
        while !input.is_empty() {
            structs.push(InputStruct::parse(input)?);
        }
        Ok(InputsMacro { structs })
    }
}

#[proc_macro]
pub fn forms(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as InputsMacro);

    let outputs = input.structs.into_iter().map(|input_struct| {
        let struct_name = &input_struct.name;
        let states = input_struct.fields.iter().map(|field| {
            let name = if field.idents.len() > 1 {
                format_ident!(
                    "_{}",
                    field
                        .idents
                        .iter()
                        .map(|ident| ident.to_string())
                        .collect::<Vec<_>>()
                        .join("_")
                )
            } else {
                field.idents[0].to_owned()
            };
            name
        }).collect::<Vec<Ident>>();
        let struct_construct = input_struct.fields.iter().enumerate().map(|(i, field)| {
            let optional = field.optional;
            let option = if optional {quote!(.option())} else {quote!()};
            let state_name = &states[i];
            if field.idents.len() == 1 {
                let ident = field.idents.get(0).unwrap();
                quote! { #ident: (*#state_name).clone()#option?, }
            }
            else {
                let idents = &field.idents;
                let indexes = (0usize..idents.len()).map(|index| {
                    let index = syn::Index::from(index); if optional {quote!(map(|v| v.#index))} else {quote!(#index)}
                });
                quote! {
                    #(
                        #idents: (*#state_name).clone()#option?.#indexes,
                    )*
                }
            }
        });
        let types = input_struct.fields.iter().map(|f| &f.ty).collect::<Vec<&Type>>();
        let errors = input_struct.fields.iter().map(|field| {
            let idents = &field.idents;
            quote! {
                'block: {
                    if let Some(ref errors) = errors {
                        #(
                            if let Some(first_error) = errors.#idents.get(0) {
                                break 'block Err(std::rc::Rc::from(first_error.as_dot_path().into_boxed_str()));
                            }
                        )*
                    }
                    Ok(())
                }
            }
        });
        let set = input_struct.fields.iter().enumerate().map(|(i, _)| {
            let ident = &states[i];
            quote! {
                let #ident = #ident.clone();
                move |v| {
                    #ident.set(v);
                }
            }
        });
        quote! {
            impl HtmlInputs for #struct_name {
                fn html_inputs(
                    on_entity_update: Callback<InputResult<Self>>,
                    on_errors_update: Callback<Option<Self::Error>>,
                ) -> Html
                where
                    Self: Sized,
                {
                    #[function_component]
                    fn Inputs(props: &HtmlInputsProps<#struct_name>) -> Html {
                    #(
                        let #states = use_state(|| InputResult::<<#types as InputType>::T>::Empty);
                    )*

                    let entity = 'block: {
                        #(
                            if matches!(&*#states, InputResult::Err) {
                                break 'block InputResult::Err
                            }
                        )*
                        #(
                            if matches!(&*#states, InputResult::Incomplete) {
                                break 'block InputResult::Incomplete
                            }
                        )*
                        (|| {
                            InputResult::Ok(#struct_name {
                                #(#struct_construct)*
                            })
                        })()
                    };
                    props.on_entity_update.emit(entity.clone());
                    let errors = Option::<#struct_name>::from(entity)
                        .as_ref()
                        .map(|e| ::actix_surreal_starter_types::Entity::validate(e).err())
                        .flatten();
                    props.on_errors_update.emit(errors.clone());
                    html!(
                        <div>
                        #(
                            <generic_input::GenericInput<#types>
                                label="todo"
                                oninput={#set}
                                error={#errors}
                            />
                        )*
                        </div>
                    )
                }
                    html! {
                        <Inputs on_entity_update={on_entity_update} on_errors_update={on_errors_update}/>
                    }
                }
            }
        }
    });

    quote! { #( #outputs )* }.into()
}
