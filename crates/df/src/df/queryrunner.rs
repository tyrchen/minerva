use anyhow::Ok;
use datafusion::prelude::*;
use datafusion_common::arrow::record_batch::RecordBatch;
use minerva_common::QueryRunner;

use object_store::aws::AmazonS3Builder;

use std::sync::Arc;
use url::Url;

pub struct DFQueryRunner {
    region: String,
    bucket: String,
    table: String,
    src: String,
}

impl DFQueryRunner {
    pub fn new_s3(dataset_name: impl Into<String>) -> Self {
        let src: String = dataset_name.into();
        let table_name = src.split('.').collect::<Vec<_>>()[0];
        Self {
            region: "us-west-2".to_owned(),
            bucket: "ds-data-438be5a".to_owned(),
            table: table_name.to_owned(),
            src,
        }
    }
}

impl QueryRunner for DFQueryRunner {
    type Error = anyhow::Error;

    async fn query(&self, sql: &str) -> std::result::Result<Vec<u8>, Self::Error> {
        // create local execution context
        let ctx = SessionContext::new();

        let s3 = AmazonS3Builder::new()
            .with_bucket_name(&self.bucket)
            .with_region(&self.region)
            .with_access_key_id("your-access-key")
            .with_secret_access_key("your-secret-key")
            .build()?;

        let path = format!("s3://{}", self.bucket);
        let s3_url = Url::parse(&path).unwrap();
        let arc_s3 = Arc::new(s3);
        ctx.runtime_env()
            .register_object_store(&s3_url, arc_s3.clone());

        let path = format!("s3://{}/{}", self.bucket, self.src);
        ctx.register_parquet(&self.table, &path, ParquetReadOptions::default())
            .await?;

        // execute the query
        let df = ctx.sql(sql).await?;
        // let schema = df.schema();
        // print!("{}", schema.to_string());

        df.clone().show().await?;

        let record_batches = df.collect().await?;
        // Failed try to convert arrow format
        // let cur = Cursor::new(Vec::<u8>::new());
        // let mut arrow_writer = StreamWriter::try_new(cur, &record_batches[0].schema())?;

        // for record in record_batches {
        //     arrow_writer.write(&record);
        // }
        // arrow_writer.finish()?;

        // let result = arrow_writer.into_inner()?;
        // Ok(result.into_inner())

        let record_batches_refs: Vec<&RecordBatch> = record_batches.iter().collect();

        let json_rows =
            datafusion::arrow::json::writer::record_batches_to_json_rows(&record_batches_refs[..])
                .unwrap();
        let result = serde_json::to_vec(&json_rows).unwrap();
        Ok(result)
    }
}
