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