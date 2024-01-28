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
use minerva_clickhouse::{get_top_level_key, ClickHouseRunner};
use minerva_common::{DatasetDescriber, QueryRunner};
use std::{collections::HashSet, sync::Arc};
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
    let runner = ClickHouseRunner::new_s3(input.id);

    match runner.query(&input.sql).await {
        Ok(data) => {
            let output = output::QueryDatasetOutput {
                data: Blob::new(data),
            };
            Ok(output)
        }
        Err(e) => Err(error::QueryDatasetError::ClickhouseQueryError(
            error::ClickhouseQueryError {
                message: e.to_string(),
            },
        )),
    }
}

pub async fn sample_dataset(
    input: input::SampleDatasetInput,
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<output::SampleDatasetOutput, error::SampleDatasetError> {
    info!("sample_dataset: {:?}", input);
    let runner = ClickHouseRunner::new_s3(input.id);
    let data = runner.sample().await.unwrap();
    let output = output::SampleDatasetOutput {
        data: Blob::new(data),
    };
    Ok(output)
}

async fn get_assets(bucket: &str, client: &aws_sdk_s3::Client) -> anyhow::Result<Vec<DatasetInfo>> {
    let objects = client
        .list_objects_v2()
        .max_keys(1000)
        .bucket(bucket)
        .send()
        .await?
        .contents
        .unwrap();

    let unique_objects = get_top_level_objects(objects);
    let mut items = vec![];
    let mut tasks = vec![];

    for object in unique_objects {
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

fn get_top_level_objects(
    objects: Vec<aws_sdk_s3::types::Object>,
) -> Vec<aws_sdk_s3::types::Object> {
    let mut unique_objects = HashSet::new();
    let mut ret = vec![];
    for object in objects {
        let key = object.key().unwrap().to_owned();
        if key.ends_with(".parquet")
            || key.ends_with(".csv")
            || key.ends_with(".json")
            || key.ends_with(".csv.gz")
            || key.ends_with(".json.gz")
        {
            let key = get_top_level_key(&key);

            if unique_objects.contains(key) {
                continue;
            }
            unique_objects.insert(key.to_owned());
            ret.push(object);
        }
    }
    ret
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
        name: get_top_level_key(name).to_owned(),
        table_name,
        size,
        last_modified,
        fields,
    };

    Ok(item)
}
