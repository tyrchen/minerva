mod data_source;
mod query;
mod runner;

pub use data_source::{DataSource, LocalFile, S3Bucket};
pub use runner::ClickHouseRunner;

pub fn get_top_level_key(key: &str) -> &str {
    key.split('/').next().unwrap()
}
