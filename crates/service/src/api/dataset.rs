use crate::AppState;
use aws_sdk_s3::types::ObjectAttributes;
// use aws_sdk_s3 as s3;
use aws_smithy_http_server::Extension;
use dataset_server_sdk::{
    error, input,
    model::{DatasetField, DatasetInfo},
    output,
    types::Blob,
};
use minerva_clickhouse::ClickHouseRunner;
use minerva_common::DatasetDescriber;
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
    let items = get_assets(&state.config.data_bucket, &state.s3)
        .await
        .unwrap();
    let output = output::ListDatasetOutput {
        items,
        next_token: None,
    };
    Ok(output)
}

pub async fn get_dataset(
    input: input::GetDatasetInput,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<output::GetDatasetOutput, error::GetDatasetError> {
    info!("get_dataset: {:?}", input);
    let asset = get_asset(&state.config.data_bucket, &input.id, &state.s3)
        .await
        .unwrap();
    let output = output::GetDatasetOutput {
        name: asset.name,
        table_name: asset.table_name,
        last_modified: asset.last_modified,
        size: asset.size,
        fields: asset.fields,
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

async fn get_assets(bucket: &str, client: &aws_sdk_s3::Client) -> anyhow::Result<Vec<DatasetInfo>> {
    let objects = client
        .list_objects()
        .bucket(bucket)
        .send()
        .await?
        .contents
        .unwrap();

    let mut items = vec![];
    let mut tasks = vec![];

    for object in objects {
        let task = tokio::spawn(async move {
            get_asset_by_object(object.key().unwrap(), object.last_modified, object.size).await
        });

        tasks.push(task);
    }

    for task in tasks {
        let item = task.await??;
        items.push(item);
    }

    Ok(items)
}

async fn get_asset(
    bucket: &str,
    key: &str,
    client: &aws_sdk_s3::Client,
) -> anyhow::Result<DatasetInfo> {
    let object = client
        .get_object_attributes()
        .bucket(bucket)
        .key(key)
        .object_attributes(ObjectAttributes::ObjectSize)
        .send()
        .await?;
    get_asset_by_object(key, object.last_modified, object.object_size).await
}

async fn get_asset_by_object(
    name: &str,
    last_modified: Option<::aws_smithy_types::DateTime>,
    size: Option<i64>,
) -> anyhow::Result<DatasetInfo> {
    let runner = ClickHouseRunner::new_s3(name);
    let table_name = runner.table_name();
    let info = runner.describe().await?;
    let fields = info
        .columns
        .into_iter()
        .map(|v| DatasetField {
            name: v.name,
            r#type: v.r#type,
            nullable: v.nullable,
        })
        .collect();

    let size = size.unwrap();
    let last_modified = last_modified.unwrap();
    let item = DatasetInfo {
        name: name.to_string(),
        table_name,
        size,
        last_modified,
        fields,
    };

    Ok(item)
}
