use crate::api_entities::syntax_model::Entity;
use crate::api_entities::syntax_model::Field;
use crate::api_entities::syntax_model::{DtoField, Entities};
use heck::ToSnakeCase;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{GenericArgument, LitStr};
use syn::Path;
use syn::PathArguments;
use syn::Type;
use syn::TypePath;

pub fn gen(entities: Entities) -> syn::Result<proc_macro::TokenStream> {
    let endpoints = gen_endpoints(&entities.entities);
    let structs = gen_structs(&entities.entities)?;
    Ok(quote! {
        #endpoints
        #structs
    }
    .into())
}

fn gen_endpoints(entities: &Vec<Entity>) -> TokenStream {
    let config_blocks = entities.iter().map(|Entity { ident, db_table_name, .. }| {
        quote! {
            .route(concat!("/api/", #db_table_name, "/all"), actix_web::web::get().to(
                |user_id: actix_surreal_starter::UserId| async move {
                    actix_surreal_starter::crud_ops::select_all::<#ident>(user_id.0.into(), #ident::query_builder()).await.map(actix_web::web::Json)
                }
            ))
            .route(concat!("/api/", #db_table_name), actix_web::web::get().to(
                |id: actix_web::web::Json<::surrealdb::RecordId>, user_id: actix_surreal_starter::UserId| async move {
                    actix_surreal_starter::crud_ops::select::<#ident>(id.0.into(), user_id.0.into(), #ident::query_builder()).await.map(actix_web::web::Json)
                }
            ))
            .route(concat!("/api/", #db_table_name), actix_web::web::post().to(
                |mut entity: actix_web::web::Json<#ident>, user_id: actix_surreal_starter::UserId| async move {
                    let result = if let Err(e) = ::actix_surreal_starter::Entity::validate(&entity.0) {
                        Err(e)
                    }
                    else {
                        Ok(entity.0.insert(user_id).await?)
                    };
                    Ok::<_, ::actix_surreal_starter::crud_ops::CrudError>(::actix_web::HttpResponse::Ok().json(result))
                }
            ))
            .route(concat!("/api/", #db_table_name), actix_web::web::put().to(
                |entity: actix_web::web::Json<actix_surreal_starter_types::WithId<#ident>>, user_id: actix_surreal_starter::UserId| async move {
                    let result = if let Err(e) = ::actix_surreal_starter::Entity::validate(&entity.0.data) {
                        Err(e)
                    }
                    else {
                        Ok(#ident::update(entity.0, user_id).await?)
                    };
                    Ok::<_, ::actix_surreal_starter::crud_ops::CrudError>(::actix_web::HttpResponse::Ok().json(result))
                }
            ))
            .route(concat!("/api/", #db_table_name), actix_web::web::delete().to(
                |id: actix_web::web::Json<::surrealdb::RecordId>, user_id: actix_surreal_starter::UserId| async move {
                    actix_surreal_starter::crud_ops::delete(id.0.into(), user_id.0.into(), #ident::query_builder()).await
                }
            ))
        }
    });
    quote! {
        #[cfg(feature = "server")]
        pub fn configure_endpoints(cfg: &mut actix_web::web::ServiceConfig) {
            cfg
            #(#config_blocks)*;
        }
    }
}

