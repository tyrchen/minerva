use crate::{query::normalize_sql, DataSource, LocalFile, S3Bucket};
use anyhow::Context;
use minerva_common::{ColumnInfo, DatasetDescriber, QueryRunner, TableInfo};
use std::{
    env,
    process::{Command, Stdio},
    thread,
};
use tokio::sync::oneshot;
use tracing::{info, warn};

pub struct ClickHouseRunner {
    pub data_source: DataSource,
    pub log_level: String,
    pub binary_path: String,
}

impl QueryRunner for ClickHouseRunner {
    type Error = anyhow::Error;

    async fn query(&self, sql: &str) -> Result<Vec<u8>, Self::Error> {
        let rx = self.run(sql, false, None)?;
        rx.await?
    }
}

impl DatasetDescriber for ClickHouseRunner {
    type Error = anyhow::Error;

    async fn describe(&self) -> Result<TableInfo, Self::Error> {
        let table_name = self.table_name();
        let rx = self.run(
            format!("describe {}", table_name),
            true,
            Some("JSONEachRow"),
        )?;
        let ret = rx.await??;
        let mut columns = Vec::new();
        for item in ret.split(|c| *c == b'\n') {
            if item.is_empty() {
                continue;
            }
            let item: ColumnInfo = serde_json::from_slice(item)
                .with_context(|| format!("failed to parse as json: {:?}", item))?;

            if item.r#type.starts_with("Nullable(") {
                let r#type = item
                    .r#type
                    .trim_start_matches("Nullable(")
                    .trim_end_matches(')');
                columns.push(ColumnInfo {
                    name: item.name,
                    r#type: r#type.to_string(),
                    nullable: true,
                });
            } else {
                columns.push(item);
            }
        }
        Ok(TableInfo {
            name: table_name,
            columns,
        })
    }

    async fn sample(&self, size: usize) -> Result<Vec<u8>, Self::Error> {
        let table_name = self.table_name();
        let rx = self.run(
            format!("SELECT * FROM {} LIMIT {}", table_name, size),
            false,
            None,
        )?;
        rx.await?
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
        sql: impl AsRef<str>,
        allow_non_query: bool,
        format: Option<&'static str>,
    ) -> anyhow::Result<oneshot::Receiver<anyhow::Result<Vec<u8>>>> {
        let (tx, rx) = oneshot::channel();

        let mut cmd = self.build_command(sql, allow_non_query, format)?;

        info!("command: {:?}", cmd);
        thread::spawn(move || {
            let output = cmd.output().context("failed to run command")?;
            let msg = if !output.status.success() {
                let msg = String::from_utf8_lossy(&output.stderr);
                warn!("stderr: {:?}", msg);
                Err(anyhow::anyhow!("failed to execute query: {msg}"))
            } else {
                Ok(output.stdout)
            };
            if let Err(e) = tx.send(msg) {
                warn!("failed to send execution result: {:?}", e);
            }
            Ok::<(), anyhow::Error>(())
        });

        Ok(rx)
    }

    pub fn table_name(&self) -> String {
        self.data_source.table_name()
    }

    fn build_command(
        &self,
        sql: impl AsRef<str>,
        allow_non_query: bool,
        format: Option<&'static str>,
    ) -> anyhow::Result<Command> {
        let query = normalize_sql(
            sql.as_ref(),
            &self.data_source,
            allow_non_query,
            format,
            None,
        )?;

        info!("query: {}", query);
        let mut cmd = Command::new(&self.binary_path);
        // TODO: compressed arrow is not supported in arrow js
        cmd.stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .arg("local")
            .arg(format!("--query={}", query));
        Ok(cmd)
    }

    fn clickhouse_path() -> String {
        env::var("CLICKHOUSE_PATH").unwrap_or_else(|_| "clickhouse".to_string())
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
        runner.run("SELECT * FROM test LIMIT 10", false, None)?;

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
        assert_eq!(table_info.columns[0].r#type, "Float64");
        assert!(table_info.columns[0].nullable);
        assert_eq!(table_info.columns[1].name, "ARRIVAL_DELAY");
        assert_eq!(table_info.columns[1].r#type, "Float64");
        assert!(table_info.columns[1].nullable);

        Ok(())
    }

    #[tokio::test]
    async fn clickhouse_runner_should_sample() -> Result<()> {
        tracing_subscriber::fmt::init();
        let runner = ClickHouseRunner::new_local("fixtures/test.arrow");
        let sample = runner.sample(100).await?;
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
