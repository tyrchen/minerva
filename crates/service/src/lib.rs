mod api;
mod auth;
mod error;
mod middleware;

pub use api::get_aws_config;

use auth::{AuthConfig, AuthSigner, AuthVerifier};
use aws_smithy_http_server::{
    plugin::IdentityPlugin, request::request_id::ServerRequestIdProviderLayer, AddExtensionLayer,
};
use axum::{
    http::{HeaderName, Method},
    middleware::from_fn,
    response::Html,
    routing::get,
    Router,
};
use axum_swagger_ui::swagger_ui;
use dataset_server_sdk::{DatasetService, DatasetServiceConfig};
use derive_more::Debug;
use middleware::{BearerTokenProviderLayer, ServerTimingLayer};
use serde::{Deserialize, Serialize};
use std::{env, sync::Arc};
use tower_http::cors::{Any, CorsLayer};

#[derive(Debug)]
pub struct AppState {
    pub(crate) config: AppConfig,
    pub(crate) s3: aws_sdk_s3::Client,
    pub(crate) verifier: AuthVerifier,
    pub(crate) signer: AuthSigner,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub server_name: String,
    pub data_bucket: String,
    pub port: u16,
    pub auth: AuthConfig,
}

pub async fn get_router(conf: AppConfig) -> Router {
    // make name with static lifetime
    let name = Box::leak(Box::new(conf.server_name.clone()));

    let state = Arc::new(AppState::new(conf).await);

    let config = DatasetServiceConfig::builder()
        // IdentityPlugin is a plugin that adds a middleware to the service, it just shows how to use plugins
        .http_plugin(IdentityPlugin)
        .layer(AddExtensionLayer::new(state.clone()))
        .layer(BearerTokenProviderLayer::new())
        .layer(ServerRequestIdProviderLayer::new_with_response_header(
            HeaderName::from_static("x-request-id"),
        ))
        .build();
    let api = DatasetService::builder(config)
        .health_check(api::health_check)
        .signin(api::signin)
        .create_dataset(api::create_dataset)
        .list_dataset(api::list_dataset)
        .get_dataset(api::get_dataset)
        .query_dataset(api::query_dataset)
        .sample_dataset(api::sample_dataset)
        .build()
        .expect("failed to build an instance of Echo Service");

    let doc_url = "/swagger/openapi.json";
    let doc =
        include_str!("../../../smithy/build/smithy/source/openapi/DatasetService.openapi.json");

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::PATCH,
            Method::HEAD,
            Method::OPTIONS,
        ])
        .allow_headers(Any)
        // allow requests from any origin
        .allow_origin(Any)
        .allow_private_network(true);

    Router::new()
        .route("/swagger", get(|| async { Html(swagger_ui(doc_url)) }))
        .route(doc_url, get(move || async move { doc }))
        .nest_service("/api/", api)
        .layer(ServerTimingLayer::new(name))
        .layer(cors)
        .layer(from_fn(middleware::check_secret))
        .with_state(state)
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server_name: "minerva".to_string(),
            data_bucket: env::var("DATA_BUCKET").unwrap(),
            port: 3000,
            auth: AuthConfig::default(),
        }
    }
}

impl AppState {
    pub async fn new(config: AppConfig) -> Self {
        let signer = AuthSigner::try_new(&config.server_name, &config.auth.sk).unwrap();
        let verifier = AuthVerifier::try_new(&config.server_name, &config.auth.pk).unwrap();

        let aws_config = get_aws_config().await;
        let s3 = aws_sdk_s3::Client::new(&aws_config);

        Self {
            config,
            s3,
            verifier,
            signer,
        }
    }
}