fn gen_structs(entities: &Vec<Entity>) -> syn::Result<TokenStream> {
    let entities = entities.iter().map(|Entity { ident, error_ident, db_table_name, paths_to_ownership, fields, dto_fields, dto_ident }| {
        let dto = {
            let fields = gen_dto_fields(dto_fields);
            quote! {
                #[derive(std::fmt::Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq)]
                pub struct #dto_ident {
                    #[serde(flatten)]
                    value: #ident,
                    #fields
                }
            }
        };
        let field_declarations = fields.iter().map(|Field { ident, ty, .. }| {
            quote! {
                pub #ident: #ty,
            }
        });
        let error_field_declarations = fields.iter().map(|Field { ident, ty, .. }| {
            let error_type = match extract_inner_record_of_type(ty) {
                None => {
                    quote! {Vec<#error_ident>}
                }
                Some(ty) => {
                    let ident = extract_ident_from_type(ty)?;
                    let ty_err = format_ident!("{ident}Error");
                    quote! {Box<Result<(), #ty_err>>}
                }
            };
            Ok::<_, syn::Error>(quote! {
                pub #ident: #error_type,
            })
        }).collect::<Result<Vec<_>, _>>()?;
        let fkey_path_map = paths_to_ownership.iter().map(|lit| {
            let value = lit.value();
            let fkey = value.split_once('.').map(|(before, _)| before).unwrap_or(&value);
            let fkey_lit = LitStr::new(fkey, Span::call_site());
            quote! {
                #fkey_lit => #lit,
            }
        });
        let insert_child_records = fields.iter().filter_map(|field| {
            extract_inner_record_of_type(&field.ty)?;
            let ident = &field.ident;
            Some(quote! {
                if let ::actix_surreal_starter_types::RecordOf::Record(replacing) = self.#ident {
                    let replaced = replacing.replace_with_ids(value[stringify!(#ident)].take());
                    self.#ident = actix_surreal_starter_types::RecordOf::Id(replaced?);
                }
            })
        });
        Ok::<_, syn::Error>(quote! {
            #[derive(std::fmt::Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq)]
            pub struct #ident {
                #(
                    #field_declarations
                )*
            }
            #dto
            #[derive(std::fmt::Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq)]
            pub struct #error_ident {
                #(
                    #error_field_declarations
                )*
            }
            impl #ident {
                fn paths() -> &'static [&'static str] {
                    &[#(#paths_to_ownership),*]
                }
                #[cfg(feature = "server")]
                fn query_builder() -> actix_surreal_starter::query_builder::QueryBuilder {
                    use phf::phf_map;
                    static fkey_path_map: phf::Map<&'static str, &'static str> = phf_map! {
                        #(
                            #fkey_path_map
                        )*
                    };
                    actix_surreal_starter::query_builder::QueryBuilder {
                        paths: Self::paths(),
                        table_name: #db_table_name,
                        fkey_path_map: Some(fkey_path_map),
                    }
                }
                // TODO: Consider removing from the macro and implementing via trait default implementation
                #[cfg(feature = "server")]
                pub async fn insert(
                    mut self,
                    user_id: actix_surreal_starter::UserId,
                ) -> Result<::serde_json::Value, ::actix_surreal_starter::crud_ops::CrudError> {
                    let mut result;
                    (self, result) = self.insert_children(user_id.clone()).await?;
                    let id =
                        ::actix_surreal_starter::crud_ops::insert(self, user_id.0.into(), #ident::query_builder(), #ident::paths()[0] == "user_id")
                            .await?;
                    result.insert("id".to_string(), ::serde_json::to_value(id)?);
                    Ok(serde_json::Value::Object(result))
                }
                // TODO: Consider removing from the macro and implementing via trait default implementation
                #[cfg(feature = "server")]
                pub async fn update(with_id: actix_surreal_starter_types::WithId<#ident>, user_id: actix_surreal_starter::UserId) -> Result<::serde_json::Value, ::actix_surreal_starter::crud_ops::CrudError> {
                    let mut data = with_id.data;
                    let id = with_id.id;
                    let mut result;
                    (data, result) = data.insert_children(user_id.clone()).await?;
                    actix_surreal_starter::crud_ops::update(id, data, user_id.0.into(), #ident::query_builder()).await?;
                    Ok(serde_json::Value::Object(result))
                }
                // TODO: Consider removing from the macro and implementing via trait default implementation
                #[cfg(feature = "server")]
                async fn insert_children(mut self, user_id: actix_surreal_starter::UserId) -> Result<(Self, ::serde_json::map::Map<String, ::serde_json::Value>), ::actix_surreal_starter::crud_ops::CrudError> {
                    let mut result = ::serde_json::Map::new();
                    #(
                        #insert_child_records
                    )*
                    Ok((self, result))
                }
            }
        })
    }).collect::<Result<Vec<_>, _>>()?;
    Ok(quote! {
        #(#entities)*
    })
}

fn gen_dto_fields(dto_fields: &Vec<DtoField>) -> TokenStream {
    let fields = dto_fields.iter().map(
        |DtoField {
             relationship,
             foreign_ident,
             ..
         }| {
            let field_name = proc_macro2::Ident::new(
                &format!(
                    "{}_{}",
                    foreign_ident.to_string().to_snake_case(),
                    relationship.as_str()
                ),
                Span::call_site(),
            );
            let foreign_ident = foreign_ident.clone();
            quote! {
                #[serde(default)]
                #field_name: Vec<#foreign_ident>,
            }
        },
    );
    quote! {
        #(#fields)*
    }
}

fn extract_inner_record_of_type(ty: &Type) -> Option<&Type> {
    match ty {
        Type::Path(type_path) => {
            if let Some(last_segment) = type_path.path.segments.last() {
                match last_segment.ident.to_string().as_str() {
                    "RecordOf" => {
                        if let PathArguments::AngleBracketed(args) = &last_segment.arguments {
                            for arg in &args.args {
                                if let GenericArgument::Type(inner_ty) = arg {
                                    return Some(inner_ty);
                                }
                            }
                        }
                    }
                    "Option" => {
                        // Recurse into Option<T>
                        if let PathArguments::AngleBracketed(args) = &last_segment.arguments {
                            for arg in &args.args {
                                if let GenericArgument::Type(inner_ty) = arg {
                                    return extract_inner_record_of_type(inner_ty);
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
    None
}

fn extract_ident_from_type(ty: &Type) -> syn::Result<&syn::Ident> {
    match ty {
        Type::Path(TypePath {
            path: Path { segments, .. },
            ..
        }) => segments.last().map(|seg| &seg.ident).ok_or_else(|| {
            syn::Error::new_spanned(ty, "Expected a path with at least one segment")
        }),
        _ => Err(syn::Error::new_spanned(
            ty,
            "Only path types (like `MyType`) are supported",
        )),
    }
}
