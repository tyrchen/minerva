use crate::AppState;
use aws_smithy_http_server::Extension;
use dataset_server_sdk::{error, input, model::DatasetInfo, output, types::Blob};
use std::sync::Arc;
use tracing::info;

pub async fn create_dataset(
    input: input::CreateDatasetInput,
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<output::CreateDatasetOutput, error::CreateDatasetError> {
    info!("create_dataset: {:?}", input);
    let output = output::CreateDatasetOutput {
        name: "test".to_string(),
    };
    Ok(output)
}

pub async fn list_dataset(
    input: input::ListDatasetInput,
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<output::ListDatasetOutput, error::ListDatasetError> {
    info!("list_dataset: {:?}", input);
    let output = output::ListDatasetOutput {
        items: vec![DatasetInfo {
            name: "test".to_string(),
            fields: vec![],
        }],
        next_token: None,
    };
    Ok(output)
}

pub async fn get_dataset(
    input: input::GetDatasetInput,
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<output::GetDatasetOutput, error::GetDatasetError> {
    info!("get_dataset: {:?}", input);
    let output = output::GetDatasetOutput {
        name: "test".to_string(),
        fields: vec![],
    };
    Ok(output)
}

pub async fn query_dataset(
    input: input::QueryDatasetInput,
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<output::QueryDatasetOutput, error::QueryDatasetError> {
    info!("query_dataset: {:?}", input);
    let output = output::QueryDatasetOutput {
        data: Blob::new(vec![]),
    };
    Ok(output)
}
