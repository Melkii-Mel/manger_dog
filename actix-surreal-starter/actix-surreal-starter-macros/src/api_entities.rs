#[macro_export]
macro_rules! api_entities {
    (
        validator: $validator_type:ident,
        error: $validation_error_type:ident,
        $(
            $name:ident|$name_error:ident( $db_table_name:literal $( [ $( $path_to_ownership:literal ),* ] )? )
            {
                $(
                    $field:ident: $type:ty $( [ $( $validator:ident $( ( $( $validation_field:ident ),*$(,)? ) )? ),* $(,)? ] )?
                ),*$(,)*
            }
        )*
    ) => {

        pub static PATHS:phf::Map<&'static str, &'static [&'static str]> = phf::phf_map! {
            $($db_table_name => &[$($($path_to_ownership),*)?] as &[&str],)*
        };

        #[cfg(feature = "server")]
        pub fn configure_endpoints(cfg: &mut actix_web::web::ServiceConfig) {
            cfg
            $(
            .route(concat!("/api/", $db_table_name, "/all"), actix_web::web::get().to(
                |user_id: actix_surreal_starter::UserId| async move {
                    actix_surreal_starter::crud_ops::select_all::<$name>(user_id.0, $name::query_builder()).await.map(actix_web::web::Json)
                }
            ))
            .route(concat!("/api/", $db_table_name), actix_web::web::get().to(
                |id: actix_web::web::Json<::surrealdb::RecordId>, user_id: actix_surreal_starter::UserId| async move {
                    actix_surreal_starter::crud_ops::select::<$name>(id.0, user_id.0, $name::query_builder()).await.map(actix_web::web::Json)
                }
            ))
            .route(concat!("/api/", $db_table_name), actix_web::web::post().to(
                |entity: actix_web::web::Json<$name>, user_id: actix_surreal_starter::UserId| async move {
                    Ok::<_, ::actix_surreal_starter::crud_ops::CrudError>(::actix_web::HttpResponse::Ok().json(actix_surreal_starter::crud_ops::insert(entity.0, user_id.0,$name::query_builder()).await?))
                }
            ))
            .route(concat!("/api/", $db_table_name), actix_web::web::put().to(
                |entity: actix_web::web::Json<actix_surreal_starter_types::WithId<serde_json::Value>>, user_id: actix_surreal_starter::UserId| async move {
                    actix_surreal_starter::crud_ops::update(entity.0.id, entity.0.data, user_id.0, $name::query_builder()).await
                }
            ))
            .route(concat!("/api/", $db_table_name), actix_web::web::delete().to(
                |id: actix_web::web::Json<surrealdb::RecordId>, user_id: actix_surreal_starter::UserId| async move {
                    actix_surreal_starter::crud_ops::delete(id.0, user_id.0, $name::query_builder()).await
                }
            ))
            )*;
        }

        $(
        #[derive(std::fmt::Debug, serde::Deserialize, serde::Serialize, Clone)]
        pub struct $name {
            $(pub $field: api_entities!(@parse_type $type)),*
        }
        #[derive(std::fmt::Debug, serde::Deserialize, serde::Serialize, Clone)]
        pub struct $name_error {
            $(pub $field: api_entities!(@parse_error_type $validation_error_type, $type)),*
        }

        impl $name {
            fn paths() -> &'static [&'static str] {
                PATHS.get($db_table_name).unwrap()
            }
            #[cfg(feature = "server")]
            fn query_builder() -> actix_surreal_starter::query_builder::QueryBuilder {
                actix_surreal_starter::query_builder::QueryBuilder {
                    paths: Self::paths(),
                    table_name: $db_table_name,
                    fkey_path_map: None, //TODO: nah oh it can't be None it's just a placeholder
                }
            }
        }
        impl ::actix_surreal_starter_types::Entity<$name_error> for $name {
            fn table_name() -> &'static str {
                $db_table_name
            }
            fn api_location() -> &'static str {
                concat!("/api/", $db_table_name)
            }
            fn validate(&self) -> Result<(), $name_error> {
                let mut erronous = false;
                let mut result = $name_error {
                    $(
                    $field: api_entities!(@parse_error $type, expr_1: {
                        let mut errors: Vec<$validation_error_type> = Vec::new();
                        $($(
                            if let Err(e) = $validator_type::$validator((&self.$field $($(, &self.$validation_field)* )?)) {
                                errors.push(e.into());
                                erronous = true;
                            }
                        )*)?
                        errors
                    }, expr_2: {
                        let errors = &self.$field.validate();
                        erronous = errors.is_err();
                        errors
                    }),
                    )*
                };
                match erronous {
                    true => Err(result),
                    false => Ok(()),
                }
            }
        } 
        )*
    };
    (@parse_type RecordOf<$ty:ty, $ty_err:ty>) => {
        RecordOf<$ty>
    };
    (@parse_type $ty:ty) => {
        $ty
    };
    (@parse_error_type $error_type:ty, RecordOf<$ty:ty, $ty_err:ty>) => {
        Result<(), $ty_err>
    };
    (@parse_error_type $error_type:ty, $ty:ty) => {
        Vec<$error_type>
    };
    (@parse_error RecordOf<$ty:ty, $ty_err:ty>, expr_1: $e1:expr, expr_2: $e2:expr) => {
        $e2
    };
    (@parse_error $ty:ty, expr_1: $e1:expr, expr_2: $e2:expr) => {
        $e1
    };
}
