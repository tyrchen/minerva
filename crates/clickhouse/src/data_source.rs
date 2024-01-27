use std::{env, path::Path};

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

impl DataSource {
    pub fn table_name(&self) -> String {
        let name = match self {
            DataSource::S3(ref bucket) => &bucket.key,
            DataSource::Local(ref local_file) => &local_file.path,
        };
        Path::new(name)
            .file_stem()
            .and_then(|s| s.to_str())
            .and_then(|s| s.split('.').next())
            .map(|s| s.to_string())
            .unwrap()
    }
}

impl S3Bucket {
    pub fn new(key: impl Into<String>) -> Self {
        let name = env::var("DATA_BUCKET").unwrap();
        let region = env::var("DATA_BUCKET_REGION").unwrap_or_else(|_| "us-west-2".to_string());
        Self {
            name,
            region,
            key: key.into(),
        }
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
    fn get_local_table_name_should_work() {
        let ds: DataSource = LocalFile::new("crates/clickhouse/fixtures/demo.csv.gz").into();
        assert_eq!(ds.table_name(), "demo");
    }
}
