#[macro_export]
macro_rules! api_entities {
    (
        validator: $validator_type:ident,
        error: $validation_error_type:ident,
        $(
            $name:ident( $db_table_name:literal $( [ $( $path_to_ownership:literal ),* ] )? )
            {
                $(
                    $field:ident: $type:ty $( [ $( $( $validator_type_override:ident::)? $validator:ident $( ( $( $validation_field:ident ),*$(,)? ) )? ),* $(,)? ] )?
                ),*$(,)*
            }
        )*
    ) => {

        pub static PATHS:phf::Map<&'static str, &'static [&'static str]> = phf::phf_map! {
            $($db_table_name => &[$($($path_to_ownership),*)?] as &[&str],)*
        };

        pub fn configure_endpoints(cfg: &mut actix_web::web::ServiceConfig) {
            cfg
            $(
            .route(concat!("/api/", $db_table_name, "/all"), actix_web::web::get().to(
                |user_id: actix_surreal_starter::UserId| async move {
                    actix_surreal_starter::crud_ops::select_all::<$name>(user_id.0, $name::query_builder()).await.map(actix_web::web::Json)
                }
            ))
            .route(concat!("/api/", $db_table_name), actix_web::web::get().to(
                |id: actix_web::web::Json<actix_surreal_starter::api::Id>, user_id: actix_surreal_starter::UserId| async move {
                    actix_surreal_starter::crud_ops::select::<$name>(id.0.0, user_id.0, $name::query_builder()).await.map(actix_web::web::Json)
                }
            ))
            .route(concat!("/api/", $db_table_name), actix_web::web::post().to(
                |entity: actix_web::web::Json<$name>, user_id: actix_surreal_starter::UserId| async move {
                    actix_surreal_starter::crud_ops::insert(entity.0, user_id.0, $name::query_builder()).await
                }
            ))
            .route(concat!("/api/", $db_table_name), actix_web::web::put().to(
                |entity: actix_web::web::Json<actix_surreal_starter::api::WithId<serde_json::Value>>, user_id: actix_surreal_starter::UserId| async move {
                    actix_surreal_starter::crud_ops::update(entity.0.id, entity.0.inner, user_id.0, $name::query_builder()).await
                }
            ))
            .route(concat!("/api/", $db_table_name), actix_web::web::delete().to(
                |id: actix_web::web::Json<actix_surreal_starter::api::Id>, user_id: actix_surreal_starter::UserId| async move {
                    actix_surreal_starter::crud_ops::delete(id.0.0, user_id.0, $name::query_builder()).await
                }
            ))
            )*;
        }

        $(
        #[derive(std::fmt::Debug, serde::Deserialize, serde::Serialize, Default, Clone)]
        pub struct $name {
            $(pub $field: $type),*
        }

        impl $name {
            fn paths() -> &'static [&'static str] {
                PATHS.get(Self::table_name()).unwrap()
            }
            fn table_name() -> &'static str {
                $db_table_name
            }
            fn query_builder() -> actix_surreal_starter::query_builder::QueryBuilder {
                actix_surreal_starter::query_builder::QueryBuilder {
                    paths: Self::paths(),
                    table_name: Self::table_name(),
                    fkey_path_map: None, //TODO: nah oh it can't be None it's just a placeholder
                }
            }
            fn validate(&self) -> Vec<$validation_error_type> {
                let mut result: Vec<$validation_error_type> = Vec::new();
                $($($(
                if let Err(e) = api_entities!(@parse_validator_type $validator_type $(=> $validator_type_override )?)::$validator((&self.$field $($(, &self.$validation_field)* )?)) {
                    result.push(e);
                }
                )*)?)*
                result
            }
        }
        )*
    };
    (@parse_validator_type $validator_type:ident => $validator_type_override:ident) => {
        $validator_type_override
    };
    (@parse_validator_type $validator_type:ident) => {
        $validator_type
    };
}
