mod dataset;

pub(crate) use dataset::*;

use crate::AppState;
use aws_smithy_http_server::Extension;
use dataset_server_sdk::{error, input, output};
use std::sync::Arc;
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
