use crate::static_files::StaticFilesSetupError::EntityNotFound;
use crate::{EnvNamesConfig, EnvValues};
use actix_files::Files;
use actix_web::web::ServiceConfig;
use actix_web::Responder;
use actix_web::{web, HttpResponse};
use colored::Colorize;
use log::error;
use serde::Deserialize;
use std::fs::{read_to_string, File};
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StaticFilesSetupError {
    #[error("Notice: Static files configuration not provided. {0} is not presented in the environment: {1}"
    )]
    EnvNotSet(&'static str, String),
    #[error("Error: Failed to read config file: {0}")]
    IOError(#[from] io::Error),
    #[error("Error: Failed to parse config: {0}")]
    ConfigFileParsingConfig(#[from] serde_json::Error),
    #[error("Warning: entity not found: {0}. Mount path was not added.")]
    EntityNotFound(String),
    #[error("Error: index.html not found in a directory for the mount path that requires index.html: {0}.")]
    IndexNotFound(String),
    #[error("Error canonizing relative path {0}: {1}")]
    CanonizationError(String, io::Error),
}

#[derive(Debug, Deserialize)]
struct StaticEndpointConfig {
    mount_path: String,
    dir: String,
    index: Option<Index>,
}

#[derive(Debug, Deserialize)]
enum Index {
    Single,
    Multiple,
}

pub struct StaticFilesSetupHandler {
    endpoints: Vec<StaticEndpointConfig>,
    static_file_errors: Vec<StaticFilesSetupError>,
}

impl StaticFilesSetupHandler {
    pub fn new(
        env_values: &EnvValues,
        env_names_config: &EnvNamesConfig,
    ) -> Result<Self, StaticFilesSetupError> {
        let config_file_path = env_values
            .static_files_serving_config
            .as_ref()
            .map_err(|e| {
                StaticFilesSetupError::EnvNotSet(
                    env_names_config.static_files_serving_config,
                    e.to_string(),
                )
            })?;
        let config_text =
            read_to_string(config_file_path).map_err(|e| StaticFilesSetupError::IOError(e))?;
        let mut endpoints: Vec<StaticEndpointConfig> = serde_json::from_str(&config_text)
            .map_err(|e| StaticFilesSetupError::ConfigFileParsingConfig(e))?;
        let mut errors: Vec<StaticFilesSetupError> = Vec::new();
        fn get_path_not_found_error(path: &Path) -> StaticFilesSetupError {
            let path = path.canonicalize().map_err(|e| {
                StaticFilesSetupError::CanonizationError(path.to_string_lossy().to_string(), e)
            });
            match path {
                Ok(path) => {
                    let path_str = path.display().to_string();
                    EntityNotFound(
                        path_str
                            .strip_prefix(r"\\?\")
                            .unwrap_or(&path_str)
                            .to_string(),
                    )
                }
                Err(e) => e,
            }
        }
        endpoints = endpoints
            .into_iter()
            .filter_map(|e| {
                let path = Path::new(&e.dir);
                if !path.exists() {
                    errors.push(get_path_not_found_error(path));
                    return None;
                }
                let path = path.join("index.html");
                if e.index.is_some() && !path.exists() {
                    errors.push(get_path_not_found_error(&path));
                    return None;
                }
                Some(e)
            })
            .collect();

        Ok(Self {
            endpoints,
            static_file_errors: errors,
        })
    }

    pub fn output_errors(&self) {
        self.static_file_errors
            .iter()
            .for_each(|e| println!("{}", e.to_string().yellow()));
    }

    pub fn config(&self, cfg: &mut ServiceConfig) {
        self.endpoints.iter().for_each(|static_endpoint_config| {
            if let Err(e) = std::fs::read_dir(&static_endpoint_config.dir) {
                println!(
                    "Failed to read directory: {}\n{}",
                    static_endpoint_config.dir, e
                );
            }
            let mut files_service = |files_config: fn(Files) -> Files| {
                cfg.service(serve_files_with_config(
                    &static_endpoint_config.mount_path,
                    &static_endpoint_config.dir,
                    files_config,
                ));
            };
            match &static_endpoint_config.index {
                None => files_service(|f| f),
                Some(index) => match index {
                    Index::Single => {
                        let dir = Arc::new(static_endpoint_config.dir.clone());
                        cfg.route(
                            &static_endpoint_config.mount_path,
                            web::get().to(move || serve_index(dir.clone())),
                        );
                    }
                    Index::Multiple => files_service(|f| f.index_file("index.html")),
                },
            }
        });
    }
}

async fn serve_index(dir: Arc<String>) -> impl Responder {
    let index = read_to_string(Path::new(&*dir).join("index.html"))
        .inspect_err(|e| println!("Failed to read index.html file from {}\n{}", &*dir, e))
        .unwrap_or("".to_string());
    HttpResponse::Ok().content_type("text/html").body(index)
}

pub fn serve_files_with_config(
    mount_path: &str,
    dist_dir: &str,
    config: fn(Files) -> Files,
) -> Files {
    if let Err(e) = std::fs::read_dir(dist_dir) {
        println!("Failed to read directory: {}\n{}", dist_dir, e);
    }
    let mut files = Files::new(mount_path, dist_dir);
    files = config(files);
    if cfg!(debug_assertions) {
        files = files.show_files_listing();
    }
    files
}
