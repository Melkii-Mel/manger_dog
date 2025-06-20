use crate::api_entities::syntax_model::DtoField;
use crate::api_entities::syntax_model::Entities;
use crate::api_entities::syntax_model::Entity;
use crate::api_entities::syntax_model::Field;
use crate::api_entities::syntax_model::Relationship;
use crate::api_entities::syntax_model::Validator;
use heck::ToSnakeCase;
use proc_macro_utils::force_parse_token;
use proc_macro_utils::parse_brackets;
use proc_macro_utils::try_parse_token;
use quote::format_ident;
use syn::parse::{Parse, ParseStream};
use syn::token::{Bracket, Paren};
use syn::{Ident, LitStr};

impl Parse for Entities {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let validator = parse_static_key_kvp(input, "validator")?;
        let error = parse_static_key_kvp(input, "error")?;
        let mut entities = Vec::new();
        while !input.is_empty() {
            entities.push(Entity::parse(input)?);
            try_parse_token!(input,);
        }
        Ok(Entities {
            validator,
            error,
            entities,
        })
    }
}

impl Parse for Entity {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        let dto_ident = format_ident!("Dto{ident}");
        let error_ident = format_ident!("{ident}Error");
        let db_table_name = LitStr::new(&ident.to_string().to_snake_case(), input.span());
        let paths_to_ownership = parse_str_list(&parse_brackets!(bracketed(input)))?;
        let input = &parse_brackets!(braced(input));
        let mut fields = Vec::new();
        let mut dto_fields = Vec::new();
        while !input.is_empty() {
            if input.peek(Bracket) {
                dto_fields.push(DtoField::parse(input, &ident)?);
            } else {
                fields.push(Field::parse(input)?);
            }
            force_parse_token!(input,)?;
        }
        Ok(Entity {
            ident,
            dto_ident,
            error_ident,
            db_table_name,
            paths_to_ownership,
            fields,
            dto_fields,
        })
    }
}

impl Parse for Field {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        force_parse_token!(input :)?;
        let ty = input.parse()?;
        let mut validators = Vec::new();
        if input.peek(Bracket) {
            let content = &parse_brackets!(bracketed(input));
            while !content.is_empty() {
                validators.push(Validator::parse(content)?);
                try_parse_token!(content,);
            }
        }

        Ok(Self {
            ident,
            ty,
            validators,
        })
    }
}

impl Parse for Validator {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        let mut fields = Vec::new();
        if input.peek(Paren) {
            let content = parse_brackets!(parenthesized(input));
            while !content.is_empty() {
                fields.push(content.parse()?);
                try_parse_token!(content,);
            }
        }
        Ok(Validator { ident, fields })
    }
}

impl DtoField {
    fn parse(input: ParseStream, struct_ident: &Ident) -> syn::Result<Self> {
        let content = &parse_brackets!(bracketed(input));
        let (relationship, order) = match content.parse::<Ident>()?.to_string().as_str() {
            "mtm" => (Relationship::MTM, true),
            "mtmi" => (Relationship::MTM, false),
            "otm" => (Relationship::OTM, true),
            "otmi" => (Relationship::OTM, false),
            _ => Err(syn::Error::new(
                content.span(),
                "Expected one of relationship identifiers: `mtm`, `mtmi`, `otm`, `otmi`",
            ))?,
        };
        let content = &parse_brackets!(parenthesized(content));
        let foreign_ident = content.parse()?;
        let junction_ident = if order {
            format_ident!("{struct_ident}{foreign_ident}")
        } else {
            format_ident!("{foreign_ident}{struct_ident}")
        };
        Ok(Self {
            relationship,
            foreign_ident,
            junction_ident,
        })
    }
}

fn parse_ident(input: ParseStream, ident: &'static str) -> syn::Result<()> {
    let parsed_ident: Ident = input.parse()?;
    if parsed_ident != ident {
        Err(syn::Error::new(
            parsed_ident.span(),
            format!("Expected `{ident}:`"),
        ))
    } else {
        Ok(())
    }
}

fn parse_static_key_kvp<T: Parse>(input: ParseStream, key: &'static str) -> syn::Result<T> {
    parse_ident(input, key)?;
    try_parse_token!(input :);
    let value: T = input.parse()?;
    try_parse_token!(input,);
    Ok(value)
}

fn parse_str_list(parse_stream: ParseStream) -> syn::Result<Vec<LitStr>> {
    let mut vec = Vec::new();
    try_parse_token!(parse_stream,);
    while !parse_stream.is_empty() {
        vec.push(parse_stream.parse()?);
        try_parse_token!(parse_stream,);
    }
    Ok(vec)
}
