use anyhow::Context;
use minerva_common::{ColumnInfo, DatasetDescriber, QueryRunner, TableInfo};
use std::{
    env,
    path::Path,
    process::{Command, Stdio},
};
use tracing::info;
pub struct ClickHouseRunner {
    pub data_source: DataSource,
    pub log_level: String,
    pub binary_path: String,
}

pub enum DataSource {
    S3(S3Bucket),
    Local(LocalFile),
}

pub struct S3Bucket {
    pub name: String,
    pub region: String,
    pub key: String,
}

pub struct LocalFile {
    pub path: String,
}

impl QueryRunner for ClickHouseRunner {
    type Response = Vec<u8>;
    type Error = anyhow::Error;

    async fn query(&self, sql: &str) -> Result<Self::Response, Self::Error> {
        self.run(sql, None)
    }
}

impl DatasetDescriber for ClickHouseRunner {
    type Error = anyhow::Error;

    async fn describe(&self) -> Result<TableInfo, Self::Error> {
        let table_name = self.table_name();
        let ret = self.run(format!("describe {}", table_name), Some("JSONEachRow"))?;
        let mut columns = Vec::new();
        for item in ret.split(|c| *c == b'\n') {
            if item.is_empty() {
                continue;
            }
            let item: ColumnInfo = serde_json::from_slice(item)
                .with_context(|| format!("failed to parse as json: {:?}", item))?;
            columns.push(item);
        }
        Ok(TableInfo {
            name: table_name,
            columns,
        })
    }

    async fn sample(&self) -> Result<Vec<u8>, Self::Error> {
        let table_name = self.table_name();
        self.run(format!("SELECT * FROM {} LIMIT 10", table_name), None)
    }
}

impl ClickHouseRunner {
    pub fn new_s3(dataset_name: impl Into<String>) -> Self {
        let bucket = S3Bucket::new(dataset_name);
        Self {
            data_source: bucket.into(),
            log_level: "information".to_string(),
            binary_path: Self::clickhouse_path(),
        }
    }

    pub fn new_local(file_path: impl Into<String>) -> Self {
        let local_file = LocalFile {
            path: file_path.into(),
        };
        Self {
            data_source: local_file.into(),
            log_level: "information".to_string(),
            binary_path: Self::clickhouse_path(),
        }
    }

    pub fn run(
        &self,
        query: impl AsRef<str>,
        format: Option<&'static str>,
    ) -> anyhow::Result<Vec<u8>> {
        let mut cmd = self
            .build_command(query, format)
            .context("failed to build command")?;
        let output = cmd.output().context("failed to run command")?;
        Ok(output.stdout)
    }

    fn build_command(
        &self,
        query: impl AsRef<str>,
        format: Option<&'static str>,
    ) -> anyhow::Result<Command> {
        let table_name = self.table_name();
        let query = match self.data_source {
            DataSource::S3(ref bucket) => {
                let s3_table = format!(
                    "s3('https://{}.s3.{}.amazonaws.com/{}')",
                    bucket.name, bucket.region, bucket.key
                );
                query.as_ref().replace(&table_name, &s3_table)
            }
            DataSource::Local(ref local_file) => {
                let file_table = format!("file('{}')", local_file.path);
                query.as_ref().replace(&table_name, &file_table)
            }
        };

        info!("query: {}", query);
        let mut cmd = Command::new(&self.binary_path);
        cmd.stdout(Stdio::piped()).arg("local").arg(format!(
            "--query={} format {}",
            query,
            format.unwrap_or("Arrow")
        ));
        info!("cmd: {:?}", cmd);
        Ok(cmd)
    }

    fn table_name(&self) -> String {
        let name = match self.data_source {
            DataSource::S3(ref bucket) => &bucket.key,
            DataSource::Local(ref local_file) => &local_file.path,
        };
        Path::new(name)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }

    fn clickhouse_path() -> String {
        env::var("CLICKHOUSE_PATH").unwrap_or_else(|_| "clickhouse".to_string())
    }
}

impl S3Bucket {
    pub fn new(key: impl Into<String>) -> Self {
        let name = env::var("DATA_BUCKET").unwrap_or_else(|_| "ds-data-438be5a".to_string());
        let region = env::var("DATA_BUCKET_REGION").unwrap_or_else(|_| "us-west-2".to_string());
        Self {
            name,
            region,
            key: key.into(),
        }
    }
}

impl From<S3Bucket> for DataSource {
    fn from(s3_bucket: S3Bucket) -> Self {
        Self::S3(s3_bucket)
    }
}

impl From<LocalFile> for DataSource {
    fn from(local_file: LocalFile) -> Self {
        Self::Local(local_file)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use minerva_common::arrow2json;
    use serde_json::json;

    #[test]
    fn clickhouse_runner_should_work() -> Result<()> {
        tracing_subscriber::fmt::init();
        let runner = ClickHouseRunner::new_local("fixtures/test.arrow");
        runner.run("SELECT * FROM test LIMIT 10", None)?;

        Ok(())
    }

    #[tokio::test]
    async fn clickhouse_runner_should_describe() -> Result<()> {
        tracing_subscriber::fmt::init();
        let runner = ClickHouseRunner::new_local("fixtures/test.arrow");
        let table_info = runner.describe().await?;
        assert_eq!(table_info.name, "test");
        assert_eq!(table_info.columns.len(), 4);
        assert_eq!(table_info.columns[0].name, "DEPARTURE_DELAY");
        assert_eq!(table_info.columns[0].r#type, "Nullable(Float64)");
        assert_eq!(table_info.columns[1].name, "ARRIVAL_DELAY");
        assert_eq!(table_info.columns[1].r#type, "Nullable(Float64)");

        Ok(())
    }

    #[tokio::test]
    async fn clickhouse_runner_should_sample() -> Result<()> {
        tracing_subscriber::fmt::init();
        let runner = ClickHouseRunner::new_local("fixtures/test.arrow");
        let sample = runner.sample().await?;
        let json = arrow2json(sample)?;
        let data: Vec<serde_json::Value> = serde_json::from_str(&json)?;
        assert_eq!(
            data[0],
            json!({
                "DEPARTURE_DELAY": -11.0,
                "ARRIVAL_DELAY": -22.0,
                "DISTANCE": 1448,
                "SCHEDULED_DEPARTURE": 0.08333333333333333
            })
        );

        Ok(())
    }
}
