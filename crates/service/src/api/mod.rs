mod dataset;

use aws_config::SdkConfig;
pub(crate) use dataset::*;

use crate::AppState;
use aws_smithy_http_server::Extension;
use dataset_server_sdk::{error, input, output};
use std::{env, sync::Arc};
use tracing::info;

pub async fn health_check(
    input: input::HealthCheckInput,
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<output::HealthCheckOutput, error::HealthCheckError> {
    info!("echo: {:?}", input);
    let message = input.message;
    let output = output::HealthCheckOutput { message };
    Ok(output)
}

pub async fn signin(
    input: input::SigninInput,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<output::SigninOutput, error::SigninError> {
    let signer = &state.signer;
    let username = input.username;
    info!("signin user: {}", username);
    let token = signer.sign(username)?;
    Ok(output::SigninOutput { token })
}

// debug build
#[cfg(debug_assertions)]
pub async fn get_aws_config() -> SdkConfig {
    let role = env::var("SANDBOX_ROLE").unwrap();
    let name = env::var("SANDBOX_NAME").unwrap_or_else(|_| "tchen".to_string());
    let config = aws_config::load_from_env().await;
    let provider = aws_config::sts::AssumeRoleProvider::builder(role)
        .session_name(name)
        .configure(&config)
        .build()
        .await;

    aws_config::from_env()
        .credentials_provider(provider)
        .load()
        .await
}

// release build
#[cfg(not(debug_assertions))]
pub async fn get_aws_config() -> SdkConfig {
    aws_config::load_from_env().await
}
