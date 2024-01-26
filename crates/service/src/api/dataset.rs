use crate::{api::get_aws_config, AppState};
// use aws_sdk_s3 as s3;
use aws_smithy_http_server::Extension;
use dataset_server_sdk::{
    error, input,
    model::{DatasetField, DatasetInfo},
    output,
    types::Blob,
};
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
    Extension(state): Extension<Arc<AppState>>,
) -> Result<output::ListDatasetOutput, error::ListDatasetError> {
    info!("list_dataset: {:?}", input);
    let config = get_aws_config().await;
    let client = aws_sdk_s3::Client::new(&config);
    let bucket = &state.config.data_bucket;
    let objects = client
        .list_objects()
        .bucket(bucket)
        .send()
        .await
        .unwrap()
        .contents
        .unwrap();

    let mut items = vec![];

    for object in objects {
        let name = object.key.unwrap();
        let size = object.size.unwrap();
        let last_modified = object.last_modified.unwrap();
        let fields = vec![];
        let item = DatasetInfo {
            name,
            size,
            last_modified: aws_smithy_types::DateTime::from_secs(last_modified.secs()),
            fields,
        };
        items.push(item);
    }

    let output = output::ListDatasetOutput {
        items: vec![],
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
        last_modified: aws_smithy_types::DateTime::from_secs(1),
        size: 0,
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

#[allow(dead_code)]
fn get_asset_info(_key: &str) -> anyhow::Result<Vec<DatasetField>> {
    todo!()
}
