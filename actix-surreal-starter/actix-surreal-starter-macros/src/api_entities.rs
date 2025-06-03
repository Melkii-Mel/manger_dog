#[macro_export]
macro_rules! api_entities {
    (
        validator: $validator_type:ident,
        error: $validation_error_type:ident,
        $(
            $name:ident|$name_error:ident( $db_table_name:literal $( [ $( $path_to_ownership:literal ),* ] )? )
            {
                $(
                    $field:ident: $type:ty$(|$record_of_error:ty)? $( [ $( $validator:ident $( ( $( $validation_field:ident ),*$(,)? ) )? ),* $(,)? ] )?
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
                    actix_surreal_starter::crud_ops::select_all::<$name>(user_id.0.into(), $name::query_builder()).await.map(actix_web::web::Json)
                }
            ))
            .route(concat!("/api/", $db_table_name), actix_web::web::get().to(
                |id: actix_web::web::Json<::surrealdb::RecordId>, user_id: actix_surreal_starter::UserId| async move {
                    actix_surreal_starter::crud_ops::select::<$name>(id.0.into(), user_id.0.into(), $name::query_builder()).await.map(actix_web::web::Json)
                }
            ))
            .route(concat!("/api/", $db_table_name), actix_web::web::post().to(
                |mut entity: actix_web::web::Json<$name>, user_id: actix_surreal_starter::UserId| async move {
                    let result = if let Err(e) = ::actix_surreal_starter::Entity::validate(&entity.0) {
                        Err(e)
                    }
                    else {
                        Ok(entity.0.insert(user_id).await?)
                    };
                    Ok::<_, ::actix_surreal_starter::crud_ops::CrudError>(::actix_web::HttpResponse::Ok().json(result))
                }
            ))
            .route(concat!("/api/", $db_table_name), actix_web::web::put().to(
                |entity: actix_web::web::Json<actix_surreal_starter_types::WithId<$name>>, user_id: actix_surreal_starter::UserId| async move {
                    let result = if let Err(e) = ::actix_surreal_starter::Entity::validate(&entity.0.data) {
                        Err(e)
                    }
                    else {
                        Ok($name::update(entity.0, user_id).await?)
                    };
                    Ok::<_, ::actix_surreal_starter::crud_ops::CrudError>(::actix_web::HttpResponse::Ok().json(result))
                }
            ))
            .route(concat!("/api/", $db_table_name), actix_web::web::delete().to(
                |id: actix_web::web::Json<::surrealdb::RecordId>, user_id: actix_surreal_starter::UserId| async move {
                    actix_surreal_starter::crud_ops::delete(id.0.into(), user_id.0.into(), $name::query_builder()).await
                }
            ))
            )*;
        }

        $(
        #[derive(std::fmt::Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq)]
        pub struct $name {
            $(pub $field: api_entities!(@parse_type $type)),*
        }
        #[derive(std::fmt::Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq)]
        pub struct $name_error {
            $(pub $field: api_entities!(@parse_error_type $validation_error_type $(, $record_of_error )? )),*
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
                    fkey_path_map: None, // TODO: Required to use select_all_by_fkey(), e.g. to select all loan payments for a loan
                }
            }
            #[cfg(feature = "server")]
            pub async fn insert(
                mut self,
                user_id: actix_surreal_starter::UserId,
            ) -> Result<::serde_json::Value, ::actix_surreal_starter::crud_ops::CrudError> {
                let mut result;
                (self, result) = self.insert_children(user_id.clone()).await?;
                let id =
                    ::actix_surreal_starter::crud_ops::insert(self, user_id.0.into(), $name::query_builder(), $name::paths()[0] == "user_id")
                        .await?;
                result.insert("id".to_string(), ::serde_json::to_value(id)?);
                Ok(serde_json::Value::Object(result))
            }
            #[cfg(feature = "server")]
            pub async fn update(with_id: actix_surreal_starter_types::WithId<$name>, user_id: actix_surreal_starter::UserId) -> Result<::serde_json::Value, ::actix_surreal_starter::crud_ops::CrudError> {
                let mut data = with_id.data;
                let id = with_id.id;
                let mut result;
                (data, result) = data.insert_children(user_id.clone()).await?;
                actix_surreal_starter::crud_ops::update(id, data, user_id.0.into(), $name::query_builder()).await?;
                Ok(serde_json::Value::Object(result))
            }
            #[cfg(feature = "server")]
            async fn insert_children(mut self, user_id: actix_surreal_starter::UserId) -> Result<(Self, ::serde_json::map::Map<String, ::serde_json::Value>), ::actix_surreal_starter::crud_ops::CrudError> {
                let mut result = ::serde_json::Map::new();
                $(
                api_entities!(@parse_match_record_of $( $record_of_error, )* expr_1: {}, expr_2: {
                    if let ::actix_surreal_starter_types::RecordOf::Record(record) = self.$field {
                        let inserted = record.insert(user_id.clone()).await?;
                        self.$field = ::actix_surreal_starter_types::RecordOf::Id(serde_json::from_value(inserted.get("id").unwrap().clone()).unwrap());
                        result.insert(stringify!($field).to_string(), inserted);
                    }
                });
                )*
                Ok((self, result))
            }
        }
        impl ::actix_surreal_starter_types::Entity for $name {
            type Error = $name_error;
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
                    $field: api_entities!(@parse_match_record_of $( $record_of_error, )* expr_1: {
                        let mut errors: Vec<$validation_error_type> = Vec::new();
                        $($(
                            if let Err(e) = $validator_type::$validator((&self.$field $($(, &self.$validation_field)* )?)) {
                                errors.push(e.into());
                                erronous = true;
                            }
                        )*)?
                        errors
                    }, expr_2: {
                        let errors$(: Box<Result<(), $record_of_error>> )? = Box::new({ if let actix_surreal_starter_types::RecordOf::Record(record) = &self.$field {
                                record.validate()
                            } else {
                                Ok(())
                            }
                        });
                        erronous |= errors.is_err();
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
        impl ::actix_surreal_starter_types::_ReplaceWithIds for $name {
            fn _replace_with_ids(mut self, value: &mut ::serde_json::Value) -> Result<Self, serde_json::Error> {
                use ::actix_surreal_starter_types::ReplaceWithIds as _;
                $(
                api_entities!(@parse_match_record_of $( $record_of_error, )? expr_1: {} , expr_2: {
                    if let ::actix_surreal_starter_types::RecordOf::Record(replacing) = self.$field {
                        let replaced = replacing.replace_with_ids(value[stringify!($field)].take());
                        self.$field = actix_surreal_starter_types::RecordOf::Id(replaced?);
                    }
                });
                )*
                Ok(self)
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
    (@parse_error_type $error_type:ty, $ty_err:ty) => {
        Box<Result<(), $ty_err>>
    };
    (@parse_error_type $error_type:ty) => {
        Vec<$error_type>
    };
    (@parse_match_record_of $ty_err:ty, expr_1: $e1:expr, expr_2: $e2:expr) => {
        $e2
    };
    (@parse_match_record_of expr_1: $e1:expr, expr_2: $e2:expr) => {
        $e1
    };
}

#[macro_export]
macro_rules! enums {
    ($($name:ident($table_name:literal)),*$(,)?) => {
        #[cfg(feature = "server")]
        pub fn configure_enum_endpoints(cfg: &mut actix_web::web::ServiceConfig) {
            cfg
            $(
            .route(concat!("/api/", $table_name, "/all"), actix_web::web::get().to(
                |_: actix_surreal_starter::UserId| async move {
                    actix_surreal_starter::crud_ops::select_all_raw::<$name>(concat!("SELECT * FROM ", $table_name)).await.map(actix_web::web::Json)
                }
            ))
            .route(concat!("/api/", $table_name), actix_web::web::get().to(
                |id: actix_web::web::Json<::actix_surreal_starter_types::RecordId>, _: actix_surreal_starter::UserId| async move {
                    actix_surreal_starter::crud_ops::select_raw_by_id::<$name>(id.0.into()).await.map(actix_web::web::Json)
                }
            ))
            )*;
        }
        $(
        #[derive(std::fmt::Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq)]
        pub struct $name {
            pub identifier: String,
        }
        impl ::actix_surreal_starter_types::Entity for $name {
            type Error = ();
            fn table_name() -> &'static str {
                $table_name
            }
            fn api_location() -> &'static str {
                concat!("/api/", $table_name)
            }
            fn validate(&self) -> Result<(), ()> {
                Ok(())
            }
        }
        )*
    };
}