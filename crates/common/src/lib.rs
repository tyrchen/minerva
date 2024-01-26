mod convert;

pub use convert::*;

use serde::{Deserialize, Serialize};

#[allow(async_fn_in_trait)]
pub trait QueryRunner {
    type Response;
    type Error;
    async fn query(&self, sql: &str) -> Result<Self::Response, Self::Error>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableInfo {
    pub name: String,
    pub columns: Vec<ColumnInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnInfo {
    pub name: String,
    pub r#type: String,
}

#[allow(async_fn_in_trait)]
pub trait DatasetDescriber {
    type Error;
    async fn describe(&self) -> Result<TableInfo, Self::Error>;
    async fn sample(&self) -> Result<Vec<u8>, Self::Error>;
}
