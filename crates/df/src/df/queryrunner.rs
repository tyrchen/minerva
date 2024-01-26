use datafusion::dataframe::DataFrameWriteOptions;
use datafusion::datasource::file_format::parquet::ParquetFormat;
use datafusion::datasource::listing::ListingOptions;
use datafusion::prelude::*;
use datafusion_common::{FileType, GetExt};
use minerva_common::QueryRunner;

use object_store::aws::AmazonS3Builder;
use std::env;
use std::sync::Arc;
use url::Url;

pub struct DFQueryRunner<'a> {
    region: &'a str,
    bucket: &'a str,
    table: &'a str,
    src: &'a str,
    dst: &'a str,
}

impl<'a> QueryRunner for DFQueryRunner<'a> {
    type Response = ();
    type Error = anyhow::Error;

    async fn query(&self, sql: &str) -> std::result::Result<Self::Response, Self::Error> {
        // create local execution context
        let ctx = SessionContext::new();

        let s3 = AmazonS3Builder::new()
            .with_bucket_name(self.bucket)
            .with_region(self.region)
            .with_access_key_id(env::var("AWS_ACCESS_KEY_ID").unwrap())
            .with_secret_access_key(env::var("AWS_SECRET_ACCESS_KEY").unwrap())
            .build()?;

        let path = format!("s3://{}", self.bucket);
        let s3_url = Url::parse(&path).unwrap();
        let arc_s3 = Arc::new(s3);
        ctx.runtime_env()
            .register_object_store(&s3_url, arc_s3.clone());

        let path = format!("s3://{}/{}/", self.bucket, self.src);
        let file_format = ParquetFormat::default().with_enable_pruning(Some(true));
        let listing_options = ListingOptions::new(Arc::new(file_format))
            .with_file_extension(FileType::PARQUET.get_ext());
        ctx.register_listing_table(self.table, &path, listing_options, None, None)
            .await?;

        // execute the query
        let df = ctx.sql(sql).await?;

        let out_path = format!("s3://{}/{}/", self.bucket, self.dst);
        df.clone()
            .write_parquet(&out_path, DataFrameWriteOptions::new(), None)
            .await?;

        //write as JSON to s3
        let json_out = format!("s3://{}/json_out", self.bucket);
        df.clone()
            .write_json(&json_out, DataFrameWriteOptions::new())
            .await?;

        //write as csv to s3
        let csv_out = format!("s3://{}/csv_out", self.bucket);
        df.write_csv(&csv_out, DataFrameWriteOptions::new(), None)
            .await?;

        let file_format = ParquetFormat::default().with_enable_pruning(Some(true));
        let listing_options = ListingOptions::new(Arc::new(file_format))
            .with_file_extension(FileType::PARQUET.get_ext());
        ctx.register_listing_table("test2", &out_path, listing_options, None, None)
            .await?;

        let df = ctx
            .sql(
                "SELECT * \
        FROM test2 \
        ",
            )
            .await?;

        df.show_limit(20).await?;

        Ok(())
    }
}
