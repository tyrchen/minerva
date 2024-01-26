use crate::AppState;
// use aws_sdk_s3 as s3;
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
    let config = aws_config::load_from_env().await;
    let _client = aws_sdk_s3::Client::new(&config);

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

pub async fn sample_dataset(
    input: input::SampleDatasetInput,
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<output::SampleDatasetOutput, error::SampleDatasetError> {
    info!("sample_dataset: {:?}", input);
    let output = output::SampleDatasetOutput {
        data: Blob::new(vec![]),
    };
    Ok(output)
}
