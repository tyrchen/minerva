use anyhow::Result;
use minerva_service::get_aws_config;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let config = get_aws_config().await;

    let client = aws_sdk_s3::Client::new(&config);
    let bucket = env::var("DATA_BUCKET").unwrap();
    let ret = client.list_objects().bucket(bucket).send().await?;

    println!("{:#?}", ret.contents);

    Ok(())
}
