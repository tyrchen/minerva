use std::{env, path::Path};

use crate::get_top_level_key;

pub enum DataSource {
    S3(S3Bucket),
    Local(LocalFile),
}

pub struct S3Bucket {
    pub name: String,
    pub region: String,
    pub key: String,
    pub need_expand: bool,
}

pub struct LocalFile {
    pub path: String,
}

impl DataSource {
    pub fn table_name(&self) -> String {
        let name = match self {
            DataSource::S3(ref bucket) => bucket.key_name(),
            DataSource::Local(ref local_file) => &local_file.path,
        };
        Path::new(name)
            .file_stem()
            .and_then(|s| s.to_str())
            .and_then(|s| s.split('.').next())
            .map(|s| s.to_string())
            .unwrap()
    }

    pub fn as_source(&self) -> String {
        match self {
            DataSource::S3(ref v) => format!("s3('{}')", v.s3url()),
            DataSource::Local(ref v) => format!("file('{}')", v.path),
        }
    }
}

impl S3Bucket {
    pub fn new(key: impl Into<String>) -> Self {
        let name = env::var("DATA_BUCKET").unwrap();
        let region = env::var("DATA_BUCKET_REGION").unwrap_or_else(|_| "us-west-2".to_string());
        let mut key = key.into();
        if key.ends_with('*') {
            key.pop();
            Self {
                name,
                region,
                key,
                need_expand: true,
            }
        } else {
            Self {
                name,
                region,
                key,
                need_expand: false,
            }
        }
    }

    pub fn key_name(&self) -> &str {
        get_top_level_key(&self.key)
    }

    pub fn s3url(&self) -> String {
        if self.need_expand {
            let key_name = self.key_name();
            let ext = Path::new(key_name)
                .extension()
                .and_then(|s| s.to_str())
                .unwrap_or("parquet");
            format!("s3://{}/{}/**/*.{}", self.name, key_name, ext)
        } else {
            format!("s3://{}/{}", self.name, self.key)
        }
    }

    pub fn http_url(&self) -> String {
        format!(
            "https://{}.s3.{}.amazonaws.com/{}",
            self.name, self.region, self.key
        )
    }
}

impl LocalFile {
    pub fn new(path: impl Into<String>) -> Self {
        Self { path: path.into() }
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

    #[test]
    fn get_s3_table_name_should_work() {
        let ds: DataSource = S3Bucket::new("flights.parquet").into();
        assert_eq!(ds.table_name(), "flights");
    }

    #[test]
    fn get_s3_table_name_should_work_with_wildcard() {
        let ds: DataSource = S3Bucket::new("flights.parquet*").into();
        assert_eq!(ds.table_name(), "flights");
        assert_eq!(
            ds.as_source(),
            format!(
                "s3('s3://{}/flights.parquet/**/*.parquet')",
                env::var("DATA_BUCKET").unwrap()
            )
        );
    }

    #[test]
    fn get_local_table_name_should_work() {
        let ds: DataSource = LocalFile::new("crates/clickhouse/fixtures/demo.csv.gz").into();
        assert_eq!(ds.table_name(), "demo");
    }
}
